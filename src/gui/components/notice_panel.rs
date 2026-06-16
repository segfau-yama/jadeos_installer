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
            "border-emerald-900/10 bg-emerald-50/70 text-emerald-900/70",
            "text-emerald-900",
        ),
        PanelTone::Warning => (
            "border-amber-200 bg-amber-50/80 text-amber-800",
            "text-amber-800",
        ),
    };

    rsx! {
        div {
            class: "rounded-[1.75rem] border px-5 py-4 {panel_class} {props.class}",
            Flexbox {
                direction: "flex-col".to_string(),
                gap: "gap-3".to_string(),
                if let Some(title) = props.title {
                    Typography {
                        tag: TypographyTag::P,
                        class: format!("m-0 text-sm font-semibold {title_class}"),
                        "{title}"
                    }
                }
                {props.children}
            }
        }
    }
}
