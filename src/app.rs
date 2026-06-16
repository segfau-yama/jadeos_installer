use dioxus::prelude::*;

use crate::gui::components::AppShell;
use crate::gui::controller::{clear_error, preview_plan, start_install};
use crate::gui::pages::{DiskPage, InstallPage, SummaryPage, UserPage, WelcomePage};
use crate::gui::state::{InstallerState, InstallerStep};

pub fn app() -> Element {
    let state = use_signal(InstallerState::default);
    let snapshot = state();
    let plan_preview = preview_plan(&snapshot);
    let shell_state = state;
    let install_state = state;

    let current_page = match snapshot.ui.step {
        InstallerStep::Welcome => rsx! { WelcomePage { state: state } },
        InstallerStep::User => rsx! { UserPage { state: state } },
        InstallerStep::Disk => rsx! { DiskPage { state: state } },
        InstallerStep::Summary => rsx! {
            SummaryPage {
                state: state,
                plan_preview: plan_preview,
                on_install: move |_| start_install(install_state),
            }
        },
        InstallerStep::Install => rsx! { InstallPage { state: state } },
    };

    rsx! {
        AppShell {
            step: snapshot.ui.step,
            error_message: snapshot.ui.error_message.clone(),
            on_dismiss_error: move |_| clear_error(shell_state),
            current_page: current_page,
        }
    }
}
