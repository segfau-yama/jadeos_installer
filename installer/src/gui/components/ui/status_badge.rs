use crate::gui::components::{ThemeColor, ThemeRadius};
use dioxus::prelude::*;

use crate::gui::components::Theme;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BadgeTone {
    Muted,
    Accent,
    Success,
    Warning,
}

#[component]
pub fn StatusBadge(
    children: Element,
    #[props(default = BadgeTone::Muted)] tone: BadgeTone,
    #[props(default = String::new())] class: String,
) -> Element {
    let theme = use_context::<Theme>();
    let tone_class = match tone {
        BadgeTone::Muted => format!(
            "{} {} {}",
            theme.border(ThemeColor::Surface),
            theme.bg(ThemeColor::Surface),
            theme.text(ThemeColor::Accent)
        ),
        BadgeTone::Accent | BadgeTone::Success => format!(
            "{} {} {}",
            theme.border(ThemeColor::Accent),
            theme.bg(ThemeColor::Accent),
            theme.text(ThemeColor::Accent)
        ),
        BadgeTone::Warning => format!(
            "{} {} {}",
            theme.border(ThemeColor::Warning),
            theme.bg(ThemeColor::Warning),
            theme.text(ThemeColor::Warning)
        ),
    };

    rsx! {
        span {
            class: format!(
                "inline-flex items-center gap-2 {} border px-3 py-1.5 text-sm font-semibold {} {}",
                theme.radius(ThemeRadius::Pill),
                tone_class,
                class
            ),
            {children}
        }
    }
}
