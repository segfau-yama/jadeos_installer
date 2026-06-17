use dioxus::prelude::*;

use crate::gui::components::{Card, CardBody, Col, Container, Row};
use crate::gui::routes::Route;
use crate::gui::state::{InstallRuntime, InstallerConfig, InstallerUiState, UserDraft};
use crate::gui::views::{AppFooter, AppHeader, ErrorBanner};

const TAILWIND_STYLES: &str = include_str!("../assets/tailwind.css");

pub fn app() -> Element {
    let config = use_signal(InstallerConfig::default);
    let user = use_signal(UserDraft::default);
    let ui = use_signal(InstallerUiState::default);
    let runtime = use_signal(InstallRuntime::default);
    use_context_provider(|| config);
    use_context_provider(|| user);
    use_context_provider(|| ui);
    use_context_provider(|| runtime);
    let ui_snapshot = use_context::<Signal<InstallerUiState>>()();
    let mut dismiss_ui = ui;

    rsx! {
        document::Style {
            {TAILWIND_STYLES}
        }
        div {
            class: "min-h-screen px-4 py-8 sm:px-6 lg:px-8",
            Container {
                Row {
                    cols: "grid-cols-1".to_string(),
                    gap: "gap-6".to_string(),
                    class: "leading-7".to_string(),
                    Col {
                        AppHeader {}
                    }
                    Col {
                        div {
                            class: "mx-auto w-full",
                            style: "max-width: 1000px;",
                            Card {
                                CardBody {
                                    class: "pt-6 sm:px-8 lg:px-10".to_string(),
                                    Router::<Route> {}
                                }
                            }
                        }
                    }
                    Col {
                        AppFooter {}
                    }
                }
            }
            ErrorBanner {
                message: ui_snapshot.error_message.clone(),
                on_dismiss: move |_| dismiss_ui.write().error_message = None,
            }
        }
    }
}
