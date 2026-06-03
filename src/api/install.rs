use crate::{api::disk::is_valid_target_disk, gui::state::InstallerConfig};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallCommand {
    pub command: String,
    pub args: Vec<String>,
    pub destructive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    pub commands: Vec<InstallCommand>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InstallValidationError {
    #[error("hostname is required")]
    EmptyHostname,
    #[error("username is required")]
    EmptyUsername,
    #[error("target disk is required")]
    EmptyTargetDisk,
    #[error("target disk is invalid")]
    InvalidTargetDisk,
    #[error("disk erase must be confirmed")]
    DiskEraseNotConfirmed,
}

pub fn validate(config: &InstallerConfig) -> Result<(), InstallValidationError> {
    if config.hostname.trim().is_empty() {
        return Err(InstallValidationError::EmptyHostname);
    }
    if config.username.trim().is_empty() {
        return Err(InstallValidationError::EmptyUsername);
    }
    if config.target_disk.trim().is_empty() {
        return Err(InstallValidationError::EmptyTargetDisk);
    }
    if !is_valid_target_disk(&config.target_disk) {
        return Err(InstallValidationError::InvalidTargetDisk);
    }
    if !config.disk_erase_confirmed {
        return Err(InstallValidationError::DiskEraseNotConfirmed);
    }

    Ok(())
}

pub fn build_install_plan(config: &InstallerConfig) -> Result<InstallPlan, InstallValidationError> {
    validate(config)?;

    let efi_partition = if config.target_disk.contains("nvme") {
        format!("{}p1", config.target_disk)
    } else {
        format!("{}1", config.target_disk)
    };
    let root_partition = if config.target_disk.contains("nvme") {
        format!("{}p2", config.target_disk)
    } else {
        format!("{}2", config.target_disk)
    };

    Ok(InstallPlan {
        commands: vec![
            InstallCommand {
                command: "sgdisk".to_string(),
                args: vec!["--zap-all".to_string(), config.target_disk.clone()],
                destructive: true,
            },
            InstallCommand {
                command: "sgdisk".to_string(),
                args: vec![
                    "-n".to_string(),
                    "1:1MiB:+512MiB".to_string(),
                    "-t".to_string(),
                    "1:EF00".to_string(),
                    config.target_disk.clone(),
                ],
                destructive: true,
            },
            InstallCommand {
                command: "sgdisk".to_string(),
                args: vec![
                    "-n".to_string(),
                    "2:0:0".to_string(),
                    "-t".to_string(),
                    "2:8300".to_string(),
                    config.target_disk.clone(),
                ],
                destructive: true,
            },
            InstallCommand {
                command: "mkfs.fat".to_string(),
                args: vec!["-F32".to_string(), efi_partition.clone()],
                destructive: true,
            },
            InstallCommand {
                command: "mkfs.ext4".to_string(),
                args: vec!["-F".to_string(), root_partition.clone()],
                destructive: true,
            },
            InstallCommand {
                command: "mount".to_string(),
                args: vec![root_partition, "/mnt".to_string()],
                destructive: false,
            },
            InstallCommand {
                command: "mkdir".to_string(),
                args: vec!["-p".to_string(), "/mnt/boot".to_string()],
                destructive: false,
            },
            InstallCommand {
                command: "mount".to_string(),
                args: vec![efi_partition, "/mnt/boot".to_string()],
                destructive: false,
            },
            InstallCommand {
                command: "mkdir".to_string(),
                args: vec!["-p".to_string(), "/mnt/etc/nixos".to_string()],
                destructive: false,
            },
            InstallCommand {
                command: "nixos-generate-config".to_string(),
                args: vec!["--root".to_string(), "/mnt".to_string()],
                destructive: false,
            },
            InstallCommand {
                command: "nixos-install".to_string(),
                args: vec!["--flake".to_string(), "/mnt/etc/nixos#jadeos".to_string()],
                destructive: false,
            },
            InstallCommand {
                command: "nixos-enter".to_string(),
                args: vec![
                    "--root".to_string(),
                    "/mnt".to_string(),
                    "-c".to_string(),
                    format!("passwd {}", config.username),
                ],
                destructive: false,
            },
        ],
    })
}
