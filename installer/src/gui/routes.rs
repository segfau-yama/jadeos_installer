use dioxus::prelude::*;

use crate::gui::pages::{DiskPage, InstallPage, SummaryPage, UserPage, WelcomePage};
use crate::gui::state::InstallerState;
use crate::gui::validation::{disk_validation_errors, user_validation_errors};
use crate::gui::views::AppShell;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(AppShell)]
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

pub fn ordered_routes() -> [Route; 5] {
    [
        Route::Welcome {},
        Route::User {},
        Route::Disk {},
        Route::Summary {},
        Route::Install {},
    ]
}

pub fn route_label(route: &Route) -> &'static str {
    match route {
        Route::Welcome {} => "Welcome",
        Route::User {} => "User",
        Route::Disk {} => "Disk",
        Route::Summary {} => "Summary",
        Route::Install {} => "Install",
    }
}

pub fn route_index(route: &Route) -> usize {
    match route {
        Route::Welcome {} => 0,
        Route::User {} => 1,
        Route::Disk {} => 2,
        Route::Summary {} => 3,
        Route::Install {} => 4,
    }
}

pub fn guard_route(state: &InstallerState, requested: Route) -> Route {
    let user_ready = user_validation_errors(&state.config, &state.user).is_empty();
    let disk_ready = disk_validation_errors(&state.config).is_empty();

    match requested {
        Route::Welcome {} => Route::Welcome {},
        Route::User {} => Route::User {},
        Route::Disk {} => {
            if user_ready {
                Route::Disk {}
            } else {
                Route::User {}
            }
        }
        Route::Summary {} => {
            if !user_ready {
                Route::User {}
            } else if !disk_ready {
                Route::Disk {}
            } else {
                Route::Summary {}
            }
        }
        Route::Install {} => {
            if !user_ready {
                Route::User {}
            } else if !disk_ready {
                Route::Disk {}
            } else if state.runtime.install_plan.is_some() {
                Route::Install {}
            } else {
                Route::Summary {}
            }
        }
    }
}
