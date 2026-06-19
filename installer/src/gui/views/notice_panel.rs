use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Typography, TypographyTag};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PanelTone {
    Muted,
    Warning,
}

#[derive(PartialEq, Clone, Props)]
pub struct NoticePanelProps {
    #[props(default = PanelTone::Muted)]
    tone: PanelTone,
    #[props(default = None)]
    title: Option<String>,
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn NoticePanel(props: NoticePanelProps) -> Element {
    let (panel_class, title_class) = match props.tone {
        PanelTone::Muted => (
            format!(
                "background-color: color-mix(in srgb, {} 10%, {}); border-color: color-mix(in srgb, {} 22%, transparent); color: {};",
                ThemeColor::Primary.css_var(),
                ThemeColor::Surface.css_var(),
                ThemeColor::Primary.css_var(),
                ThemeColor::Secondary.css_var(),
            ),
            format!("color: {};", ThemeColor::Primary.css_var()),
        ),
        PanelTone::Warning => (
            format!(
                "background-color: color-mix(in srgb, {} 10%, {}); border-color: color-mix(in srgb, {} 22%, transparent); color: {};",
                ThemeColor::Warning.css_var(),
                ThemeColor::Surface.css_var(),
                ThemeColor::Warning.css_var(),
                ThemeColor::Warning.css_var(),
            ),
            format!("color: {};", ThemeColor::Warning.css_var()),
        ),
    };

    rsx! {
        div {
            class: format!("rounded-[1.75rem] border px-5 py-4 {}", props.class),
            style: panel_class,
            Flexbox {
                direction: "flex-col".to_string(),
                gap: "gap-3".to_string(),
                if let Some(title) = props.title {
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0 text-sm font-semibold".to_string(),
                        style: title_class,
                        "{title}"
                    }
                }
                {props.children}
            }
        }
    }
}
