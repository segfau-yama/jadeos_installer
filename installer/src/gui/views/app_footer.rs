use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Col, Row, Theme, Typography, TypographyTag};

#[component]
pub fn AppFooter() -> Element {
    let theme = use_context::<Theme>();

    rsx! {
        footer {
            Row {
                cols: "grid-cols-1 xl:grid-cols-2".to_string(),
                gap: "gap-2".to_string(),
                class: format!(
                    "border-t {} pt-4 {}",
                    theme.border(ThemeColor::Surface), theme.text(ThemeColor::Muted)
                ),
                Col {
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0 text-xs font-bold uppercase tracking-[0.16em]".to_string(),
                        "JadeOS Installer"
                    }
                }
                Col {
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0 text-sm leading-6 xl:text-right".to_string(),
                        "Desktop-first installer shell with a web-safe demo path."
                    }
                }
            }
        }
    }
}
