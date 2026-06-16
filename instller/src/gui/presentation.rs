use crate::api::install::InstallPhase;

pub const APP_TITLE: &str = "JadeOS Installer";
pub const APP_SUBTITLE: &str =
    "A safety-first scaffold for the JadeOS Live CD installer. GUI state stays in the app layer, while disk and install operations stay in api modules.";

pub const WELCOME_BULLETS: [&str; 3] = [
    "The selected disk will be fully erased.",
    "MVP support is limited to UEFI + GPT + ext4.",
    "Manual partitioning, GitHub integration, and encryption are intentionally out of scope.",
];

pub const SUMMARY_FIXED_SETTINGS: [(&str, &str); 7] = [
    ("Boot mode", "UEFI"),
    ("Partition table", "GPT"),
    ("Filesystem", "ext4"),
    ("Swap", "none"),
    ("Encryption", "none"),
    ("Desktop", "Niri"),
    ("Locale", "ja_JP.UTF-8"),
];

pub const ERASE_CONFIRMATION_COPY: &str =
    "I understand that the selected disk will be completely erased.";

pub fn install_phases() -> [InstallPhase; 8] {
    [
        InstallPhase::Validate,
        InstallPhase::Partition,
        InstallPhase::Format,
        InstallPhase::Mount,
        InstallPhase::GenerateConfig,
        InstallPhase::InstallSystem,
        InstallPhase::SetPassword,
        InstallPhase::Finish,
    ]
}
