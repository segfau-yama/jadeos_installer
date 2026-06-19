use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Typography, TypographyTag};

#[component]
fn ValidationMessage(message: String) -> Element {
    rsx! {
        div {
            class: "rounded-3xl border px-4 py-3".to_string(),
            style: format!(
                "background-color: color-mix(in srgb, {} 10%, {}); border-color: color-mix(in srgb, {} 22%, transparent);",
                ThemeColor::Error.css_var(),
                ThemeColor::Surface.css_var(),
                ThemeColor::Error.css_var(),
            ),
            Typography {
                tag: TypographyTag::P,
                class: "m-0 text-sm font-medium".to_string(),
                style: format!("color: {};", ThemeColor::Error.css_var()),
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
