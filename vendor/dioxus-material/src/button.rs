use crate::use_theme;
use dioxus::prelude::*;

/// Filled button component.
///
/// Buttons let people take action and make choices with one tap.
///
/// [material.io](https://m3.material.io/components/buttons)
///
/// ## Panics
/// This component requires access to a [`Theme`](crate::Theme).
///
/// ## Examples
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_material::{Button, Theme};
///
/// fn app() -> Element {
///    rsx!(Theme {
///         Button { onpress: |_| log::info!("clicked!"), "Click me!" } }
///    )
/// }
/// ```
#[component]
pub fn Button(
    /// Handler for button press events.
    onpress: EventHandler<MouseEvent>,

    /// Label child element.
    children: Element,

    /// Disable the button (optional).
    #[props(default = false)]
    disabled: bool,

    /// Background color of the container (optional).
    background_color: Option<String>,

    /// Border radius of the container (optional).
    border_radius: Option<String>,

    /// Height of the container (optional).
    height: Option<String>,
) -> Element {
    let theme = use_theme();
    let content_color = if disabled {
        theme.on_surface_variant.clone()
    } else {
        theme.on_primary_color.clone()
    };
    let background_color = background_color.as_deref().unwrap_or(&theme.primary_color);
    let border_radius = border_radius
        .as_deref()
        .unwrap_or(&theme.border_radius_medium);
    let height = height.as_deref().unwrap_or("50px");
    let background = if disabled {
        theme.surface_variant_color.as_ref()
    } else {
        background_color
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            display: "inline-flex",
            align_items: "center",
            justify_content: "center",
            height: "{height}",
            padding: "0 24px",
            border: "none",
            color: "{content_color}",
            background: "{background}",
            border_radius: "{border_radius}",
            overflow: "hidden",
            box_shadow: if disabled { "none" } else { "0 16px 30px rgba(0, 110, 78, 0.22)" },
            cursor: if disabled { "not-allowed" } else { "pointer" },
            font_family: "system-ui, sans-serif",
            font_weight: "600",
            letter_spacing: "0.02em",
            transition: "transform 120ms ease, box-shadow 120ms ease, background 120ms ease",
            onclick: move |event| {
                if !disabled {
                    onpress.call(event);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn TextButton(
    /// Handler for button press events.
    onpress: EventHandler<MouseEvent>,

    /// Label child element.
    children: Element,

    /// Disable the button (optional).
    #[props(default = false)]
    disabled: bool,

    /// Border radiusof the container (optional).
    border_radius: Option<String>,

    /// Text color (optional).
    color: Option<String>,

    /// Height of the container (optional).
    height: Option<String>,
) -> Element {
    let theme = use_theme();
    let color = if disabled {
        theme.on_surface_variant.as_ref()
    } else {
        color.as_deref().unwrap_or(&theme.primary_color)
    };
    let border_radius = border_radius
        .as_deref()
        .unwrap_or(&theme.border_radius_medium);
    let height = height.as_deref().unwrap_or("40px");

    rsx! {
        button {
            r#type: "button",
            disabled,
            display: "inline-flex",
            align_items: "center",
            justify_content: "center",
            height: "{height}",
            padding: "0 16px",
            border: "none",
            background: "transparent",
            border_radius: "{border_radius}",
            color: "{color}",
            font_weight: "600",
            overflow: "hidden",
            cursor: if disabled { "not-allowed" } else { "pointer" },
            font_family: "system-ui, sans-serif",
            letter_spacing: "0.02em",
            transition: "background 120ms ease, color 120ms ease",
            onclick: move |event| {
                if !disabled {
                    onpress.call(event);
                }
            },
            {children}
        }
    }
}
