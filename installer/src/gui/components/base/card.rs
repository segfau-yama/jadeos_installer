use dioxus::prelude::*;

use super::Flexbox;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    #[props(default = String::new())]
    class: String,
    #[props(default = String::new())]
    style: String,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            class: format!(
                "relative overflow-hidden rounded-[2rem] border backdrop-blur-xl shadow-none {}",
                props.class
            ),
            style: format!(
                "background-color: {}; border-color: color-mix(in srgb, {} 22%, transparent); {}",
                crate::gui::components::ThemeColor::Surface.css_var(),
                crate::gui::components::ThemeColor::Secondary.css_var(),
                props.style
            ),
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardHeaderProps {
    #[props(default = String::new())]
    class: String,
    #[props(default = String::new())]
    style: String,
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-2".to_string(),
            class: format!("relative px-6 pt-6 {}", props.class),
            style: props.style,
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardBodyProps {
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-6".to_string(),
            class: format!("px-6 pb-6 {}", props.class),
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardFooterProps {
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    rsx! {
        Flexbox {
            wrap: "flex-wrap".to_string(),
            items: "items-center".to_string(),
            gap: "gap-3".to_string(),
            class: format!("mt-auto px-6 pb-6 {}", props.class),
            {props.children}
        }
    }
}
