use dioxus::prelude::*;

use crate::gui::components::{
    ButtonVariant, Card, CardBody, CardHeader, Col, Row, TextInput, Typography, TypographyTag,
    Theme, UiButton,
};
use crate::gui::routes::{next_route, previous_route, Route};
use crate::gui::state::{InstallerConfig, InstallerContext, InstallerUiState, UserDraft};
use crate::gui::views::{ActionRow, PageIntro, PageSection, ValidationList};

#[component]
pub fn UserPage() -> Element {
    let installer = use_context::<InstallerContext>();
    let theme = use_context::<Theme>();
    let mut config = installer.config;
    let mut user = installer.user;
    let mut ui = installer.ui;
    let config_snapshot = config();
    let user_snapshot = user();
    let validation_errors = user_validation_errors(&config_snapshot, &user_snapshot);
    let navigator = use_navigator();
    let back_navigator = navigator.clone();
    let continue_navigator = navigator.clone();

    rsx! {
        PageSection {
            gap: "gap-6".to_string(),
            PageIntro {
                title: "User".to_string(),
                description: "Create the normal user for the installed system. This scaffold keeps password data in memory only.".to_string(),
            }
            Card {
                color: theme.colors.surface_muted.to_string(),
                CardHeader {
                    Typography {
                        tag: TypographyTag::P,
                        class: format!(
                            "m-0 text-xs font-bold uppercase tracking-[0.16em] {}",
                            theme.colors.text_muted
                        ),
                        "Account setup"
                    }
                    Typography {
                        tag: TypographyTag::P,
                        class: format!("m-0 text-base leading-7 {}", theme.colors.text_secondary),
                        "Keep the system identity and login credentials compact and easy to scan."
                    }
                }
                CardBody {
                    class: "pt-0".to_string(),
                    Row {
                        cols: "grid-cols-1 md:grid-cols-2".to_string(),
                        gap: "gap-5 md:gap-6".to_string(),
                        Col {
                            TextInput {
                                label: "Hostname".to_string(),
                                value: config_snapshot.hostname.clone(),
                                supporting_text: Some("Used for the installed system hostname.".to_string()),
                                autocomplete: Some("off".to_string()),
                                onchange: move |event: FormEvent| {
                                    config.write().hostname = event.value();
                                    ui.write().error_message = None;
                                }
                            }
                        }
                        Col {
                            TextInput {
                                label: "Username".to_string(),
                                value: config_snapshot.username.clone(),
                                supporting_text: Some("This becomes the main login account.".to_string()),
                                autocomplete: Some("username".to_string()),
                                onchange: move |event: FormEvent| {
                                    config.write().username = event.value();
                                    ui.write().error_message = None;
                                }
                            }
                        }
                        Col {
                            TextInput {
                                label: "Password".to_string(),
                                value: user_snapshot.password.clone(),
                                input_type: Some("password".to_string()),
                                autocomplete: Some("new-password".to_string()),
                                supporting_text: Some("Kept only in memory in this scaffold.".to_string()),
                                onchange: move |event: FormEvent| {
                                    user.write().password = event.value();
                                    ui.write().error_message = None;
                                }
                            }
                        }
                        Col {
                            TextInput {
                                label: "Password confirmation".to_string(),
                                value: user_snapshot.password_confirmation.clone(),
                                input_type: Some("password".to_string()),
                                autocomplete: Some("new-password".to_string()),
                                supporting_text: Some("Repeat the same password to continue.".to_string()),
                                onchange: move |event: FormEvent| {
                                    user.write().password_confirmation = event.value();
                                    ui.write().error_message = None;
                                }
                            }
                        }
                    }
                }
            }
            ValidationList { messages: validation_errors }
            ActionRow {
                UiButton {
                    variant: ButtonVariant::Ghost,
                    onpress: move |_: MouseEvent| {
                        if let Some(route) = previous_route(&Route::User {}) {
                            back_navigator.push(route);
                        }
                    },
                    "Back"
                }
                UiButton {
                    onpress: move |_: MouseEvent| {
                        if continue_from_user(config, user, ui) {
                            if let Some(route) = next_route(&Route::User {}) {
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

fn continue_from_user(
    config: Signal<InstallerConfig>,
    user: Signal<UserDraft>,
    mut ui: Signal<InstallerUiState>,
) -> bool {
    let errors = user_validation_errors(&config(), &user());

    if errors.is_empty() {
        ui.write().error_message = None;
        true
    } else {
        ui.write().error_message = Some(errors.join(" "));
        false
    }
}

fn user_validation_errors(config: &InstallerConfig, user: &UserDraft) -> Vec<String> {
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
