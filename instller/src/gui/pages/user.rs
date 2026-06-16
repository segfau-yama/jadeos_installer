use dioxus::prelude::*;

use crate::gui::components::{
    ButtonVariant, Card, CardBody, CardHeader, Col, Row, TextInput, Typography, TypographyTag,
    UiButton,
};
use crate::gui::controller::continue_from_user;
use crate::gui::routes::Route;
use crate::gui::state::use_installer_state;
use crate::gui::validation::user_validation_errors;
use crate::gui::views::{ActionRow, PageIntro, PageSection, ValidationList};

#[component]
pub fn UserPage() -> Element {
    let mut state = use_installer_state();
    let snapshot = state();
    let validation_errors = user_validation_errors(&snapshot.config, &snapshot.user);
    let navigator = use_navigator();
    let back_navigator = navigator.clone();
    let continue_navigator = navigator.clone();

    rsx! {
        PageSection {
            class: "max-w-5xl".to_string(),
            gap: "gap-6".to_string(),
            PageIntro {
                title: "User".to_string(),
                description: "Create the normal user for the installed system. This scaffold keeps password data in memory only.".to_string(),
                class: "max-w-2xl".to_string(),
            }
            Card {
                color: "bg-white/75".to_string(),
                CardHeader {
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0 text-xs font-bold uppercase tracking-[0.16em] text-emerald-900/65".to_string(),
                        "Account setup"
                    }
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0 max-w-2xl text-base leading-7 text-emerald-900/70".to_string(),
                        "Keep the system identity and login credentials compact and easy to scan."
                    }
                }
                CardBody {
                    class: "pt-0".to_string(),
                    Row {
                        cols: "grid-cols-1 xl:grid-cols-2".to_string(),
                        gap: "gap-5 md:gap-6".to_string(),
                        Col {
                            TextInput {
                                label: "Hostname".to_string(),
                                value: snapshot.config.hostname.clone(),
                                supporting_text: Some("Used for the installed system hostname.".to_string()),
                                autocomplete: Some("off".to_string()),
                                onchange: move |event: FormEvent| {
                                    let mut draft = state.write();
                                    draft.config.hostname = event.value();
                                    draft.ui.error_message = None;
                                }
                            }
                        }
                        Col {
                            TextInput {
                                label: "Username".to_string(),
                                value: snapshot.config.username.clone(),
                                supporting_text: Some("This becomes the main login account.".to_string()),
                                autocomplete: Some("username".to_string()),
                                onchange: move |event: FormEvent| {
                                    let mut draft = state.write();
                                    draft.config.username = event.value();
                                    draft.ui.error_message = None;
                                }
                            }
                        }
                        Col {
                            TextInput {
                                label: "Password".to_string(),
                                value: snapshot.user.password.clone(),
                                input_type: Some("password".to_string()),
                                autocomplete: Some("new-password".to_string()),
                                supporting_text: Some("Kept only in memory in this scaffold.".to_string()),
                                onchange: move |event: FormEvent| {
                                    let mut draft = state.write();
                                    draft.user.password = event.value();
                                    draft.ui.error_message = None;
                                }
                            }
                        }
                        Col {
                            TextInput {
                                label: "Password confirmation".to_string(),
                                value: snapshot.user.password_confirmation.clone(),
                                input_type: Some("password".to_string()),
                                autocomplete: Some("new-password".to_string()),
                                supporting_text: Some("Repeat the same password to continue.".to_string()),
                                onchange: move |event: FormEvent| {
                                    let mut draft = state.write();
                                    draft.user.password_confirmation = event.value();
                                    draft.ui.error_message = None;
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
                        back_navigator.push(Route::Welcome {});
                    },
                    "Back"
                }
                UiButton {
                    onpress: move |_: MouseEvent| {
                        if continue_from_user(state) {
                            continue_navigator.push(Route::Disk {});
                        }
                    },
                    "Continue"
                }
            }
        }
    }
}
