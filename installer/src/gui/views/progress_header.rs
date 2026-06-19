use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Typography, TypographyTag};
use crate::gui::routes::{ordered_routes, route_index, Route};

#[derive(PartialEq, Clone, Props)]
struct ProgressStepProps {
    route: Route,
    index: usize,
    label: String,
    is_selected: bool,
    is_reached: bool,
}

#[component]
fn ProgressStep(props: ProgressStepProps) -> Element {
    let ProgressStepProps {
        route,
        index,
        label,
        is_selected,
        is_reached,
    } = props;
    let navigator = use_navigator();
    let step_style = if is_selected {
        format!(
            "background-color: {}; border-color: {}; color: white; outline-color: {};",
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
        )
    } else if is_reached {
        format!(
            "background-color: color-mix(in srgb, {} 10%, {}); border-color: color-mix(in srgb, {} 24%, transparent); color: {}; outline-color: {};",
            ThemeColor::Primary.css_var(),
            ThemeColor::Surface.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
        )
    } else {
        format!(
            "background-color: {}; border-color: color-mix(in srgb, {} 22%, transparent); color: {}; outline-color: {};",
            ThemeColor::Surface.css_var(),
            ThemeColor::Secondary.css_var(),
            ThemeColor::Secondary.css_var(),
            ThemeColor::Primary.css_var(),
        )
    };
    let circle_style = if is_selected {
        format!(
            "background-color: white; color: {};",
            ThemeColor::Primary.css_var(),
        )
    } else if is_reached {
        format!(
            "background-color: {}; color: white;",
            ThemeColor::Primary.css_var(),
        )
    } else {
        format!(
            "background-color: color-mix(in srgb, {} 18%, {}); color: {};",
            ThemeColor::Secondary.css_var(),
            ThemeColor::Surface.css_var(),
            ThemeColor::Secondary.css_var(),
        )
    };

    rsx! {
        button {
            r#type: "button",
            disabled: !is_reached,
            class: "inline-flex items-center gap-3 rounded-full border px-4 py-3 text-left text-sm font-semibold transition-opacity duration-150 hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 disabled:cursor-not-allowed disabled:opacity-60".to_string(),
            style: "{step_style}",
            onclick: move |_| {
                if is_reached && !is_selected {
                    navigator.push(route.clone());
                }
            },
            span {
                class: "inline-flex h-7 w-7 items-center justify-center rounded-full text-xs font-bold".to_string(),
                style: "{circle_style}",
                "{index + 1}"
            }
            span { "{label}" }
        }
    }
}

#[component]
pub fn ProgressHeader(active_route: Route) -> Element {
    let active_index = route_index(&active_route);

    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-3".to_string(),
            Typography {
                tag: TypographyTag::P,
                class: "m-0 text-xs font-bold uppercase tracking-[0.16em]".to_string(),
                style: format!("color: {};", ThemeColor::Secondary.css_var()),
                "Progress"
            }
            Flexbox {
                items: "items-center".to_string(),
                wrap: "flex-wrap".to_string(),
                gap: "gap-3".to_string(),
                for route in ordered_routes() {
                    ProgressStep {
                        key: "{route.label()}",
                        route: route.clone(),
                        index: route_index(&route),
                        label: route.label().to_string(),
                        is_selected: *route == active_route,
                        is_reached: route_index(&route) <= active_index,
                    }
                }
            }
        }
    }
}
