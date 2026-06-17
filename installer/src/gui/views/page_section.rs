use dioxus::prelude::*;

use crate::gui::components::Flexbox;

#[derive(PartialEq, Clone, Props)]
pub struct PageSectionProps {
    #[props(default = String::new())]
    class: String,
    #[props(default = "gap-5".to_string())]
    gap: String,
    children: Element,
}

#[component]
pub fn PageSection(props: PageSectionProps) -> Element {
    rsx! {
        section {
            class: "{props.class}",
            Flexbox {
                direction: "flex-col".to_string(),
                gap: props.gap,
                {props.children}
            }
        }
    }
}
