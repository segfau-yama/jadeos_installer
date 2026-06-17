use dioxus::prelude::*;

use crate::gui::pages::{DiskPage, InstallPage, SummaryPage, UserPage, WelcomePage};
use crate::gui::views::RouteShell;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(RouteShell)]
        #[route("/", WelcomePage)]
        Welcome {},
        #[route("/user", UserPage)]
        User {},
        #[route("/disk", DiskPage)]
        Disk {},
        #[route("/summary", SummaryPage)]
        Summary {},
        #[route("/install", InstallPage)]
        Install {},
}

const ORDERED_ROUTES: [Route; 5] = [
    Route::Welcome {},
    Route::User {},
    Route::Disk {},
    Route::Summary {},
    Route::Install {},
];

impl Route {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Welcome {} => "Welcome",
            Self::User {} => "User",
            Self::Disk {} => "Disk",
            Self::Summary {} => "Summary",
            Self::Install {} => "Install",
        }
    }
}

pub fn ordered_routes() -> &'static [Route; 5] {
    &ORDERED_ROUTES
}

pub fn route_index(route: &Route) -> usize {
    ordered_routes()
        .iter()
        .position(|current| current == route)
        .unwrap_or(0)
}

pub fn next_route(route: &Route) -> Option<Route> {
    ordered_routes().get(route_index(route) + 1).cloned()
}

pub fn previous_route(route: &Route) -> Option<Route> {
    route_index(route)
        .checked_sub(1)
        .and_then(|index| ordered_routes().get(index).cloned())
}
