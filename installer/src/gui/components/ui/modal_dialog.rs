use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Row, Theme};

#[component]
pub fn ModalDialog(is_visible: bool, children: Element) -> Element {
    if !is_visible {
        return rsx! { Fragment {} };
    }
    let theme = use_context::<Theme>();

    rsx! {
        Flexbox {
            items: "items-center".to_string(),
            justify: "justify-center".to_string(),
            class: format!("fixed inset-0 z-50 {} px-4 py-8 backdrop-blur-sm", theme.colors.overlay_bg),
            Row {
                cols: "grid-cols-1".to_string(),
                gap: "gap-4".to_string(),
                class: format!(
                    "{} border {} {} p-6 {}",
                    theme.shape.dialog_radius,
                    theme.colors.border_subtle,
                    theme.colors.surface_elevated,
                    theme.shadow.overlay
                ),
                {children}
            }
        }
    }
}
