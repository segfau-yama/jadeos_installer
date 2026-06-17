use crate::gui::components::{ThemeColor, ThemeRadius, ThemeShadow};
use dioxus::prelude::*;

use super::{theme::Theme, Flexbox};

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    #[props(default = String::new())]
    color: String,
    #[props(default = String::new())]
    shadow: String,
    #[props(default = String::new())]
    rounded: String,
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let theme = use_context::<Theme>();
    let color = if props.color.is_empty() {
        theme.color(ThemeColor::Surface)
    } else {
        props.color.as_str()
    };
    let shadow = if props.shadow.is_empty() {
        theme.shadow(ThemeShadow::Card)
    } else {
        props.shadow.as_str()
    };
    let rounded = if props.rounded.is_empty() {
        theme.radius(ThemeRadius::Card)
    } else {
        props.rounded.as_str()
    };

    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            class: format!(
                "relative overflow-hidden border backdrop-blur-xl {} {} {} {} {}",
                theme.color(ThemeColor::Border), color, shadow, rounded, props.class
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
