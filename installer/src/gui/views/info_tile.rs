use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Theme, Typography, TypographyTag};

#[derive(PartialEq, Clone, Props)]
pub struct InfoTileProps {
    label: String,
    value: String,
    #[props(default = String::new())]
    class: String,
}

#[component]
pub fn InfoTile(props: InfoTileProps) -> Element {
    let theme = use_context::<Theme>();

    rsx! {
        div {
            class: format!(
                "rounded-[1.5rem] border {} {} px-5 py-4 {}",
                theme.border(ThemeColor::Surface), theme.bg(ThemeColor::Accent), props.class
            ),
            Typography {
                tag: TypographyTag::P,
                class: format!("m-0 text-xs font-bold uppercase tracking-[0.12em] {}", theme.text(ThemeColor::Muted)),
                "{props.label}"
            }
            Typography {
                tag: TypographyTag::P,
                class: format!("mt-2 text-base font-semibold {}", theme.text(ThemeColor::Text)),
                "{props.value}"
            }
        }
    }
}
