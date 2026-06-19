use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Col, Row, Typography, TypographyTag};

#[component]
pub fn AppFooter() -> Element {
    rsx! {
        footer {
            Row {
                cols: "grid-cols-1 xl:grid-cols-2".to_string(),
                gap: "gap-2".to_string(),
                class: "border-t pt-4".to_string(),
                style: format!(
                    "border-color: color-mix(in srgb, {} 22%, transparent); color: {};",
                    ThemeColor::Secondary.css_var(),
                    ThemeColor::Secondary.css_var(),
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
