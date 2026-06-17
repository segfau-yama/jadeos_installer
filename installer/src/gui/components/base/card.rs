use dioxus::prelude::*;

use super::Flexbox;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    #[props(default = "bg-white/90".to_string())]
    color: String,
    #[props(default = "shadow".to_string())]
    shadow: String,
    #[props(default = "rounded-[2rem]".to_string())]
    rounded: String,
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            class: format!(
                "relative overflow-hidden border border-emerald-950/10 backdrop-blur-xl {} {} {} {}",
                props.color, props.shadow, props.rounded, props.class
            ),
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardHeaderProps {
    #[props(default = String::new())]
    color: String,
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-2".to_string(),
            class: format!("relative px-6 pt-6 {} {}", props.color, props.class),
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
