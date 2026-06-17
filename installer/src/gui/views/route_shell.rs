use dioxus::prelude::*;

use crate::gui::routes::Route;

use super::ProgressHeader;

#[component]
pub fn RouteShell() -> Element {
    let current_route = use_route::<Route>();

    rsx! {
        ProgressHeader { active_route: current_route.clone() }
        div {
            class: "mt-2",
            Outlet::<Route> {}
        }
    }
}
