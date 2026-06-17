use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Theme, Typography, TypographyTag};

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
    let theme = use_context::<Theme>();
    let (panel_class, title_class) = match props.tone {
        PanelTone::Muted => (
            format!(
                "{} {} {}",
                theme.color(ThemeColor::Border),
                theme.color(ThemeColor::SurfaceAccent),
                theme.color(ThemeColor::TextMuted)
            ),
            theme.color(ThemeColor::TextAccent),
        ),
        PanelTone::Warning => (
            format!(
                "{} {} {}",
                theme.color(ThemeColor::WarningBorder),
                theme.color(ThemeColor::WarningBg),
                theme.color(ThemeColor::WarningText)
            ),
            theme.color(ThemeColor::WarningText),
        ),
    };

    rsx! {
        div {
            class: format!("rounded-[1.75rem] border px-5 py-4 {} {}", panel_class, props.class),
            Flexbox {
                direction: "flex-col".to_string(),
                gap: "gap-3".to_string(),
                if let Some(title) = props.title {
                    Typography {
                        tag: TypographyTag::P,
                        class: format!("m-0 text-sm font-semibold {}", title_class),
                        "{title}"
                    }
                }
                {props.children}
            }
        }
    }
}
