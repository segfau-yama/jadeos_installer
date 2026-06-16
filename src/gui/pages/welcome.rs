use dioxus::prelude::*;

use crate::gui::components::{BadgeTone, Flexbox, PageIntro, PageSection, StatusBadge, UiButton};
use crate::gui::presentation::WELCOME_BULLETS;
use crate::gui::routes::Route;

#[component]
pub fn WelcomePage() -> Element {
    let navigator = use_navigator();

    rsx! {
        PageSection {
            PageIntro {
                title: "Welcome".to_string(),
                description: "Install JadeOS from the Live CD using a focused, safety-first workflow.".to_string(),
            }
            Flexbox {
                wrap: "flex-wrap".to_string(),
                gap: "gap-3".to_string(),
                for bullet in WELCOME_BULLETS {
                    StatusBadge {
                        key: "{bullet}",
                        tone: BadgeTone::Accent,
                        "{bullet}"
                    }
                }
            }
            UiButton {
                onpress: move |_: MouseEvent| {
                    navigator.push(Route::User {});
                },
                class: "self-start".to_string(),
                "Start"
            }
        }
    }
}
