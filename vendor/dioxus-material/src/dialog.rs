use crate::use_theme;
use dioxus::prelude::*;

/// Dialogs provide important prompts in a user flow.
///
/// [material.io](https://m3.material.io/components/dialogs)
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_material::{Dialog, Theme};
///
/// fn app() -> Element {
///     rsx!(
///         Theme {
///             Dialog { is_visible: true, h1 { "Dialog" } }
///         }
///     )
/// }
/// ```
#[component]
pub fn Dialog(children: Element, is_visible: bool) -> Element {
    let theme = use_theme();

    rsx! {
        div {
            display: if is_visible { "block" } else { "none" },
            position: "fixed",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
            z_index: 100,
            background: "rgba(10, 18, 16, 0.38)",
            div {
                position: "absolute",
                top: "50%",
                left: "50%",
                transform: "translate(-50%, -50%)",
                border_radius: "{theme.border_radius_medium}",
                background: "{theme.surface_color}",
                box_shadow: "0 24px 60px rgba(15, 23, 42, 0.22)",
                width: "min(520px, calc(100vw - 32px))",
                padding: "24px",
                {children}
            }
        }
    }
}
