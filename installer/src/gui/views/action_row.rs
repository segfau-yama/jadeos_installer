use dioxus::prelude::*;

use crate::gui::components::Flexbox;

#[derive(PartialEq, Clone, Props)]
pub struct ActionRowProps {
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn ActionRow(props: ActionRowProps) -> Element {
    rsx! {
        Flexbox {
            wrap: "flex-wrap".to_string(),
            items: "items-center".to_string(),
            gap: "gap-3".to_string(),
            class: props.class,
            {props.children}
        }
    }
}
