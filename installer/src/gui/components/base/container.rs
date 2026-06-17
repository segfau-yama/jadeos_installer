use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ContainerProps {
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    rsx! {
        div {
            class: "mx-auto {props.class}",
            {props.children}
        }
    }
}
