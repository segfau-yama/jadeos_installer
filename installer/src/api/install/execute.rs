use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::gui::state::InstallerConfig;

use super::nix_templates::{
    render_hardware_configuration, render_host_configuration, render_host_flake_parts,
    render_install_args, HostTemplateContext, InstallArgsContext,
};
use super::types::{InstallPhase, InstallPlan, InstallationReport, INSTALL_MOUNT_ROOT};

const SUDO_NON_INTERACTIVE_FLAG: &str = "--non-interactive";

pub fn run_install_plan(
    config: &InstallerConfig,
    password: &str,
    plan: &InstallPlan,
) -> InstallationReport {
    run_install_plan_with_progress(config, password, plan, |_| {})
}

pub fn run_install_plan_with_progress<F>(
    config: &InstallerConfig,
    password: &str,
    plan: &InstallPlan,
    mut on_progress: F,
) -> InstallationReport
where
    F: FnMut(&InstallationReport),
{
    let mut report = InstallationReport {
        final_phase: InstallPhase::Validate,
        current_command: None,
        log: Vec::new(),
        error_message: None,
    };
    publish_report(&report, &mut on_progress);

    if let Err(error) =
        run_install_plan_inner(config, password, plan, &mut report, &mut on_progress)
    {
        report.error_message = Some(error.clone());
        report.log.push(error);
        publish_report(&report, &mut on_progress);
    }

    report
}

fn run_install_plan_inner(
    config: &InstallerConfig,
    password: &str,
    plan: &InstallPlan,
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
) -> Result<(), String> {
    report.log.push(format!(
        "Starting install for {} on {}",
        plan.hostname, plan.target_disk
    ));
    publish_report(report, on_progress);

    report.final_phase = InstallPhase::Validate;
    report
        .log
        .push("Validated installer inputs. Beginning install execution.".to_string());
    publish_report(report, on_progress);
    ensure_root_access(report, on_progress)?;

    if is_mountpoint_busy(plan)? {
        run_command(
            report,
            on_progress,
            InstallPhase::Validate,
            "Unmount stale installer mounts",
            "umount",
            &["-R", INSTALL_MOUNT_ROOT],
        )?;
    }

    run_command(
        report,
        on_progress,
        InstallPhase::Partition,
        "Create a fresh GPT partition table",
        "parted",
        &["-s", &plan.target_disk, "mklabel", "gpt"],
    )?;
    run_command(
        report,
        on_progress,
        InstallPhase::Partition,
        "Create the EFI partition",
        "parted",
        &[
            "-s",
            &plan.target_disk,
            "mkpart",
            "ESP",
            "fat32",
            "1MiB",
            "512MiB",
        ],
    )?;
    run_command(
        report,
        on_progress,
        InstallPhase::Partition,
        "Mark the EFI partition bootable",
        "parted",
        &["-s", &plan.target_disk, "set", "1", "esp", "on"],
    )?;
    run_command(
        report,
        on_progress,
        InstallPhase::Partition,
        "Create the root partition",
        "parted",
        &[
            "-s",
            &plan.target_disk,
            "mkpart",
            "nixos",
            "ext4",
            "512MiB",
            "100%",
        ],
    )?;

    run_command(
        report,
        on_progress,
        InstallPhase::Format,
        "Format the EFI partition as FAT32",
        "mkfs.fat",
        &["-F", "32", "-n", "boot", &plan.efi_partition],
    )?;
    run_command(
        report,
        on_progress,
        InstallPhase::Format,
        "Format the root partition as ext4",
        "mkfs.ext4",
        &["-L", "nixos", "-F", &plan.root_partition],
    )?;

    run_command(
        report,
        on_progress,
        InstallPhase::Mount,
        "Mount the root filesystem",
        "mount",
        &[&plan.root_partition, INSTALL_MOUNT_ROOT],
    )?;
    run_command(
        report,
        on_progress,
        InstallPhase::Mount,
        "Prepare the EFI mountpoint",
        "mkdir",
        &["-p", "/mnt/boot"],
    )?;
    run_command(
        report,
        on_progress,
        InstallPhase::Mount,
        "Mount the EFI partition",
        "mount",
        &[&plan.efi_partition, "/mnt/boot"],
    )?;

    let repo_root = Path::new("/mnt/etc/nixos");
    let host_dir = repo_root.join("nixos").join(config.hostname.trim());
    let detected_cpu = detect_cpu_kind();
    let detected_gpu = detect_gpu_kind();
    report.log.push(format!(
        "Detected hardware defaults: cpu={}, gpu={}",
        detected_cpu, detected_gpu
    ));
    publish_report(report, on_progress);

    run_command(
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Prepare /mnt/etc",
        "mkdir",
        &["-p", "/mnt/etc"],
    )?;
    remove_path_if_exists(report, on_progress, InstallPhase::GenerateConfig, repo_root)?;
    run_command(
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Clone the NixOS configuration repository",
        "git",
        &["clone", &plan.repository_url, "/mnt/etc/nixos"],
    )?;

    let template_context = HostTemplateContext {
        hostname: config.hostname.trim(),
        username: config.username.trim(),
        detected_cpu,
        detected_gpu,
    };
    write_file(
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        &host_dir.join("flake-parts.nix"),
        &render_host_flake_parts(&template_context)?,
    )?;
    write_file(
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        &host_dir.join("configuration.nix"),
        &render_host_configuration(&template_context)?,
    )?;
    let install_args_context = InstallArgsContext {
        efi_partition: &plan.efi_partition,
        root_partition: &plan.root_partition,
    };
    write_file(
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        &host_dir.join("install-args.nix"),
        &render_install_args(&install_args_context)?,
    )?;

    let generated_hardware_output = capture_command(
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Capture hardware configuration",
        "nixos-generate-config",
        &["--root", INSTALL_MOUNT_ROOT, "--show-hardware-config"],
    )?;
    write_file(
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        &host_dir.join("hardware-configuration.nix"),
        &render_hardware_configuration(&generated_hardware_output)?,
    )?;
    run_command(
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Track generated host files in git",
        "git",
        &["-C", "/mnt/etc/nixos", "add", "."],
    )?;

    run_command(
        report,
        on_progress,
        InstallPhase::InstallSystem,
        "Install the target system",
        "nixos-install",
        &[
            "--flake",
            &format!("path:/mnt/etc/nixos#{}", config.hostname.trim()),
            "--no-root-passwd",
        ],
    )?;

    run_command_with_input(
        report,
        on_progress,
        InstallPhase::SetPassword,
        "Set the installed user's password",
        "nixos-enter",
        &["--root", INSTALL_MOUNT_ROOT, "-c", "chpasswd"],
        &format!("{}:{password}\n", config.username.trim()),
    )?;

    report.final_phase = InstallPhase::Finish;
    report.current_command = None;
    report
        .log
        .push("Install finished successfully.".to_string());
    publish_report(report, on_progress);
    Ok(())
}

fn ensure_root_access(
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
) -> Result<(), String> {
    let rendered = render_command("sudo", &[SUDO_NON_INTERACTIVE_FLAG, "true"]);
    report.current_command = Some(rendered.clone());
    report.log.push("[Validate] Verify root access".to_string());
    report.log.push(format!("$ {rendered}"));
    publish_report(report, on_progress);

    let output = Command::new("sudo")
        .args([SUDO_NON_INTERACTIVE_FLAG, "true"])
        .output()
        .map_err(|error| format!("failed to start `{rendered}`: {error}"))?;

    push_process_output(report, on_progress, &output.stdout, &output.stderr);

    if output.status.success() {
        Ok(())
    } else {
        Err(
            "installer cannot obtain root privileges via sudo; check live image sudo policy"
                .to_string(),
        )
    }
}

fn run_command(
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
    phase: InstallPhase,
    description: &str,
    program: &str,
    args: &[&str],
) -> Result<(), String> {
    let rendered = render_sudo_command(program, args);
    report.final_phase = phase;
    report.current_command = Some(rendered.clone());
    report
        .log
        .push(format!("[{}] {}", phase.label(), description));
    report.log.push(format!("$ {rendered}"));
    publish_report(report, on_progress);

    let output = Command::new("sudo")
        .arg(SUDO_NON_INTERACTIVE_FLAG)
        .arg(program)
        .args(args)
        .output()
        .map_err(|error| format!("{description}: failed to start `{rendered}`: {error}"))?;

    push_process_output(report, on_progress, &output.stdout, &output.stderr);

    if output.status.success() {
        Ok(())
    } else {
        Err(format_command_failure(
            description,
            &rendered,
            &output.stderr,
        ))
    }
}

fn capture_command(
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
    phase: InstallPhase,
    description: &str,
    program: &str,
    args: &[&str],
) -> Result<String, String> {
    let rendered = render_sudo_command(program, args);
    report.final_phase = phase;
    report.current_command = Some(rendered.clone());
    report
        .log
        .push(format!("[{}] {}", phase.label(), description));
    report.log.push(format!("$ {rendered}"));
    publish_report(report, on_progress);

    let output = Command::new("sudo")
        .arg(SUDO_NON_INTERACTIVE_FLAG)
        .arg(program)
        .args(args)
        .output()
        .map_err(|error| format!("{description}: failed to start `{rendered}`: {error}"))?;

    push_process_output(report, on_progress, &output.stdout, &output.stderr);

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format_command_failure(
            description,
            &rendered,
            &output.stderr,
        ))
    }
}

fn run_command_with_input(
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
    phase: InstallPhase,
    description: &str,
    program: &str,
    args: &[&str],
    input: &str,
) -> Result<(), String> {
    let rendered = render_sudo_command(program, args);
    report.final_phase = phase;
    report.current_command = Some(rendered.clone());
    report
        .log
        .push(format!("[{}] {}", phase.label(), description));
    report.log.push(format!("$ {rendered}"));
    publish_report(report, on_progress);

    let mut child = Command::new("sudo")
        .arg(SUDO_NON_INTERACTIVE_FLAG)
        .arg(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("{description}: failed to start `{rendered}`: {error}"))?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(input.as_bytes())
            .map_err(|error| format!("{description}: failed to send input: {error}"))?;
    }

    let output = child.wait_with_output().map_err(|error| {
        format!("{description}: failed while waiting for `{rendered}`: {error}")
    })?;

    push_process_output(report, on_progress, &output.stdout, &output.stderr);

    if output.status.success() {
        Ok(())
    } else {
        Err(format_command_failure(
            description,
            &rendered,
            &output.stderr,
        ))
    }
}

fn push_process_output(
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
    stdout: &[u8],
    stderr: &[u8],
) {
    for line in String::from_utf8_lossy(stdout).lines() {
        if !line.trim().is_empty() {
            report.log.push(line.to_string());
        }
    }
    for line in String::from_utf8_lossy(stderr).lines() {
        if !line.trim().is_empty() {
            report.log.push(line.to_string());
        }
    }
    publish_report(report, on_progress);
}

fn format_command_failure(description: &str, rendered: &str, stderr: &[u8]) -> String {
    let stderr = String::from_utf8_lossy(stderr).trim().to_string();
    if stderr.is_empty() {
        format!("{description}: `{rendered}` failed")
    } else {
        format!("{description}: `{rendered}` failed: {stderr}")
    }
}

fn render_command(program: &str, args: &[&str]) -> String {
    std::iter::once(program)
        .chain(args.iter().copied())
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_sudo_command(program: &str, args: &[&str]) -> String {
    let mut sudo_args = Vec::with_capacity(args.len() + 2);
    sudo_args.push(SUDO_NON_INTERACTIVE_FLAG);
    sudo_args.push(program);
    sudo_args.extend_from_slice(args);
    render_command("sudo", &sudo_args)
}

fn write_file(
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
    phase: InstallPhase,
    path: &Path,
    contents: &str,
) -> Result<(), String> {
    report.final_phase = phase;
    report.current_command = Some(format!("write {}", path.display()));
    report
        .log
        .push(format!("[{}] Write {}", phase.label(), path.display()));
    publish_report(report, on_progress);

    if let Some(parent) = path.parent() {
        run_command(
            report,
            on_progress,
            phase,
            &format!("Prepare {}", parent.display()),
            "mkdir",
            &["-p", &parent.display().to_string()],
        )?;
    }

    let rendered = render_sudo_command("tee", &[&path.display().to_string()]);
    report.current_command = Some(rendered.clone());
    report.log.push(format!("$ {rendered}"));
    publish_report(report, on_progress);

    let mut child = Command::new("sudo")
        .arg(SUDO_NON_INTERACTIVE_FLAG)
        .arg("tee")
        .arg(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("failed to start `{rendered}`: {error}"))?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(contents.as_bytes())
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|error| format!("failed while waiting for `{rendered}`: {error}"))?;

    push_process_output(report, on_progress, &[], &output.stderr);

    if !output.status.success() {
        return Err(format_command_failure(
            &format!("failed to write {}", path.display()),
            &rendered,
            &output.stderr,
        ));
    }

    Ok(())
}

fn remove_path_if_exists(
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
    phase: InstallPhase,
    path: &Path,
) -> Result<(), String> {
    run_command(
        report,
        on_progress,
        phase,
        &format!("Reset {}", path.display()),
        "rm",
        &["-rf", &path.display().to_string()],
    )
}

fn publish_report(report: &InstallationReport, on_progress: &mut dyn FnMut(&InstallationReport)) {
    on_progress(report);
}

fn is_mountpoint_busy(plan: &InstallPlan) -> Result<bool, String> {
    let output = Command::new("lsblk")
        .args(["-nr", "-o", "NAME,MOUNTPOINT", &plan.target_disk])
        .output()
        .map_err(|error| format!("failed to inspect existing mounts: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("failed to inspect existing mounts: {stderr}"));
    }

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.splitn(2, char::is_whitespace);
        let _device = parts.next();
        let mountpoint = parts.next().unwrap_or_default().trim();
        if mountpoint == INSTALL_MOUNT_ROOT || mountpoint.starts_with("/mnt/") {
            return Ok(true);
        }
        if !mountpoint.is_empty() {
            return Err(format!(
                "target disk {} has mounted partitions outside /mnt: {}",
                plan.target_disk, mountpoint
            ));
        }
    }

    Ok(false)
}

fn detect_cpu_kind() -> &'static str {
    if std::env::consts::ARCH == "aarch64" {
        return "aarch64";
    }

    match fs::read_to_string("/proc/cpuinfo") {
        Ok(cpuinfo) if cpuinfo.contains("GenuineIntel") => "intel",
        Ok(cpuinfo) if cpuinfo.contains("AuthenticAMD") => "amd",
        _ => "amd",
    }
}

fn detect_gpu_kind() -> &'static str {
    let Ok(entries) = fs::read_dir("/sys/class/drm") else {
        return "none";
    };

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if !file_name.starts_with("card") || file_name.contains('-') {
            continue;
        }

        let vendor_path = entry.path().join("device").join("vendor");
        let Ok(vendor) = fs::read_to_string(vendor_path) else {
            continue;
        };

        return match vendor.trim() {
            "0x10de" => "nvidia",
            "0x1002" | "0x1022" => "amd",
            "0x8086" => "intel",
            "0x1af4" => "virtio",
            _ => "none",
        };
    }

    "none"
}
