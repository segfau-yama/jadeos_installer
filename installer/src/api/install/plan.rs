use crate::gui::state::InstallerConfig;

use super::types::{
    InstallCommand, InstallError, InstallPhase, InstallPlan, DEFAULT_REPOSITORY_URL,
    INSTALL_BOOT_SIZE, INSTALL_MOUNT_ROOT,
};
use super::validate::validate_config;

pub fn preview_install_plan(config: &InstallerConfig) -> Result<InstallPlan, InstallError> {
    build_install_plan(config, false)
}

pub fn generate_install_plan(config: &InstallerConfig) -> Result<InstallPlan, InstallError> {
    build_install_plan(config, true)
}

fn build_install_plan(
    config: &InstallerConfig,
    require_erase_confirmation: bool,
) -> Result<InstallPlan, InstallError> {
    validate_config(config, require_erase_confirmation)?;

    let efi_partition = format_partition_path(&config.target_disk, 1);
    let root_partition = format_partition_path(&config.target_disk, 2);
    let host_dir = format!(
        "{INSTALL_MOUNT_ROOT}/etc/nixos/nixos/{}",
        config.hostname.trim()
    );
    let repository_url = DEFAULT_REPOSITORY_URL.to_string();

    let commands = vec![
        InstallCommand {
            phase: InstallPhase::Validate,
            description: "Validate the installer inputs".to_string(),
            argv: vec!["validate-installer-inputs".to_string()],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::Partition,
            description: "Create a fresh GPT partition table".to_string(),
            argv: vec![
                "parted".to_string(),
                "-s".to_string(),
                config.target_disk.clone(),
                "mklabel".to_string(),
                "gpt".to_string(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Partition,
            description: "Create the EFI partition".to_string(),
            argv: vec![
                "parted".to_string(),
                "-s".to_string(),
                config.target_disk.clone(),
                "mkpart".to_string(),
                "ESP".to_string(),
                "fat32".to_string(),
                "1MiB".to_string(),
                INSTALL_BOOT_SIZE.to_string(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Partition,
            description: "Mark the EFI partition bootable".to_string(),
            argv: vec![
                "parted".to_string(),
                "-s".to_string(),
                config.target_disk.clone(),
                "set".to_string(),
                "1".to_string(),
                "esp".to_string(),
                "on".to_string(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Partition,
            description: "Create the root partition".to_string(),
            argv: vec![
                "parted".to_string(),
                "-s".to_string(),
                config.target_disk.clone(),
                "mkpart".to_string(),
                "nixos".to_string(),
                "ext4".to_string(),
                INSTALL_BOOT_SIZE.to_string(),
                "100%".to_string(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Format,
            description: "Format the EFI partition as FAT32".to_string(),
            argv: vec![
                "mkfs.fat".to_string(),
                "-F".to_string(),
                "32".to_string(),
                "-n".to_string(),
                "boot".to_string(),
                efi_partition.clone(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Format,
            description: "Format the root partition as ext4".to_string(),
            argv: vec![
                "mkfs.ext4".to_string(),
                "-L".to_string(),
                "nixos".to_string(),
                "-F".to_string(),
                root_partition.clone(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Mount,
            description: "Mount the root filesystem".to_string(),
            argv: vec![
                "mount".to_string(),
                "/dev/disk/by-label/nixos".to_string(),
                INSTALL_MOUNT_ROOT.to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::Mount,
            description: "Prepare the EFI mountpoint".to_string(),
            argv: vec![
                "mkdir".to_string(),
                "-p".to_string(),
                format!("{INSTALL_MOUNT_ROOT}/boot"),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::Mount,
            description: "Mount the EFI partition".to_string(),
            argv: vec![
                "mount".to_string(),
                "/dev/disk/by-label/boot".to_string(),
                format!("{INSTALL_MOUNT_ROOT}/boot"),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::GenerateConfig,
            description: "Clone the NixOS configuration repository".to_string(),
            argv: vec![
                "git".to_string(),
                "clone".to_string(),
                repository_url.clone(),
                format!("{INSTALL_MOUNT_ROOT}/etc/nixos"),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::GenerateConfig,
            description: "Prepare the generated host directory".to_string(),
            argv: vec!["mkdir".to_string(), "-p".to_string(), host_dir.clone()],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::GenerateConfig,
            description: "Capture hardware configuration".to_string(),
            argv: vec![
                "nixos-generate-config".to_string(),
                "--root".to_string(),
                INSTALL_MOUNT_ROOT.to_string(),
                "--show-hardware-config".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::GenerateConfig,
            description: "Track the generated host files in git".to_string(),
            argv: vec![
                "git".to_string(),
                "-C".to_string(),
                format!("{INSTALL_MOUNT_ROOT}/etc/nixos"),
                "add".to_string(),
                ".".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::InstallSystem,
            description: "Install the target system from the cloned flake".to_string(),
            argv: vec![
                "nixos-install".to_string(),
                "--flake".to_string(),
                format!(
                    "path:{INSTALL_MOUNT_ROOT}/etc/nixos#{}",
                    config.hostname.trim()
                ),
                "--no-root-passwd".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::SetPassword,
            description: "Set the installed user's password".to_string(),
            argv: vec![
                "nixos-enter".to_string(),
                "--root".to_string(),
                INSTALL_MOUNT_ROOT.to_string(),
                "-c".to_string(),
                "chpasswd".to_string(),
            ],
            destructive: false,
        },
    ];

    Ok(InstallPlan {
        repository_url,
        hostname: config.hostname.trim().to_string(),
        target_disk: config.target_disk.clone(),
        efi_partition,
        root_partition,
        commands,
    })
}

fn format_partition_path(target_disk: &str, partition_number: u8) -> String {
    let separator = if uses_partition_separator(target_disk) {
        "p"
    } else {
        ""
    };
    format!("{target_disk}{separator}{partition_number}")
}

fn uses_partition_separator(target_disk: &str) -> bool {
    let name = target_disk.strip_prefix("/dev/").unwrap_or(target_disk);
    name.starts_with("nvme") || name.starts_with("mmcblk")
}
