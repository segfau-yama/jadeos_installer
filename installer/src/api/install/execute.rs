use std::fs;
use std::path::Path;

use crate::api::command::{
    CommandStrategy, GIT_COMMAND, LSBLK_COMMAND, MKDIR_COMMAND, MKFS_EXT4_COMMAND,
    MKFS_FAT_COMMAND, MOUNT_COMMAND, NIXOS_ENTER_COMMAND, NIXOS_GENERATE_CONFIG_COMMAND,
    NIXOS_INSTALL_COMMAND, PARTED_COMMAND, RM_COMMAND, TEE_COMMAND, TRUE_COMMAND, UMOUNT_COMMAND,
};
use crate::api::execute::{CommandError, CommandExecutor};
use crate::gui::state::InstallerConfig;

use super::layout::InstallLayout;
use super::nix_templates::{
    render_gui_user_configuration, render_hardware_configuration, render_host_configuration,
    render_host_flake_parts, render_install_args, HostTemplateContext, InstallArgsContext,
};
use super::types::{
    InstallPhase, InstallPlan, InstallationReport, INSTALL_BOOT_SIZE, INSTALL_MOUNT_ROOT,
};

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
    let layout = InstallLayout::from_config(config);
    let detected_hardware = DetectedHardware::detect();
    let mut executor = CommandExecutor::new();

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

    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Validate,
        "Verify root access",
        &TRUE_COMMAND,
        &[],
        None,
        false,
    )?;

    if is_mountpoint_busy(&mut executor, plan)? {
        execute_step(
            &mut executor,
            report,
            on_progress,
            InstallPhase::Validate,
            "Unmount stale installer mounts",
            &UMOUNT_COMMAND,
            &["-R", INSTALL_MOUNT_ROOT],
            None,
            false,
        )?;
    }

    report.log.push(format!(
        "Detected hardware defaults: arch={}, cpu={}, gpu={}",
        detected_hardware.system_arch, detected_hardware.cpu, detected_hardware.gpu
    ));
    publish_report(report, on_progress);

    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Partition,
        "Create a fresh GPT partition table",
        &PARTED_COMMAND,
        &["-s", layout.target_disk.as_str(), "mklabel", "gpt"],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Partition,
        "Create the EFI partition",
        &PARTED_COMMAND,
        &[
            "-s",
            layout.target_disk.as_str(),
            "mkpart",
            "ESP",
            "fat32",
            "1MiB",
            INSTALL_BOOT_SIZE,
        ],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Partition,
        "Mark the EFI partition bootable",
        &PARTED_COMMAND,
        &["-s", layout.target_disk.as_str(), "set", "1", "esp", "on"],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Partition,
        "Create the root partition",
        &PARTED_COMMAND,
        &[
            "-s",
            layout.target_disk.as_str(),
            "mkpart",
            "nixos",
            "ext4",
            INSTALL_BOOT_SIZE,
            "100%",
        ],
        None,
        false,
    )?;

    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Format,
        "Format the EFI partition as FAT32",
        &MKFS_FAT_COMMAND,
        &["-F", "32", "-n", "boot", layout.efi_partition.as_str()],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Format,
        "Format the root partition as ext4",
        &MKFS_EXT4_COMMAND,
        &["-L", "nixos", "-F", layout.root_partition.as_str()],
        None,
        false,
    )?;

    let boot_mount_dir = layout.boot_mount_dir();
    let etc_dir = format!("{INSTALL_MOUNT_ROOT}/etc");
    let repo_root = layout.repo_root.display().to_string();
    let host_dir = layout.host_dir.display().to_string();
    let user_dir = layout.user_dir.display().to_string();
    let hardware_dir = layout.hardware_dir.display().to_string();

    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Mount,
        "Mount the root filesystem",
        &MOUNT_COMMAND,
        &[layout.root_partition.as_str(), INSTALL_MOUNT_ROOT],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Mount,
        "Prepare the EFI mountpoint",
        &MKDIR_COMMAND,
        &["-p", boot_mount_dir.as_str()],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::Mount,
        "Mount the EFI partition",
        &MOUNT_COMMAND,
        &[layout.efi_partition.as_str(), boot_mount_dir.as_str()],
        None,
        false,
    )?;

    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Prepare /mnt/etc",
        &MKDIR_COMMAND,
        &["-p", etc_dir.as_str()],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Reset /mnt/etc/nixos",
        &RM_COMMAND,
        &["-rf", repo_root.as_str()],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Clone the NixOS configuration repository",
        &GIT_COMMAND,
        &["clone", layout.repository_url.as_str(), repo_root.as_str()],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Prepare the generated host module directory",
        &MKDIR_COMMAND,
        &["-p", host_dir.as_str()],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Prepare the generated user module directory",
        &MKDIR_COMMAND,
        &["-p", user_dir.as_str()],
        None,
        false,
    )?;
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Prepare the generated hardware directory",
        &MKDIR_COMMAND,
        &["-p", hardware_dir.as_str()],
        None,
        false,
    )?;

    write_generated_configuration_files(
        &mut executor,
        report,
        on_progress,
        &layout,
        &detected_hardware,
    )?;

    let generated_hardware_output = execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Capture hardware configuration",
        &NIXOS_GENERATE_CONFIG_COMMAND,
        &["--root", INSTALL_MOUNT_ROOT, "--show-hardware-config"],
        None,
        true,
    )?;
    write_file(
        &mut executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        &layout.hardware_dir.join("hardware-configuration.nix"),
        &render_hardware_configuration(&generated_hardware_output)?,
    )?;

    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        "Track the generated host files in git",
        &GIT_COMMAND,
        &["-C", repo_root.as_str(), "add", "."],
        None,
        false,
    )?;

    let target_flake_ref = layout.target_flake_ref();
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::InstallSystem,
        "Install the target system from the cloned flake",
        &NIXOS_INSTALL_COMMAND,
        &["--flake", target_flake_ref.as_str(), "--no-root-passwd"],
        None,
        true,
    )?;

    let password_input = format!("{}:{password}\n", layout.username);
    execute_step(
        &mut executor,
        report,
        on_progress,
        InstallPhase::SetPassword,
        "Set the installed user's password",
        &NIXOS_ENTER_COMMAND,
        &["--root", INSTALL_MOUNT_ROOT, "-c", "chpasswd"],
        Some(password_input.as_str()),
        false,
    )?;

    report.final_phase = InstallPhase::Finish;
    report.current_command = None;
    report
        .log
        .push("Install finished successfully.".to_string());
    publish_report(report, on_progress);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DetectedHardware {
    system_arch: &'static str,
    cpu: &'static str,
    gpu: &'static str,
}

impl DetectedHardware {
    fn detect() -> Self {
        Self {
            system_arch: detect_system_arch(),
            cpu: detect_cpu_kind(),
            gpu: detect_gpu_kind(),
        }
    }
}

fn execute_step(
    executor: &mut CommandExecutor,
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
    phase: InstallPhase,
    description: &str,
    strategy: &'static dyn CommandStrategy,
    args: &[&str],
    input: Option<&str>,
    log_stdout: bool,
) -> Result<String, String> {
    executor.set_strategy(strategy);
    let rendered = executor
        .render_command(args, true)
        .map_err(|error| format_step_error(description, error))?;

    report.final_phase = phase;
    report.current_command = Some(rendered.clone());
    report
        .log
        .push(format!("[{}] {}", phase.label(), description));
    report.log.push(format!("$ {rendered}"));
    publish_report(report, on_progress);

    let stdout = executor
        .super_user_execute(args, input)
        .map_err(|error| format_step_error(description, error))?;

    if log_stdout {
        push_stdout(report, on_progress, &stdout);
    } else {
        publish_report(report, on_progress);
    }

    Ok(stdout)
}

fn write_generated_configuration_files(
    executor: &mut CommandExecutor,
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
    layout: &InstallLayout,
    detected_hardware: &DetectedHardware,
) -> Result<(), String> {
    let template_context = HostTemplateContext {
        system_arch: detected_hardware.system_arch,
        hostname: &layout.hostname,
        username: &layout.username,
        detected_cpu: detected_hardware.cpu,
        detected_gpu: detected_hardware.gpu,
    };
    let install_args_context = InstallArgsContext {
        efi_partition: &layout.efi_partition,
        root_partition: &layout.root_partition,
    };

    write_file(
        executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        &layout.host_dir.join("flake-parts.nix"),
        &render_host_flake_parts(&template_context)?,
    )?;
    write_file(
        executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        &layout.host_dir.join("configuration.nix"),
        &render_host_configuration(&template_context)?,
    )?;
    write_file(
        executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        &layout.user_file,
        &render_gui_user_configuration(&template_context)?,
    )?;
    write_file(
        executor,
        report,
        on_progress,
        InstallPhase::GenerateConfig,
        &layout.hardware_dir.join("install-args.nix"),
        &render_install_args(&install_args_context)?,
    )?;

    Ok(())
}

fn write_file(
    executor: &mut CommandExecutor,
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
        let parent_path = parent.display().to_string();
        let description = format!("Prepare {}", parent.display());
        execute_step(
            executor,
            report,
            on_progress,
            phase,
            description.as_str(),
            &MKDIR_COMMAND,
            &["-p", parent_path.as_str()],
            None,
            false,
        )?;
    }

    let path_string = path.display().to_string();
    let description = format!("Write {}", path.display());
    execute_step(
        executor,
        report,
        on_progress,
        phase,
        description.as_str(),
        &TEE_COMMAND,
        &[path_string.as_str()],
        Some(contents),
        false,
    )?;

    Ok(())
}

fn publish_report(report: &InstallationReport, on_progress: &mut dyn FnMut(&InstallationReport)) {
    on_progress(report);
}

fn push_stdout(
    report: &mut InstallationReport,
    on_progress: &mut dyn FnMut(&InstallationReport),
    stdout: &str,
) {
    for line in stdout.lines() {
        if !line.trim().is_empty() {
            report.log.push(line.to_string());
        }
    }

    publish_report(report, on_progress);
}

fn format_step_error(description: &str, error: CommandError) -> String {
    format!("{description}: {error}")
}

fn is_mountpoint_busy(executor: &mut CommandExecutor, plan: &InstallPlan) -> Result<bool, String> {
    executor.set_strategy(&LSBLK_COMMAND);
    let stdout = executor
        .execute(&["-nr", "-o", "NAME,MOUNTPOINT", plan.target_disk.as_str()])
        .map_err(|error| format!("failed to inspect existing mounts: {error}"))?;

    for line in stdout.lines() {
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

fn detect_system_arch() -> &'static str {
    match std::env::consts::ARCH {
        "aarch64" => "aarch64-linux",
        _ => "x86_64-linux",
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
