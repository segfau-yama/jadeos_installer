use dioxus::prelude::{Signal, WritableExt};

use crate::gui::state::{InstallerConfig, InstallerUiState, UserDraft};

pub fn apply_validation_errors(mut ui: Signal<InstallerUiState>, errors: Vec<String>) -> bool {
    if errors.is_empty() {
        ui.write().error_message = None;
        true
    } else {
        ui.write().error_message = Some(errors.join(" "));
        false
    }
}

pub fn disk_validation_errors(config: &InstallerConfig) -> Vec<String> {
    let mut errors = Vec::new();

    if config.target_disk.trim().is_empty() {
        errors.push("A target disk must be selected.".to_string());
    }

    errors
}

pub fn user_validation_errors(config: &InstallerConfig, user: &UserDraft) -> Vec<String> {
    let mut errors = Vec::new();

    if config.hostname.trim().is_empty() {
        errors.push("Hostname is required.".to_string());
    }

    if config.username.trim().is_empty() {
        errors.push("Username is required.".to_string());
    }

    if user.password.trim().is_empty() {
        errors.push("Password is required.".to_string());
    }

    if user.password != user.password_confirmation {
        errors.push("Password confirmation must match.".to_string());
    }

    errors
}

pub fn summary_validation_errors(config: &InstallerConfig, user: &UserDraft) -> Vec<String> {
    let mut errors = user_validation_errors(config, user);
    errors.extend(disk_validation_errors(config));

    if !config.disk_erase_confirmed {
        errors.push("The disk erase confirmation must be checked.".to_string());
    }

    errors
}
