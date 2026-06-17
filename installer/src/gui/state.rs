use dioxus::prelude::Signal;
use serde::{Deserialize, Serialize};

use crate::api::disk::DiskDeviceInfo;
use crate::api::install::{InstallPhase, InstallPlan};

pub const INSTALLER_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallerConfig {
    pub schema_version: u32,
    pub hostname: String,
    pub username: String,
    pub target_disk: String,
    pub disk_erase_confirmed: bool,
}

impl Default for InstallerConfig {
    fn default() -> Self {
        Self {
            schema_version: INSTALLER_SCHEMA_VERSION,
            hostname: "jadeos".to_string(),
            username: "jade".to_string(),
            target_disk: String::new(),
            disk_erase_confirmed: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserDraft {
    pub password: String,
    pub password_confirmation: String,
}

impl Default for UserDraft {
    fn default() -> Self {
        Self {
            password: String::new(),
            password_confirmation: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstallerUiState {
    pub available_disks: Vec<DiskDeviceInfo>,
    pub error_message: Option<String>,
}

impl Default for InstallerUiState {
    fn default() -> Self {
        Self {
            available_disks: Vec::new(),
            error_message: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstallRuntime {
    pub install_plan: Option<InstallPlan>,
    pub install_phase: InstallPhase,
    pub current_command: Option<String>,
    pub install_log: Vec<String>,
}

impl Default for InstallRuntime {
    fn default() -> Self {
        Self {
            install_plan: None,
            install_phase: InstallPhase::Validate,
            current_command: None,
            install_log: Vec::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct InstallerContext {
    pub config: Signal<InstallerConfig>,
    pub user: Signal<UserDraft>,
    pub ui: Signal<InstallerUiState>,
    pub runtime: Signal<InstallRuntime>,
}
