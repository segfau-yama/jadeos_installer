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
            class: "rounded-[1.5rem] border border-emerald-900/10 bg-emerald-50/75 px-5 py-4 {props.class}",
            Typography {
                tag: TypographyTag::P,
                class: "m-0 text-xs font-bold uppercase tracking-[0.12em] text-emerald-900/65".to_string(),
                "{props.label}"
            }
            Typography {
                tag: TypographyTag::P,
                class: "mt-2 text-base font-semibold text-jade-950".to_string(),
                "{props.value}"
            }
        }
    }
}
