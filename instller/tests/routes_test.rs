use jade_installer::api::install::preview_install_plan;
use jade_installer::gui::routes::{guard_route, ordered_routes, route_index, Route};
use jade_installer::gui::state::InstallerState;

#[test]
fn disk_route_redirects_to_user_when_user_input_is_incomplete() {
    let state = InstallerState::default();

    assert_eq!(guard_route(&state, Route::Disk {}), Route::User {});
}

#[test]
fn summary_route_redirects_to_disk_when_disk_is_missing() {
    let state = state_ready_for_disk();

    assert_eq!(guard_route(&state, Route::Summary {}), Route::Disk {});
}

#[test]
fn install_route_redirects_to_summary_when_plan_runtime_is_missing() {
    let state = state_ready_for_summary();

    assert_eq!(guard_route(&state, Route::Install {}), Route::Summary {});
}

#[test]
fn summary_route_allows_missing_erase_confirmation() {
    let state = state_ready_for_summary();

    assert_eq!(guard_route(&state, Route::Summary {}), Route::Summary {});
}

#[test]
fn route_order_matches_wizard_progression() {
    let routes = ordered_routes();

    assert_eq!(
        routes,
        [
            Route::Welcome {},
            Route::User {},
            Route::Disk {},
            Route::Summary {},
            Route::Install {},
        ]
    );

    for (index, route) in routes.iter().enumerate() {
        assert_eq!(route_index(route), index);
    }
}

fn state_ready_for_disk() -> InstallerState {
    let mut state = InstallerState::default();
    state.user.password = "jade-secret".to_string();
    state.user.password_confirmation = "jade-secret".to_string();
    state
}

fn state_ready_for_summary() -> InstallerState {
    let mut state = state_ready_for_disk();
    state.config.target_disk = "/dev/nvme0n1".to_string();
    state
}

#[allow(dead_code)]
fn state_ready_for_install() -> InstallerState {
    let mut state = state_ready_for_summary();
    state.runtime.install_plan = preview_install_plan(&state.config).ok();
    state
}
