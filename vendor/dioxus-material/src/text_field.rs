use crate::use_theme;
use dioxus::prelude::*;

/// Text field component.
///
/// Text fields let users enter text into a UI.
///
/// [material.io](https://m3.material.io/components/text-fields)
///
/// ## Panics
/// This component requires access to a [`Theme`](crate::Theme).
///
/// ## Examples
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_material::{TextField, Theme};
///
/// fn app() -> Element {
///     let mut value = use_signal(|| String::from("Filled"));
///     rsx!(
///         Theme {
///             TextField {
///                 label: "Text field",
///                 value: "{value}",
///                 onchange: move |event: FormEvent| value.set(event.value())
///             }
///         }
///     )
/// }
/// ```
#[component]
pub fn TextField(
    label: String,
    value: String,
    onchange: EventHandler<FormEvent>,
    input_type: Option<String>,
    supporting_text: Option<String>,
    background: Option<String>,
    width: Option<String>,
) -> Element {
    let theme = use_theme();
    let background = background
        .as_deref()
        .unwrap_or(&theme.surface_variant_color);
    let width = width.as_deref().unwrap_or("100%");
    let input_type = input_type.as_deref().unwrap_or("text");

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            gap: "8px",
            width,
            font_family: "system-ui, sans-serif",
            label {
                color: "{theme.on_surface_variant}",
                font_size: "{theme.label_small}px",
                font_weight: "700",
                letter_spacing: "0.08em",
                text_transform: "uppercase",
                "{label}"
            }
            input {
                r#type: "{input_type}",
                value: value.clone(),
                width: "100%",
                height: "58px",
                padding: "0 18px",
                box_sizing: "border-box",
                border: "1px solid {theme.outline_color}",
                border_radius: "18px",
                outline: "none",
                background: "{background}",
                color: "{theme.on_surface_color}",
                font_size: "{theme.label_medium}px",
                line_height: "1.3",
                box_shadow: "inset 0 1px 0 rgba(255, 255, 255, 0.75), 0 8px 20px rgba(18, 33, 28, 0.04)",
                transition: "border-color 120ms ease, box-shadow 120ms ease, background 120ms ease",
                oninput: move |event| onchange.call(event)
            }
            if let Some(supporting_text) = supporting_text {
                p {
                    margin: 0,
                    color: "{theme.on_surface_variant}",
                    font_size: "12px",
                    line_height: "1.45",
                    "{supporting_text}"
                }
            }
        }
    }
}
