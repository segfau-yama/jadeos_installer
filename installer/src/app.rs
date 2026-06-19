use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Card, CardBody, Col, Container, Row, Theme, ThemeProvider};
use crate::gui::routes::Route;
use crate::gui::state::{
    InstallRuntime, InstallerConfig, InstallerContext, InstallerUiState, UserDraft,
};
use crate::gui::views::{AppFooter, AppHeader, ErrorBanner};

const TAILWIND_STYLES: &str = include_str!("../assets/tailwind.css");

pub fn app() -> Element {
    let theme = Theme::light();
    let config = use_signal(InstallerConfig::default);
    let user = use_signal(UserDraft::default);
    let ui = use_signal(InstallerUiState::default);
    let runtime = use_signal(InstallRuntime::default);
    #[cfg(not(target_arch = "wasm32"))]
    let install_progress = use_signal(|| None);
    let installer = InstallerContext {
        config,
        user,
        ui,
        runtime,
        #[cfg(not(target_arch = "wasm32"))]
        install_progress,
    };
    use_context_provider(|| installer);
    let ui_snapshot = (installer.ui)();
    let mut dismiss_ui = installer.ui;

    rsx! {
        document::Style {
            {TAILWIND_STYLES}
        }
        ThemeProvider {
            theme: theme,
            div {
                class: "min-h-screen px-4 py-8 sm:px-6 lg:px-8".to_string(),
                style: format!(
                    "background-color: {}; color: {};",
                    ThemeColor::BackGround.css_var(),
                    ThemeColor::Secondary.css_var(),
                ),
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
}
