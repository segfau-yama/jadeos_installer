use std::path::{Path, PathBuf};

use crate::gui::state::InstallerConfig;

use super::types::{DEFAULT_REPOSITORY_URL, INSTALL_MOUNT_ROOT};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallLayout {
    pub hostname: String,
    pub username: String,
    pub repository_url: String,
    pub target_disk: String,
    pub efi_partition: String,
    pub root_partition: String,
    pub repo_root: PathBuf,
    pub host_dir: PathBuf,
    pub user_dir: PathBuf,
    pub user_file: PathBuf,
    pub hardware_dir: PathBuf,
}

impl InstallLayout {
    pub fn from_config(config: &InstallerConfig) -> Self {
        let hostname = config.hostname.trim().to_string();
        let username = config.username.trim().to_string();
        let target_disk = config.target_disk.clone();
        let efi_partition = format_partition_path(&target_disk, 1);
        let root_partition = format_partition_path(&target_disk, 2);
        let repo_root = Path::new(INSTALL_MOUNT_ROOT).join("etc").join("nixos");
        let host_dir = repo_root.join("modules").join("hosts").join(&hostname);
        let user_dir = repo_root.join("modules").join("users").join(&username);
        let user_file = user_dir.join(format!("{username}.nix"));
        let hardware_dir = repo_root.join("nixos").join(&hostname);

        Self {
            hostname,
            username,
            repository_url: DEFAULT_REPOSITORY_URL.to_string(),
            target_disk,
            efi_partition,
            root_partition,
            repo_root,
            host_dir,
            user_dir,
            user_file,
            hardware_dir,
        }
    }

    pub fn boot_mount_dir(&self) -> String {
        format!("{INSTALL_MOUNT_ROOT}/boot")
    }

    pub fn target_flake_ref(&self) -> String {
        format!("path:{}#{}", self.repo_root.display(), self.hostname)
    }
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
