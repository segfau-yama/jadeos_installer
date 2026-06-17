use dioxus::prelude::*;

use crate::gui::components::{BadgeTone, Flexbox, StatusBadge, UiButton};
use crate::gui::routes::{next_route, Route};
use crate::gui::views::{PageIntro, PageSection};

const WELCOME_BULLETS: [&str; 3] = [
    "The selected disk will be fully erased.",
    "MVP support is limited to UEFI + GPT + ext4.",
    "Manual partitioning and encryption are intentionally out of scope.",
];

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
                    if let Some(route) = next_route(&Route::Welcome {}) {
                        navigator.push(route);
                    }
                },
                class: "self-start".to_string(),
                "Start"
            }
        }
    }
}
