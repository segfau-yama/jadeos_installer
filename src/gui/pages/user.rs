use dioxus::prelude::*;
use dioxus_material::{Button, TextButton, TextField};

use crate::gui::components::ValidationList;
use crate::gui::controller::{continue_from_user, navigate_to};
use crate::gui::state::{InstallerState, InstallerStep};
use crate::gui::validation::user_validation_errors;

#[component]
pub fn UserPage(mut state: Signal<InstallerState>) -> Element {
    let snapshot = state();
    let validation_errors = user_validation_errors(&snapshot.config, &snapshot.user);

    rsx! {
        section {
            div {
                style: "display: flex; flex-direction: column; gap: 10px;",
                h2 {
                    style: "margin: 0; color: #10201b; font-size: 2rem;",
                    "User"
                }
                p {
                    style: "margin: 0; color: #51625a; max-width: 60ch;",
                    "Create the normal user for the installed system. This scaffold keeps password data in memory only."
                }
            }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 18px;",
                TextField {
                    label: "Hostname".to_string(),
                    value: snapshot.config.hostname.clone(),
                    supporting_text: Some("Used for the installed system hostname.".to_string()),
                    onchange: move |event: FormEvent| {
                        let mut draft = state.write();
                        draft.config.hostname = event.value();
                        draft.ui.error_message = None;
                    }
                }
                TextField {
                    label: "Username".to_string(),
                    value: snapshot.config.username.clone(),
                    supporting_text: Some("This becomes the main login account.".to_string()),
                    onchange: move |event: FormEvent| {
                        let mut draft = state.write();
                        draft.config.username = event.value();
                        draft.ui.error_message = None;
                    }
                }
                TextField {
                    label: "Password".to_string(),
                    value: snapshot.user.password.clone(),
                    input_type: Some("password".to_string()),
                    supporting_text: Some("Kept only in memory in this scaffold.".to_string()),
                    onchange: move |event: FormEvent| {
                        let mut draft = state.write();
                        draft.user.password = event.value();
                        draft.ui.error_message = None;
                    }
                }
                TextField {
                    label: "Password confirmation".to_string(),
                    value: snapshot.user.password_confirmation.clone(),
                    input_type: Some("password".to_string()),
                    supporting_text: Some("Repeat the same password to continue.".to_string()),
                    onchange: move |event: FormEvent| {
                        let mut draft = state.write();
                        draft.user.password_confirmation = event.value();
                        draft.ui.error_message = None;
                    }
                }
            }
            ValidationList { messages: validation_errors }
            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px; margin-top: 8px;",
                TextButton {
                    onpress: move |_| {
                        navigate_to(state, InstallerStep::Welcome);
                    },
                    "Back"
                }
                Button {
                    onpress: move |_| continue_from_user(state),
                    "Continue"
                }
            }
        }
    }
}
