use dioxus::prelude::*;

use crate::gui::components::{Col, Flexbox, Row, Typography, TypographyTag};

const APP_TITLE: &str = "JadeOS Installer";
const APP_SUBTITLE: &str =
    "A safety-first JadeOS Live CD installer. It clones the NixOS configuration repository, generates a host module, and installs from that flake.";

#[component]
pub fn AppHeader() -> Element {
    rsx! {
        header {
            Row {
                cols: "grid-cols-1 xl:grid-cols-2".to_string(),
                gap: "gap-4".to_string(),
                Col {
                    Flexbox {
                        direction: "flex-col".to_string(),
                        gap: "gap-3".to_string(),
                        Typography {
                            tag: TypographyTag::P,
                            class: "m-0 text-xs font-bold uppercase tracking-[0.18em] text-emerald-900/65".to_string(),
                            "Safety-first installer"
                        }
                        Typography {
                            tag: TypographyTag::H1,
                            class: "m-0 text-4xl font-bold tracking-[-0.04em] text-jade-950 sm:text-5xl lg:text-6xl".to_string(),
                            "{APP_TITLE}"
                        }
                    }
                }
                Col {
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0 text-base text-emerald-900/70 sm:text-lg".to_string(),
                        "{APP_SUBTITLE}"
                    }
                }
            }
        }
    }
}
