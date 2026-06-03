use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallerStep {
    Welcome,
    User,
    Disk,
    Summary,
    Install,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallPhase {
    Validate,
    Partition,
    Format,
    Mount,
    GenerateConfig,
    InstallSystem,
    SetPassword,
    Finish,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstallerConfig {
    pub schema_version: u32,
    pub hostname: String,
    pub username: String,
    pub target_disk: String,
    pub disk_erase_confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallerState {
    pub step: InstallerStep,
    pub phase: InstallPhase,
    pub config: InstallerConfig,
    pub password: Option<String>,
    pub password_confirmation: Option<String>,
    pub logs: Vec<String>,
}

impl Default for InstallerConfig {
    fn default() -> Self {
        Self {
            schema_version: 1,
            hostname: "jadeos".to_string(),
            username: "jade".to_string(),
            target_disk: String::new(),
            disk_erase_confirmed: false,
        }
    }
}

impl Default for InstallerState {
    fn default() -> Self {
        Self {
            step: InstallerStep::Welcome,
            phase: InstallPhase::Validate,
            config: InstallerConfig::default(),
            password: None,
            password_confirmation: None,
            logs: Vec::new(),
        }
    }
}
