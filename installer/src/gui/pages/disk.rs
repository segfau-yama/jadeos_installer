use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::api::disk::list_disks;
use crate::gui::components::{ButtonVariant, Flexbox, Theme, Typography, TypographyTag, UiButton};
use crate::gui::routes::{next_route, previous_route, Route};
use crate::gui::state::{InstallerConfig, InstallerContext, InstallerUiState};
use crate::gui::views::{ActionRow, DiskCard, NoticePanel, PageIntro, PageSection, ValidationList};

#[component]
pub fn DiskPage() -> Element {
    let installer = use_context::<InstallerContext>();
    let theme = use_context::<Theme>();
    let mut config = installer.config;
    let mut ui = installer.ui;
    let config_snapshot = config();
    let ui_snapshot = ui();
    let validation_errors = disk_validation_errors(&config_snapshot);
    let selected_disk = config_snapshot.target_disk.clone();
    let navigator = use_navigator();
    let back_navigator = navigator.clone();
    let continue_navigator = navigator.clone();

    rsx! {
        PageSection {
            PageIntro {
                title: "Disk".to_string(),
                description: "Choose the whole disk that should be erased and installed.".to_string(),
            }
            UiButton {
                variant: ButtonVariant::Ghost,
                onpress: move |_: MouseEvent| refresh_disks(ui),
                class: "self-start".to_string(),
                "Refresh disks"
            }
            if ui_snapshot.available_disks.is_empty() {
                NoticePanel {
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0".to_string(),
                        "No disks loaded yet. Press \"Refresh disks\" to inspect available devices."
                    }
                }
            } else {
                Flexbox {
                    direction: "flex-col".to_string(),
                    gap: "gap-4".to_string(),
                    for disk in ui_snapshot.available_disks.iter().cloned() {
                        {
                            let disk_path = disk.path.clone();
                            let is_selected = selected_disk == disk_path;

                            rsx! {
                                DiskCard {
                                    disk: disk,
                                    is_selected: is_selected,
                                    on_select: move |_| {
                                        config.write().target_disk = disk_path.clone();
                                        ui.write().error_message = None;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !config_snapshot.target_disk.is_empty() {
                Typography {
                    tag: TypographyTag::P,
                    class: format!("m-0 text-sm font-semibold {}", theme.color(ThemeColor::TextAccent)),
                    "Current selection: {config_snapshot.target_disk}"
                }
            }
            ValidationList { messages: validation_errors }
            ActionRow {
                UiButton {
                    variant: ButtonVariant::Ghost,
                    onpress: move |_: MouseEvent| {
                        if let Some(route) = previous_route(&Route::Disk {}) {
                            back_navigator.push(route);
                        }
                    },
                    "Back"
                }
                UiButton {
                    onpress: move |_: MouseEvent| {
                        if continue_from_disk(config, ui) {
                            if let Some(route) = next_route(&Route::Disk {}) {
                                continue_navigator.push(route);
                            }
                        }
                    },
                    "Continue"
                }
            }
        }
    }
}

fn refresh_disks(mut ui: Signal<InstallerUiState>) {
    match list_disks() {
        Ok(disks) => {
            let mut draft = ui.write();
            draft.available_disks = disks;
            draft.error_message = None;
        }
        Err(error) => ui.write().error_message = Some(error.to_string()),
    }
}

fn continue_from_disk(config: Signal<InstallerConfig>, mut ui: Signal<InstallerUiState>) -> bool {
    let errors = disk_validation_errors(&config());
    if errors.is_empty() {
        ui.write().error_message = None;
        true
    } else {
        ui.write().error_message = Some(errors.join(" "));
        false
    }
}

fn disk_validation_errors(config: &InstallerConfig) -> Vec<String> {
    let mut errors = Vec::new();

    if config.target_disk.trim().is_empty() {
        errors.push("A target disk must be selected.".to_string());
    }

    errors
}
