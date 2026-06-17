use jade_installer::gui::routes::{next_route, ordered_routes, previous_route, route_index, Route};

#[test]
fn route_order_matches_wizard_progression() {
    let routes = ordered_routes();

    assert_eq!(
        routes,
        &[
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

#[test]
fn next_route_moves_forward_one_step() {
    assert_eq!(next_route(&Route::Welcome {}), Some(Route::User {}));
    assert_eq!(next_route(&Route::User {}), Some(Route::Disk {}));
    assert_eq!(next_route(&Route::Disk {}), Some(Route::Summary {}));
    assert_eq!(next_route(&Route::Summary {}), Some(Route::Install {}));
    assert_eq!(next_route(&Route::Install {}), None);
}

#[test]
fn previous_route_moves_backward_one_step() {
    assert_eq!(previous_route(&Route::Welcome {}), None);
    assert_eq!(previous_route(&Route::User {}), Some(Route::Welcome {}));
    assert_eq!(previous_route(&Route::Disk {}), Some(Route::User {}));
    assert_eq!(previous_route(&Route::Summary {}), Some(Route::Disk {}));
    assert_eq!(previous_route(&Route::Install {}), Some(Route::Summary {}));
}
