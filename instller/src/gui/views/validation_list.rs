use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Typography, TypographyTag};

#[component]
fn ValidationMessage(message: String) -> Element {
    rsx! {
        div {
            class: "rounded-3xl border border-rose-200 bg-rose-50/90 px-4 py-3",
            Typography {
                tag: TypographyTag::P,
                class: "m-0 text-sm font-medium text-rose-700".to_string(),
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
