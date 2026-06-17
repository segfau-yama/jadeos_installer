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
            class: "fixed inset-0 z-50 bg-emerald-950/40 px-4 py-8 backdrop-blur-sm".to_string(),
            Row {
                cols: "grid-cols-1".to_string(),
                gap: "gap-4".to_string(),
                class: "rounded-[2rem] border border-white/50 bg-white/95 p-6 shadow-[0_30px_90px_rgba(15,23,42,0.22)]".to_string(),
                {children}
            }
        }
    }
}
