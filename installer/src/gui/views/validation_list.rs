use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Theme, Typography, TypographyTag};

#[component]
fn ValidationMessage(message: String) -> Element {
    let theme = use_context::<Theme>();

    rsx! {
        div {
            class: format!(
                "rounded-3xl border {} {} px-4 py-3",
                theme.colors.border_danger, theme.colors.danger_bg
            ),
            Typography {
                tag: TypographyTag::P,
                class: format!("m-0 text-sm font-medium {}", theme.colors.danger_fg),
                "{message}"
            }
        }
    }
}

#[component]
pub fn ValidationList(messages: Vec<String>) -> Element {
    if messages.is_empty() {
        return rsx! { Fragment {} };
    }

    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-3".to_string(),
            for message in messages {
                ValidationMessage {
                    key: "{message}",
                    message: message,
                }
            }
        }
    }
}
