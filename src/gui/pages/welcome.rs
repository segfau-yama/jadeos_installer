use dioxus::prelude::*;
use dioxus_material::{Button, Chip};

use crate::gui::controller::navigate_to;
use crate::gui::presentation::WELCOME_BULLETS;
use crate::gui::state::{InstallerState, InstallerStep};

#[component]
pub fn WelcomePage(state: Signal<InstallerState>) -> Element {
    rsx! {
        section {
            style: "display: flex; flex-direction: column; gap: 20px;",
            div {
                style: "display: flex; flex-direction: column; gap: 10px;",
                h2 {
                    style: "margin: 0; color: #10201b; font-size: clamp(1.7rem, 4vw, 2.4rem);",
                    "Welcome"
                }
                p {
                    style: "margin: 0; color: #51625a; max-width: 64ch;",
                    "Install JadeOS from the Live CD using a focused, safety-first workflow."
                }
            }
            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px;",
                for bullet in WELCOME_BULLETS {
                    div {
                        key: "{bullet}",
                        Chip {
                            is_selected: Some(true),
                            onclick: move |_| {},
                            "{bullet}"
                        }
                    }
                }
            }
            div {
                style: "display: flex; justify-content: flex-start;",
                Button {
                    onpress: move |_| {
                    navigate_to(state, InstallerStep::User);
                    },
                    "Start"
                }
            }
        }
    }
}
