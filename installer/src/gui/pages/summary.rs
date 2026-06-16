use dioxus::prelude::*;

use crate::gui::components::{
    BadgeTone, ButtonVariant, Card, CardBody, Col, Flexbox, Row, StatusBadge, TogglePill,
    Typography, TypographyTag, UiButton,
};
use crate::gui::controller::{preview_plan, start_install};
use crate::gui::presentation::{ERASE_CONFIRMATION_COPY, SUMMARY_FIXED_SETTINGS};
use crate::gui::routes::Route;
use crate::gui::state::use_installer_state;
use crate::gui::validation::summary_validation_errors;
use crate::gui::views::{ActionRow, InfoTile, PageIntro, PageSection, ValidationList};

#[component]
pub fn SummaryPage() -> Element {
    let mut state = use_installer_state();
    let snapshot = state();
    let plan_preview = preview_plan(&snapshot);
    let validation_errors = summary_validation_errors(&snapshot.config, &snapshot.user);
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
                        value: snapshot.config.hostname.clone(),
                    }
                }
                Col {
                    InfoTile {
                        label: "Username".to_string(),
                        value: snapshot.config.username.clone(),
                    }
                }
                Col {
                    InfoTile {
                        label: "Target disk".to_string(),
                        value: snapshot.config.target_disk.clone(),
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
                color: "bg-amber-50/80".to_string(),
                class: "border-amber-200".to_string(),
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
                            class: "m-0 text-sm font-medium text-amber-800".to_string(),
                            "{ERASE_CONFIRMATION_COPY}"
                        }
                    }
                    TogglePill {
                        selected: snapshot.config.disk_erase_confirmed,
                        onpress: move |_| {
                            let mut draft = state.write();
                            draft.config.disk_erase_confirmed = !draft.config.disk_erase_confirmed;
                            draft.ui.error_message = None;
                        },
                        if snapshot.config.disk_erase_confirmed {
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
                        back_navigator.push(Route::Disk {});
                    },
                    "Back"
                }
                UiButton {
                    disabled: !install_ready,
                    onpress: move |_: MouseEvent| {
                        if start_install(state) {
                            install_navigator.push(Route::Install {});
                        }
                    },
                    "Install"
                }
            }
        }
    }
}
