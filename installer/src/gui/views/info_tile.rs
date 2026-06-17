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
                theme.colors.border_subtle, theme.colors.surface_accent, props.class
            ),
            Typography {
                tag: TypographyTag::P,
                class: format!("m-0 text-xs font-bold uppercase tracking-[0.12em] {}", theme.colors.text_muted),
                "{props.label}"
            }
            Typography {
                tag: TypographyTag::P,
                class: format!("mt-2 text-base font-semibold {}", theme.colors.text_primary),
                "{props.value}"
            }
        }
    }
}
