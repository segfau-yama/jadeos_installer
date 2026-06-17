use crate::gui::components::{ThemeColor, ThemeRadius};
use dioxus::prelude::*;

use crate::gui::components::Theme;

#[component]
pub fn TogglePill(
    onpress: EventHandler<MouseEvent>,
    children: Element,
    #[props(default = false)] selected: bool,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
) -> Element {
    let theme = use_context::<Theme>();
    let selected_class = if selected {
        format!(
            "{} {} {}",
            theme.color(ThemeColor::BorderAccent),
            theme.color(ThemeColor::SurfaceAccent),
            theme.color(ThemeColor::TextAccent)
        )
    } else {
        format!(
            "{} {} {}",
            theme.color(ThemeColor::Border),
            theme.color(ThemeColor::Surface),
            theme.color(ThemeColor::TextAccent)
        )
    };
    let disabled_class = if disabled {
        "cursor-not-allowed opacity-60".to_string()
    } else {
        format!(
            "cursor-pointer {} {}",
            theme.color(ThemeColor::BorderHover),
            theme.color(ThemeColor::SurfaceHover)
        )
    };
    let dot_class = if selected {
        theme.color(ThemeColor::Accent)
    } else {
        theme.color(ThemeColor::AccentSoft)
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            class: format!(
                "inline-flex items-center gap-3 {} border px-4 py-2 text-sm font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 {} focus-visible:ring-offset-2 {} {} {}",
                theme.radius(ThemeRadius::Pill),
                theme.color(ThemeColor::FocusVisible),
                selected_class,
                disabled_class,
                class
            ),
            onclick: move |event| {
                if !disabled {
                    onpress.call(event);
                }
            },
            span {
                aria_hidden: "true",
                class: format!("inline-flex h-2.5 w-2.5 rounded-full {}", dot_class),
            }
            {children}
        }
    }
}
