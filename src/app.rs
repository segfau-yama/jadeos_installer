use dioxus::prelude::*;

use crate::gui::routes::Route;
use crate::gui::state::InstallerState;

const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");

pub fn app() -> Element {
    let state = use_signal(InstallerState::default);
    use_context_provider(|| state);

    rsx! {
        document::Stylesheet {
            href: TAILWIND_STYLES
        }
        Router::<Route> {}
    }
}
