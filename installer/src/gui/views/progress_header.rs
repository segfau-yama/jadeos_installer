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
            theme.colors.border_accent,
            theme.colors.accent_surface,
            theme.colors.accent_fg,
            theme.shadow.interactive
        )
    } else if is_reached {
        format!(
            "{} {} {} {}",
            theme.colors.border_subtle,
            theme.colors.surface_base,
            theme.colors.accent_fg,
            [
                theme.colors.border_accent_hover,
                theme.colors.surface_neutral_hover,
            ]
            .join(" ")
        )
    } else {
        format!(
            "{} {} {}",
            theme.colors.border_neutral, theme.colors.surface_neutral, theme.colors.text_disabled
        )
    };
    let circle_class = if is_selected {
        format!("{} {}", theme.colors.accent_bg, theme.colors.text_inverse)
    } else if is_reached {
        format!("{} {}", theme.colors.accent_surface, theme.colors.accent_fg)
    } else {
        format!(
            "{} {}",
            theme.colors.surface_disabled, theme.colors.text_disabled
        )
    };

    rsx! {
        button {
            r#type: "button",
            disabled: !is_reached,
            class: format!(
                "inline-flex items-center gap-3 {} border px-4 py-3 text-left text-sm font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 {} focus-visible:ring-offset-2 disabled:cursor-not-allowed {}",
                theme.shape.pill_radius,
                theme.colors.focus_visible_ring,
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
                    theme.shape.pill_radius,
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
                    theme.colors.text_muted
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
