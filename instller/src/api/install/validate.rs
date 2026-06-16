use crate::api::disk::is_disk_device_path;
use crate::gui::state::InstallerConfig;

use super::types::InstallError;

pub fn validate_config(
    config: &InstallerConfig,
    require_erase_confirmation: bool,
) -> Result<(), InstallError> {
    if config.hostname.trim().is_empty() {
        return Err(InstallError::EmptyHostname);
    }

    if config.username.trim().is_empty() {
        return Err(InstallError::EmptyUsername);
    }

    if config.target_disk.trim().is_empty() {
        return Err(InstallError::EmptyTargetDisk);
    }

    if !is_disk_device_path(&config.target_disk) {
        return Err(InstallError::InvalidTargetDisk(config.target_disk.clone()));
    }

    if require_erase_confirmation && !config.disk_erase_confirmed {
        return Err(InstallError::DiskEraseNotConfirmed);
    }

    Ok(())
}
