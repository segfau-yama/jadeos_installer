use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Row};

#[component]
pub fn ModalDialog(is_visible: bool, children: Element) -> Element {
    if !is_visible {
        return rsx! { Fragment {} };
    }

    rsx! {
        Flexbox {
            items: "items-center".to_string(),
            justify: "justify-center".to_string(),
            class: "fixed inset-0 z-50 px-4 py-8 backdrop-blur-sm".to_string(),
            style: format!(
                "background-color: color-mix(in srgb, {} 72%, transparent);",
                ThemeColor::BackGround.css_var()
            ),
            Row {
                cols: "grid-cols-1".to_string(),
                gap: "gap-4".to_string(),
                class: "rounded-[2rem] border p-6 shadow-none".to_string(),
                style: format!(
                    "background-color: {}; border-color: color-mix(in srgb, {} 22%, transparent);",
                    ThemeColor::Surface.css_var(),
                    ThemeColor::Secondary.css_var(),
                ),
                {children}
            }
        }
    }
}
