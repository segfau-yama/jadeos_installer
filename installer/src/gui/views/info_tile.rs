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
                theme.color(ThemeColor::Border), theme.color(ThemeColor::SurfaceAccent), props.class
            ),
            Typography {
                tag: TypographyTag::P,
                class: format!("m-0 text-xs font-bold uppercase tracking-[0.12em] {}", theme.color(ThemeColor::TextMuted)),
                "{props.label}"
            }
            Typography {
                tag: TypographyTag::P,
                class: format!("mt-2 text-base font-semibold {}", theme.color(ThemeColor::Text)),
                "{props.value}"
            }
        }
    }
}
