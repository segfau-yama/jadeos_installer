use dioxus::prelude::*;
use dioxus_material::Theme;

use crate::gui::components::ErrorBanner;
use crate::gui::components::ProgressHeader;
use crate::gui::presentation::{APP_SUBTITLE, APP_TITLE};
use crate::gui::state::InstallerStep;

#[component]
pub fn AppShell(
    step: InstallerStep,
    error_message: Option<String>,
    on_dismiss_error: EventHandler<()>,
    current_page: Element,
) -> Element {
    rsx! {
        Theme {
            primary_color: "#00695c",
            background_color: "#edf5f1",
            secondary_container_color: "#d8f0e4",
            surface_color: "#ffffff",
            surface_variant_color: "#f6fbf8",
            outline_color: "#adc3b7",
            on_surface_color: "#12211c",
            on_surface_variant: "#51625a",
            on_primary_color: "#ffffff",
            div {
                style: "min-height: 100vh; padding: 32px 20px 48px; background: radial-gradient(circle at top left, #d7f0e4 0%, #edf5f1 45%, #f8fbfa 100%);",
                div {
                    style: "max-width: 1080px; margin: 0 auto; display: flex; flex-direction: column; gap: 24px; line-height: 1.5;",
                    header {
                        style: "display: flex; flex-direction: column; gap: 10px;",
                        p {
                            style: "margin: 0; color: #456458; font-size: 13px; font-weight: 700; letter-spacing: 0.14em; text-transform: uppercase;",
                            "Safety-first installer"
                        }
                        h1 {
                            style: "margin: 0; font-size: clamp(2.2rem, 5vw, 3.5rem); line-height: 1.05; color: #10201b;",
                            "{APP_TITLE}"
                        }
                        p {
                            style: "margin: 0; max-width: 72ch; color: #51625a; font-size: 1.05rem;",
                            "{APP_SUBTITLE}"
                        }
                    }
                    div {
                        style: "border-radius: 32px; border: 1px solid rgba(173, 195, 183, 0.8); background: rgba(255, 255, 255, 0.9); box-shadow: 0 26px 70px rgba(12, 34, 27, 0.12); backdrop-filter: blur(18px); padding: 24px;",
                        ProgressHeader { step: step }
                        div {
                            style: "margin-top: 24px;",
                            {current_page}
                        }
                    }
                    ErrorBanner {
                        message: error_message,
                        on_dismiss: move |_| on_dismiss_error.call(()),
                    }
                }
            }
        }
    }
}
