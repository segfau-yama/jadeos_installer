use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

use crate::gui::state::InstallerConfig;

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Serialize(toml::ser::Error),
    Deserialize(toml::de::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "failed to read or write installer config: {error}"),
            Self::Serialize(error) => write!(f, "failed to serialize installer config: {error}"),
            Self::Deserialize(error) => {
                write!(f, "failed to deserialize installer config: {error}")
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Serialize(error) => Some(error),
            Self::Deserialize(error) => Some(error),
        }
    }
}

pub fn save_installer_config(
    path: impl AsRef<Path>,
    config: &InstallerConfig,
) -> Result<(), ConfigError> {
    let toml = toml::to_string_pretty(config).map_err(ConfigError::Serialize)?;
    fs::write(path, toml).map_err(ConfigError::Io)
}

pub fn load_installer_config(path: impl AsRef<Path>) -> Result<InstallerConfig, ConfigError> {
    let toml = fs::read_to_string(path).map_err(ConfigError::Io)?;
    toml::from_str(&toml).map_err(ConfigError::Deserialize)
}
