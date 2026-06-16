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
            gap: "10px",
            width,
            font_family: "system-ui, sans-serif",
            label {
                color: "{theme.on_surface_variant}",
                font_size: "{theme.label_small}px",
                font_weight: "600",
                letter_spacing: "0.03em",
                text_transform: "uppercase",
                "{label}"
            }
            input {
                r#type: "{input_type}",
                value: value.clone(),
                padding: "14px 16px",
                min_height: "52px",
                border: "1px solid {theme.outline_color}",
                border_radius: "{theme.border_radius_small}",
                outline: "none",
                background: "{background}",
                color: "{theme.on_surface_color}",
                font_size: "{theme.label_medium}px",
                line_height: "1.3",
                oninput: move |event| onchange.call(event)
            }
            if let Some(supporting_text) = supporting_text {
                p {
                    margin: 0,
                    color: "{theme.on_surface_variant}",
                    font_size: "13px",
                    "{supporting_text}"
                }
            }
        }
    }
}
