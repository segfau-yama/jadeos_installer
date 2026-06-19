use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct FlexboxProps {
    #[props(default = String::new())]
    items: String,
    #[props(default = String::new())]
    justify: String,
    #[props(default = String::new())]
    gap: String,
    #[props(default = String::new())]
    direction: String,
    #[props(default = String::new())]
    wrap: String,
    #[props(default = String::new())]
    class: String,
    #[props(default = String::new())]
    style: String,
    children: Element,
}

#[component]
pub fn Flexbox(props: FlexboxProps) -> Element {
    rsx! {
        div {
            class: "flex {props.direction} {props.wrap} {props.items} {props.justify} {props.gap} {props.class}",
            style: "{props.style}",
            {props.children}
        }
    }
}
