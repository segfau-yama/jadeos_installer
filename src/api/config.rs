use crate::gui::state::InstallerConfig;
use std::{fs, path::Path};

pub fn save_config(path: &Path, config: &InstallerConfig) -> Result<(), std::io::Error> {
    let content = toml::to_string(config)
        .map_err(|error| std::io::Error::other(format!("failed to serialize config: {error}")))?;
    fs::write(path, content)
}

pub fn load_config(path: &Path) -> Result<InstallerConfig, std::io::Error> {
    let content = fs::read_to_string(path)?;
    toml::from_str(&content)
        .map_err(|error| std::io::Error::other(format!("failed to parse config: {error}")))
}
