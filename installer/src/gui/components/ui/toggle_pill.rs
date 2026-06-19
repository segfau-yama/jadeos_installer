use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

#[component]
pub fn TogglePill(
    onpress: EventHandler<MouseEvent>,
    children: Element,
    #[props(default = false)] selected: bool,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
) -> Element {
    let control_style = if selected {
        format!(
            "background-color: {}; border-color: {}; color: white; outline-color: {};",
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
        )
    } else {
        format!(
            "background-color: {}; border-color: color-mix(in srgb, {} 22%, transparent); color: {}; outline-color: {};",
            ThemeColor::Surface.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
        )
    };
    let dot_style = if selected {
        "background-color: white;".to_string()
    } else {
        format!(
            "background-color: {}; opacity: 0.35;",
            ThemeColor::Primary.css_var()
        )
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            class: format!(
                "inline-flex items-center gap-3 rounded-full border px-4 py-2 text-sm font-semibold transition-opacity duration-150 hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 disabled:cursor-not-allowed disabled:opacity-60 {}",
                class
            ),
            style: "{control_style}",
            onclick: move |event| {
                if !disabled {
                    onpress.call(event);
                }
            },
            span {
                aria_hidden: "true",
                class: "inline-flex h-2.5 w-2.5 rounded-full",
                style: "{dot_style}",
            }
            {children}
        }
    }
}
