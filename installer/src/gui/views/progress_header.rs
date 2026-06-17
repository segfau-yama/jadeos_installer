use crate::gui::components::{ThemeColor, ThemeRadius, ThemeShadow};
use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Theme, Typography, TypographyTag};
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
    let theme = use_context::<Theme>();
    let navigator = use_navigator();
    let step_class = if is_selected {
        format!(
            "{} {} {} {}",
            theme.color(ThemeColor::BorderAccent),
            theme.color(ThemeColor::SurfaceAccent),
            theme.color(ThemeColor::TextAccent),
            theme.shadow(ThemeShadow::Interactive)
        )
    } else if is_reached {
        format!(
            "{} {} {} {}",
            theme.color(ThemeColor::Border),
            theme.color(ThemeColor::Surface),
            theme.color(ThemeColor::TextAccent),
            [
                theme.color(ThemeColor::BorderHover),
                theme.color(ThemeColor::SurfaceHover),
            ]
            .join(" ")
        )
    } else {
        format!(
            "{} {} {}",
            theme.color(ThemeColor::Border),
            theme.color(ThemeColor::Surface),
            theme.color(ThemeColor::TextDisabled)
        )
    };
    let circle_class = if is_selected {
        format!(
            "{} {}",
            theme.color(ThemeColor::Accent),
            theme.color(ThemeColor::TextInverse)
        )
    } else if is_reached {
        format!(
            "{} {}",
            theme.color(ThemeColor::SurfaceAccent),
            theme.color(ThemeColor::TextAccent)
        )
    } else {
        format!(
            "{} {}",
            theme.color(ThemeColor::SurfaceDisabled),
            theme.color(ThemeColor::TextDisabled)
        )
    };

    rsx! {
        button {
            r#type: "button",
            disabled: !is_reached,
            class: format!(
                "inline-flex items-center gap-3 {} border px-4 py-3 text-left text-sm font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 {} focus-visible:ring-offset-2 disabled:cursor-not-allowed {}",
                theme.radius(ThemeRadius::Pill),
                theme.color(ThemeColor::FocusVisible),
                step_class
            ),
            onclick: move |_| {
                if is_reached && !is_selected {
                    navigator.push(route.clone());
                }
            },
            span {
                class: format!(
                    "inline-flex h-7 w-7 items-center justify-center {} text-xs font-bold {}",
                    theme.radius(ThemeRadius::Pill),
                    circle_class
                ),
                "{index + 1}"
            }
            span { "{label}" }
        }
    }
}

#[component]
pub fn ProgressHeader(active_route: Route) -> Element {
    let active_index = route_index(&active_route);
    let theme = use_context::<Theme>();

    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-3".to_string(),
            Typography {
                tag: TypographyTag::P,
                class: format!(
                    "m-0 text-xs font-bold uppercase tracking-[0.16em] {}",
                    theme.color(ThemeColor::TextMuted)
                ),
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
