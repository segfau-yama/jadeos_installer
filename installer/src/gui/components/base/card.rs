use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    #[props(default = "bg-white/90".to_string())]
    color: String,
    #[props(default = "shadow-[0_26px_70px_rgba(12,34,27,0.12)]".to_string())]
    shadow: String,
    #[props(default = "rounded-[2rem]".to_string())]
    rounded: String,
    #[props(default = String::new())]
    size: String,
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: "relative flex flex-col overflow-hidden border border-emerald-950/10 backdrop-blur-xl {props.color} {props.shadow} {props.rounded} {props.size} {props.class}",
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardHeaderProps {
    #[props(default = String::new())]
    color: String,
    #[props(default = String::new())]
    size: String,
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        div {
            class: "relative flex flex-col gap-2 px-6 pt-6 {props.color} {props.size} {props.class}",
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardBodyProps {
    #[props(default = String::new())]
    size: String,
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-6 px-6 pb-6 {props.size} {props.class}",
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardFooterProps {
    #[props(default = String::new())]
    size: String,
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    rsx! {
        div {
            class: "mt-auto flex flex-wrap items-center gap-3 px-6 pb-6 {props.size} {props.class}",
            {props.children}
        }
    }
}
