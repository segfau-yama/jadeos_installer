use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

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
    let tone_style = match tone {
        BadgeTone::Muted => format!(
            "background-color: color-mix(in srgb, {} 12%, {}); border-color: color-mix(in srgb, {} 22%, transparent); color: {};",
            ThemeColor::Secondary.css_var(),
            ThemeColor::Surface.css_var(),
            ThemeColor::Secondary.css_var(),
            ThemeColor::Secondary.css_var(),
        ),
        BadgeTone::Accent => format!(
            "background-color: color-mix(in srgb, {} 12%, {}); border-color: color-mix(in srgb, {} 22%, transparent); color: {};",
            ThemeColor::Primary.css_var(),
            ThemeColor::Surface.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
        ),
        BadgeTone::Success => format!(
            "background-color: color-mix(in srgb, {} 12%, {}); border-color: color-mix(in srgb, {} 22%, transparent); color: {};",
            ThemeColor::Success.css_var(),
            ThemeColor::Surface.css_var(),
            ThemeColor::Success.css_var(),
            ThemeColor::Success.css_var(),
        ),
        BadgeTone::Warning => format!(
            "background-color: color-mix(in srgb, {} 12%, {}); border-color: color-mix(in srgb, {} 22%, transparent); color: {};",
            ThemeColor::Warning.css_var(),
            ThemeColor::Surface.css_var(),
            ThemeColor::Warning.css_var(),
            ThemeColor::Warning.css_var(),
        ),
    };

    rsx! {
        span {
            class: format!("inline-flex items-center gap-2 rounded-full border px-3 py-1.5 text-sm font-semibold {}", class),
            style: "{tone_style}",
            {children}
        }
    }
}
