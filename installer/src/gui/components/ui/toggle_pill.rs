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
            theme.border(ThemeColor::Accent),
            theme.bg(ThemeColor::Accent),
            theme.text(ThemeColor::Accent)
        )
    } else {
        format!(
            "{} {} {}",
            theme.border(ThemeColor::Surface),
            theme.bg(ThemeColor::Surface),
            theme.text(ThemeColor::Accent)
        )
    };
    let disabled_class = if disabled {
        "cursor-not-allowed opacity-60".to_string()
    } else {
        format!(
            "cursor-pointer {} {}",
            theme.hover_border(ThemeColor::Accent),
            theme.hover_bg(ThemeColor::Surface)
        )
    };
    let dot_class = if selected {
        theme.fill(ThemeColor::Accent)
    } else {
        theme.soft_fill(ThemeColor::Accent)
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            class: format!(
                "inline-flex items-center gap-3 {} border px-4 py-2 text-sm font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 {} focus-visible:ring-offset-2 {} {} {}",
                theme.radius(ThemeRadius::Pill),
                theme.focus_visible(ThemeColor::Accent),
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
