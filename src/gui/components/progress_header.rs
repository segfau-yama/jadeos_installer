use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Typography, TypographyTag};
use crate::gui::routes::{ordered_routes, route_index, route_label, Route};

#[component]
pub fn ProgressHeader(active_route: Route) -> Element {
    let navigator = use_navigator();
    let active_index = route_index(&active_route);

    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-3".to_string(),
            Typography {
                tag: TypographyTag::P,
                class: "m-0 text-xs font-bold uppercase tracking-[0.16em] text-emerald-900/65".to_string(),
                "Progress"
            }
            Flexbox {
                items: "items-center".to_string(),
                wrap: "flex-wrap".to_string(),
                gap: "gap-3".to_string(),
                class: "w-full".to_string(),
                for route in ordered_routes() {
                    {
                        let item_route = route.clone();
                        let active_route_for_click = active_route.clone();
                        let navigator = navigator.clone();
                        let index = route_index(&route);
                        let label = route_label(&route);
                        let is_selected = route == active_route;
                        let is_reached = index <= active_index;
                        let step_class = if is_selected {
                            "border-emerald-600/40 bg-emerald-100 text-emerald-800 shadow-[0_14px_34px_rgba(6,95,70,0.12)]"
                        } else if is_reached {
                            "border-emerald-900/10 bg-white text-emerald-900 hover:border-emerald-400/40 hover:bg-emerald-50"
                        } else {
                            "border-slate-200 bg-slate-50/80 text-slate-400"
                        };
                        let circle_class = if is_selected {
                            "bg-emerald-700 text-white"
                        } else if is_reached {
                            "bg-emerald-100 text-emerald-800"
                        } else {
                            "bg-slate-200 text-slate-500"
                        };

                        rsx! {
                            button {
                                key: "{label}",
                                r#type: "button",
                                disabled: !is_reached,
                                class: "inline-flex items-center gap-3 rounded-full border px-4 py-3 text-left text-sm font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-emerald-400/70 focus-visible:ring-offset-2 disabled:cursor-not-allowed {step_class}",
                                onclick: move |_| {
                                    if is_reached && item_route != active_route_for_click {
                                        navigator.push(item_route.clone());
                                    }
                                },
                                span {
                                    class: "inline-flex h-7 w-7 items-center justify-center rounded-full text-xs font-bold {circle_class}",
                                    "{index + 1}"
                                }
                                span { "{label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
