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
            theme.color(ThemeColor::Border),
            theme.color(ThemeColor::Surface),
            theme.color(ThemeColor::TextAccent)
        ),
        BadgeTone::Accent | BadgeTone::Success => format!(
            "{} {} {}",
            theme.color(ThemeColor::BorderAccent),
            theme.color(ThemeColor::SurfaceAccent),
            theme.color(ThemeColor::TextAccent)
        ),
        BadgeTone::Warning => format!(
            "{} {} {}",
            theme.color(ThemeColor::WarningBorder),
            theme.color(ThemeColor::WarningBg),
            theme.color(ThemeColor::WarningText)
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
