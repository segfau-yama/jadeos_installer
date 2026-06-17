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
            theme.colors.border_subtle, theme.colors.surface_muted, theme.colors.accent_fg
        ),
        BadgeTone::Accent | BadgeTone::Success => format!(
            "{} {} {}",
            theme.colors.border_accent, theme.colors.accent_surface, theme.colors.accent_fg
        ),
        BadgeTone::Warning => format!(
            "{} {} {}",
            theme.colors.warning_border, theme.colors.warning_bg, theme.colors.warning_fg
        ),
    };

    rsx! {
        span {
            class: format!(
                "inline-flex items-center gap-2 {} border px-3 py-1.5 text-sm font-semibold {} {}",
                theme.shape.pill_radius,
                tone_class,
                class
            ),
            {children}
        }
    }
}
