use crate::use_theme;
use dioxus::prelude::*;

/// Chip component.
///
/// Chips help people enter information, make selections, filter content, or trigger actions.
///
/// [material.io](https://m3.material.io/components/chips)
///
/// ## Panics
/// This component requires access to a [`Theme`](crate::Theme) and [`IconFont`](crate::IconFont).
///
/// ## Examples
/// ```rust
///
/// use dioxus::prelude::*;
/// use dioxus_material::{Chip, Theme, IconFont};
///
/// fn app() -> Element {
///     rsx!(Theme {
///         IconFont {}
///         div { display: "flex", gap: "10px",
///             Chip { onclick: |_| {}, "Asset chip" }
///             Chip { is_selected: true, onclick: |_| {}, "Asset chip" }
///         }
///     })
/// }
/// ```
#[component]
pub fn Chip(
    children: Element,
    is_selected: Option<bool>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme = use_theme();
    let is_selected = is_selected == Some(true);
    let (border_color, background, color) = if is_selected {
        (
            theme.primary_color.as_ref(),
            theme.secondary_container_color.as_ref(),
            theme.primary_color.as_ref(),
        )
    } else {
        (
            theme.outline_color.as_ref(),
            "transparent",
            theme.on_surface_color.as_ref(),
        )
    };

    rsx! {
        button {
            r#type: "button",
            display: "inline-flex",
            align_items: "center",
            gap: "8px",
            min_height: "36px",
            padding: "0 14px",
            border_radius: "{theme.border_radius_small}",
            font_family: "system-ui, sans-serif",
            font_size: "14px",
            font_weight: 500,
            border: "1px solid {border_color}",
            background,
            color: "{color}",
            cursor: "pointer",
            transition: "background 120ms ease, border-color 120ms ease, color 120ms ease",
            onclick: move |event| onclick.call(event),
            if is_selected {
                span {
                    aria_hidden: "true",
                    style: "display: inline-flex; width: 8px; height: 8px; border-radius: 999px; background: currentColor;",
                }
            }
            {children}
        }
    }
}
