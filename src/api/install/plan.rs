use crate::gui::state::InstallerConfig;

use super::types::{InstallCommand, InstallError, InstallPhase, InstallPlan};
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

    let commands = vec![
        InstallCommand {
            phase: InstallPhase::Validate,
            description: "Validate the installer inputs".to_string(),
            argv: vec!["validate-installer-inputs".to_string()],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::Partition,
            description: "Wipe the target disk".to_string(),
            argv: vec![
                "sgdisk".to_string(),
                "--zap-all".to_string(),
                config.target_disk.clone(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Partition,
            description: "Create the EFI partition".to_string(),
            argv: vec![
                "sgdisk".to_string(),
                "-n".to_string(),
                "1:1MiB:+512MiB".to_string(),
                "-t".to_string(),
                "1:EF00".to_string(),
                config.target_disk.clone(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Partition,
            description: "Create the root partition".to_string(),
            argv: vec![
                "sgdisk".to_string(),
                "-n".to_string(),
                "2:0:0".to_string(),
                "-t".to_string(),
                "2:8300".to_string(),
                config.target_disk.clone(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Format,
            description: "Format the EFI partition as FAT32".to_string(),
            argv: vec![
                "mkfs.fat".to_string(),
                "-F32".to_string(),
                efi_partition.clone(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Format,
            description: "Format the root partition as ext4".to_string(),
            argv: vec![
                "mkfs.ext4".to_string(),
                "-F".to_string(),
                root_partition.clone(),
            ],
            destructive: true,
        },
        InstallCommand {
            phase: InstallPhase::Mount,
            description: "Mount the root partition".to_string(),
            argv: vec![
                "mount".to_string(),
                root_partition.clone(),
                "/mnt".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::Mount,
            description: "Prepare the EFI mountpoint".to_string(),
            argv: vec![
                "mkdir".to_string(),
                "-p".to_string(),
                "/mnt/boot".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::Mount,
            description: "Mount the EFI partition".to_string(),
            argv: vec![
                "mount".to_string(),
                efi_partition.clone(),
                "/mnt/boot".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::GenerateConfig,
            description: "Prepare the NixOS configuration directory".to_string(),
            argv: vec![
                "mkdir".to_string(),
                "-p".to_string(),
                "/mnt/etc/nixos".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::GenerateConfig,
            description: "Generate the JadeOS flake and configuration files".to_string(),
            argv: vec![
                "jadeos-template-install".to_string(),
                "/mnt/etc/nixos".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::GenerateConfig,
            description: "Generate the hardware configuration".to_string(),
            argv: vec![
                "nixos-generate-config".to_string(),
                "--root".to_string(),
                "/mnt".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::InstallSystem,
            description: "Install the target system".to_string(),
            argv: vec![
                "nixos-install".to_string(),
                "--flake".to_string(),
                "/mnt/etc/nixos#jadeos".to_string(),
            ],
            destructive: false,
        },
        InstallCommand {
            phase: InstallPhase::SetPassword,
            description: "Set the created user's password".to_string(),
            argv: vec![
                "nixos-enter".to_string(),
                "--root".to_string(),
                "/mnt".to_string(),
                "-c".to_string(),
                format!("passwd {}", config.username),
            ],
            destructive: false,
        },
    ];

    Ok(InstallPlan {
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
