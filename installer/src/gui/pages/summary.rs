use crate::gui::components::ThemeColor;
use dioxus::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::mpsc;

use crate::api::install::{generate_install_plan, preview_install_plan};
#[cfg(target_arch = "wasm32")]
use crate::api::install::{run_install_plan, InstallationReport};
#[cfg(not(target_arch = "wasm32"))]
use crate::api::install::{run_install_plan_with_progress, InstallationReport};
use crate::gui::components::{
    BadgeTone, ButtonVariant, Card, CardBody, Col, Flexbox, Row, StatusBadge, Theme, TogglePill,
    Typography, TypographyTag, UiButton,
};
use crate::gui::routes::{next_route, previous_route, Route};
use crate::gui::state::{
    InstallRuntime, InstallerConfig, InstallerContext, InstallerUiState, UserDraft,
};
use crate::gui::views::{ActionRow, InfoTile, PageIntro, PageSection, ValidationList};

const SUMMARY_FIXED_SETTINGS: [(&str, &str); 7] = [
    ("Boot mode", "UEFI"),
    ("Partition table", "GPT"),
    ("Filesystem", "ext4"),
    ("Swap", "none"),
    ("Encryption", "none"),
    ("Desktop", "Hyprland"),
    ("Locale", "ja_JP.UTF-8"),
];

const ERASE_CONFIRMATION_COPY: &str =
    "I understand that the selected disk will be completely erased.";

#[component]
pub fn SummaryPage() -> Element {
    let installer = use_context::<InstallerContext>();
    let theme = use_context::<Theme>();
    let mut config = installer.config;
    let user = installer.user;
    let mut ui = installer.ui;
    let runtime = installer.runtime;
    #[cfg(not(target_arch = "wasm32"))]
    let install_progress = installer.install_progress;
    let config_snapshot = config();
    let user_snapshot = user();
    let plan_preview = preview_plan(&config_snapshot);
    let validation_errors = summary_validation_errors(&config_snapshot, &user_snapshot);
    let install_ready = validation_errors.is_empty() && plan_preview.is_some();
    let navigator = use_navigator();
    let back_navigator = navigator.clone();
    let install_navigator = navigator.clone();

    rsx! {
        PageSection {
            PageIntro {
                title: "Summary".to_string(),
                description: "Review the chosen inputs and the fixed MVP settings before starting the install flow.".to_string(),
            }
            Row {
                cols: "grid-cols-1 md:grid-cols-2 xl:grid-cols-3".to_string(),
                gap: "gap-4".to_string(),
                Col {
                    InfoTile {
                        label: "Hostname".to_string(),
                        value: config_snapshot.hostname.clone(),
                    }
                }
                Col {
                    InfoTile {
                        label: "Username".to_string(),
                        value: config_snapshot.username.clone(),
                    }
                }
                Col {
                    InfoTile {
                        label: "Target disk".to_string(),
                        value: config_snapshot.target_disk.clone(),
                    }
                }
                for (label, value) in SUMMARY_FIXED_SETTINGS {
                    Col {
                        key: "{label}",
                        InfoTile {
                            label: label.to_string(),
                            value: value.to_string(),
                        }
                    }
                }
            }
            Card {
                color: theme.bg(ThemeColor::Warning).to_string(),
                class: theme.border(ThemeColor::Warning).to_string(),
                shadow: "shadow-none".to_string(),
                rounded: "rounded-[1.75rem]".to_string(),
                CardBody {
                    class: "gap-4".to_string(),
                    Flexbox {
                        wrap: "flex-wrap".to_string(),
                        items: "items-center".to_string(),
                        gap: "gap-3".to_string(),
                        StatusBadge {
                            tone: BadgeTone::Warning,
                            "Destructive confirmation"
                        }
                        Typography {
                            tag: TypographyTag::P,
                            class: format!("m-0 text-sm font-medium {}", theme.text(ThemeColor::Warning)),
                            "{ERASE_CONFIRMATION_COPY}"
                        }
                    }
                    TogglePill {
                        selected: config_snapshot.disk_erase_confirmed,
                        onpress: move |_| {
                            let is_confirmed = config().disk_erase_confirmed;
                            config.write().disk_erase_confirmed = !is_confirmed;
                            ui.write().error_message = None;
                        },
                        if config_snapshot.disk_erase_confirmed {
                            "Erase confirmed"
                        } else {
                            "Tap to confirm disk erase"
                        }
                    }
                }
            }
            ValidationList { messages: validation_errors }
            ActionRow {
                UiButton {
                    variant: ButtonVariant::Ghost,
                    onpress: move |_: MouseEvent| {
                        if let Some(route) = previous_route(&Route::Summary {}) {
                            back_navigator.push(route);
                        }
                    },
                    "Back"
                }
                UiButton {
                    disabled: !install_ready,
                    onpress: move |_: MouseEvent| {
                        #[cfg(not(target_arch = "wasm32"))]
                        let started = start_install(config, user, ui, runtime, install_progress);
                        #[cfg(target_arch = "wasm32")]
                        let started = start_install(config, user, ui, runtime);

                        if started {
                            if let Some(route) = next_route(&Route::Summary {}) {
                                install_navigator.push(route);
                            }
                        }
                    },
                    "Install"
                }
            }
        }
    }
}

fn preview_plan(config: &InstallerConfig) -> Option<crate::api::install::InstallPlan> {
    preview_install_plan(config).ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn start_install(
    config: Signal<InstallerConfig>,
    user: Signal<UserDraft>,
    mut ui: Signal<InstallerUiState>,
    mut runtime: Signal<InstallRuntime>,
    mut install_progress: Signal<Option<mpsc::UnboundedReceiver<InstallationReport>>>,
) -> bool {
    let summary_errors = summary_validation_errors(&config(), &user());

    if !summary_errors.is_empty() {
        ui.write().error_message = Some(summary_errors.join(" "));
        return false;
    }

    let config_snapshot = config();
    let user_snapshot = user();
    let install_config = config_snapshot.clone();
    let password = user_snapshot.password.clone();
    match generate_install_plan(&install_config) {
        Ok(plan) => {
            {
                let mut runtime_state = runtime.write();
                runtime_state.install_plan = Some(plan.clone());
                runtime_state.install_phase = crate::api::install::InstallPhase::Validate;
                runtime_state.current_command =
                    Some("Starting install in the background...".to_string());
                runtime_state.install_log = vec![
                    format!(
                        "Queued install for {} on {}",
                        install_config.hostname, install_config.target_disk
                    ),
                    "Switching to the install page and starting background execution.".to_string(),
                ];
            }
            ui.write().error_message = None;
            let (progress_tx, progress_rx) = mpsc::unbounded_channel::<InstallationReport>();
            *install_progress.write() = Some(progress_rx);
            std::thread::spawn(move || {
                let _ =
                    run_install_plan_with_progress(&install_config, &password, &plan, |report| {
                        let _ = progress_tx.send(report.clone());
                    });
            });
            true
        }
        Err(error) => {
            ui.write().error_message = Some(error.to_string());
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn start_install(
    config: Signal<InstallerConfig>,
    user: Signal<UserDraft>,
    mut ui: Signal<InstallerUiState>,
    mut runtime: Signal<InstallRuntime>,
) -> bool {
    let summary_errors = summary_validation_errors(&config(), &user());

    if !summary_errors.is_empty() {
        ui.write().error_message = Some(summary_errors.join(" "));
        return false;
    }

    let config_snapshot = config();
    let user_snapshot = user();
    let install_config = config_snapshot.clone();
    let password = user_snapshot.password.clone();
    let Ok(plan) = generate_install_plan(&install_config) else {
        ui.write().error_message = Some("failed to generate install plan".to_string());
        return false;
    };
    let report = run_install_plan(&install_config, &password, &plan);
    update_runtime_state(&mut runtime, &mut ui, &plan, report);
    true
}

#[cfg(target_arch = "wasm32")]
fn update_runtime_state(
    runtime: &mut Signal<InstallRuntime>,
    ui: &mut Signal<InstallerUiState>,
    plan: &crate::api::install::InstallPlan,
    report: InstallationReport,
) {
    let mut runtime_state = runtime.write();
    runtime_state.install_plan = Some(plan.clone());
    runtime_state.install_phase = report.final_phase;
    runtime_state.current_command = report.current_command;
    runtime_state.install_log = report.log;
    ui.write().error_message = report.error_message;
}

fn summary_validation_errors(config: &InstallerConfig, user: &UserDraft) -> Vec<String> {
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

    if config.target_disk.trim().is_empty() {
        errors.push("A target disk must be selected.".to_string());
    }

    if !config.disk_erase_confirmed {
        errors.push("The disk erase confirmation must be checked.".to_string());
    }

    errors
}
