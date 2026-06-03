use crate::gui::state::InstallerConfig;

pub fn render(config: &InstallerConfig) -> String {
    format!(
        "Summary: hostname={}, username={}, disk={}",
        config.hostname, config.username, config.target_disk
    )
}
