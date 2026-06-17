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
            theme.colors.border_accent, theme.colors.accent_surface, theme.colors.accent_fg
        )
    } else {
        format!(
            "{} {} {}",
            theme.colors.border_subtle, theme.colors.surface_muted, theme.colors.accent_fg
        )
    };
    let disabled_class = if disabled {
        "cursor-not-allowed opacity-60".to_string()
    } else {
        format!(
            "cursor-pointer {} {}",
            theme.colors.border_accent_hover, theme.colors.surface_neutral_hover
        )
    };
    let dot_class = if selected {
        theme.colors.accent_bg
    } else {
        theme.colors.accent_fill_soft
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            class: format!(
                "inline-flex items-center gap-3 {} border px-4 py-2 text-sm font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 {} focus-visible:ring-offset-2 {} {} {}",
                theme.shape.pill_radius,
                theme.colors.focus_visible_ring,
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
