use dioxus::prelude::*;

use crate::gui::components::{
    Card, CardBody, Container, ErrorBanner, Flexbox, ProgressHeader, Typography, TypographyTag,
};
use crate::gui::controller::clear_error;
use crate::gui::presentation::{APP_SUBTITLE, APP_TITLE};
use crate::gui::routes::{guard_route, Route};
use crate::gui::state::use_installer_state;

#[component]
pub fn AppShell() -> Element {
    let state = use_installer_state();
    let snapshot = state();
    let current_route = use_route::<Route>();
    let redirect_route = guard_route(&snapshot, current_route.clone());
    let display_route = if current_route == redirect_route {
        current_route.clone()
    } else {
        redirect_route.clone()
    };
    let navigator = use_navigator();
    let dismiss_state = state;
    let current_route_for_guard = current_route.clone();
    let redirect_route_for_guard = redirect_route.clone();

    use_effect(move || {
        if current_route_for_guard != redirect_route_for_guard {
            navigator.replace(redirect_route_for_guard.clone());
        }
    });

    rsx! {
        div {
            class: "min-h-screen px-4 py-8 sm:px-6 lg:px-8",
            Container {
                class: "max-w-6xl",
                Flexbox {
                    direction: "flex-col".to_string(),
                    gap: "gap-6".to_string(),
                    class: "leading-7".to_string(),
                    header {
                        class: "flex flex-col gap-3",
                        Typography {
                            tag: TypographyTag::P,
                            class: "m-0 text-xs font-bold uppercase tracking-[0.18em] text-emerald-900/65".to_string(),
                            "Safety-first installer"
                        }
                        Typography {
                            tag: TypographyTag::H1,
                            class: "m-0 max-w-4xl text-4xl font-bold tracking-[-0.04em] text-jade-950 sm:text-5xl lg:text-6xl".to_string(),
                            "{APP_TITLE}"
                        }
                        Typography {
                            tag: TypographyTag::P,
                            class: "m-0 max-w-3xl text-base text-emerald-900/70 sm:text-lg".to_string(),
                            "{APP_SUBTITLE}"
                        }
                    }
                    Card {
                        class: "w-full".to_string(),
                        CardBody {
                            class: "pt-6 sm:px-8 lg:px-10".to_string(),
                            ProgressHeader { active_route: display_route.clone() }
                            div {
                                class: "mt-2",
                                if display_route == current_route {
                                    Outlet::<Route> {}
                                } else {
                                    section {
                                        class: "py-12 text-emerald-900/65",
                                        "Redirecting to the first valid step..."
                                    }
                                }
                            }
                        }
                    }
                    ErrorBanner {
                        message: snapshot.ui.error_message.clone(),
                        on_dismiss: move |_| clear_error(dismiss_state),
                    }
                }
            }
        }
    }
}
