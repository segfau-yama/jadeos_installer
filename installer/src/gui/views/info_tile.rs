use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Typography, TypographyTag};

#[derive(PartialEq, Clone, Props)]
pub struct InfoTileProps {
    label: String,
    value: String,
    #[props(default = String::new())]
    class: String,
}

#[component]
pub fn InfoTile(props: InfoTileProps) -> Element {
    rsx! {
        div {
            class: format!("rounded-[1.5rem] border px-5 py-4 {}", props.class),
            style: format!(
                "background-color: color-mix(in srgb, {} 8%, {}); border-color: color-mix(in srgb, {} 22%, transparent);",
                ThemeColor::Primary.css_var(),
                ThemeColor::Surface.css_var(),
                ThemeColor::Primary.css_var(),
            ),
            Typography {
                tag: TypographyTag::P,
                class: "m-0 text-xs font-bold uppercase tracking-[0.12em]".to_string(),
                style: format!("color: {};", ThemeColor::Primary.css_var()),
                "{props.label}"
            }
            Typography {
                tag: TypographyTag::P,
                class: "mt-2 text-base font-semibold".to_string(),
                style: format!("color: {};", ThemeColor::Secondary.css_var()),
                "{props.value}"
            }
        }
    }
}
