use crate::gui::state::InstallerConfig;

use super::layout::InstallLayout;
use super::types::{InstallError, InstallPlan};
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

    let layout = InstallLayout::from_config(config);

    Ok(InstallPlan {
        repository_url: layout.repository_url,
        hostname: layout.hostname,
        target_disk: layout.target_disk,
        efi_partition: layout.efi_partition,
        root_partition: layout.root_partition,
    })
}
