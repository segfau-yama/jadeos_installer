use std::error::Error;
use std::fmt::{Display, Formatter};

pub const DEFAULT_REPOSITORY_URL: &str = "https://github.com/segfau-yama/nixos_configuration.git";
pub const INSTALL_BOOT_SIZE: &str = "512MiB";
pub const INSTALL_MOUNT_ROOT: &str = "/mnt";

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

impl InstallPhase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Validate => "Validate",
            Self::Partition => "Partition",
            Self::Format => "Format",
            Self::Mount => "Mount",
            Self::GenerateConfig => "Generate config",
            Self::InstallSystem => "Install system",
            Self::SetPassword => "Set password",
            Self::Finish => "Finish",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallCommand {
    pub phase: InstallPhase,
    pub description: String,
    pub argv: Vec<String>,
    pub destructive: bool,
}

impl InstallCommand {
    pub fn render_command(&self) -> String {
        self.argv.join(" ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    pub repository_url: String,
    pub hostname: String,
    pub target_disk: String,
    pub efi_partition: String,
    pub root_partition: String,
    pub commands: Vec<InstallCommand>,
}

impl InstallPlan {
    pub fn rendered_commands(&self) -> Vec<String> {
        self.commands
            .iter()
            .map(InstallCommand::render_command)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallationReport {
    pub final_phase: InstallPhase,
    pub current_command: Option<String>,
    pub log: Vec<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallError {
    EmptyHostname,
    EmptyUsername,
    EmptyTargetDisk,
    InvalidTargetDisk(String),
    DiskEraseNotConfirmed,
}

impl Display for InstallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyHostname => write!(f, "hostname is required"),
            Self::EmptyUsername => write!(f, "username is required"),
            Self::EmptyTargetDisk => write!(f, "target disk is required"),
            Self::InvalidTargetDisk(path) => {
                write!(f, "target disk must be a whole disk device, got {path}")
            }
            Self::DiskEraseNotConfirmed => {
                write!(f, "the destructive disk erase confirmation must be checked")
            }
        }
    }
}

impl Error for InstallError {}
