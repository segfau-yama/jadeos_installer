use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    cols: String,
    #[props(default = String::new())]
    gap: String,
    #[props(default = String::new())]
    class: String,
    #[props(default = String::new())]
    style: String,
    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    rsx! {
        div {
            class: "grid {props.cols} {props.gap} {props.class}",
            style: "{props.style}",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ColProps {
    #[props(default = String::new())]
    class: String,
    #[props(default = String::new())]
    style: String,
    children: Element,
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        div {
            class: "{props.class}",
            style: "{props.style}",
            {props.children}
        }
    }
}
