use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
}

#[component]
pub fn UiButton(
    onpress: EventHandler<MouseEvent>,
    children: Element,
    #[props(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
) -> Element {
    let variant_style = match variant {
        ButtonVariant::Primary => format!(
            "background-color: {}; border-color: {}; color: white; outline-color: {};",
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
        ),
        ButtonVariant::Secondary => format!(
            "background-color: color-mix(in srgb, {} 10%, {}); border-color: color-mix(in srgb, {} 24%, transparent); color: {}; outline-color: {};",
            ThemeColor::Primary.css_var(),
            ThemeColor::Surface.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
        ),
        ButtonVariant::Ghost => format!(
            "background-color: transparent; border-color: transparent; color: {}; outline-color: {};",
            ThemeColor::Primary.css_var(),
            ThemeColor::Primary.css_var(),
        ),
    };
    let variant_class = match variant {
        ButtonVariant::Primary => "shadow-none",
        ButtonVariant::Secondary | ButtonVariant::Ghost => "",
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            class: format!(
                "inline-flex min-h-11 items-center justify-center rounded-full border px-5 text-sm font-semibold tracking-[0.01em] transition-opacity duration-150 hover:opacity-90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 disabled:cursor-not-allowed disabled:opacity-60 {} {}",
                variant_class,
                class
            ),
            style: "{variant_style}",
            onclick: move |event| {
                if !disabled {
                    onpress.call(event);
                }
            },
            {children}
        }
    }
}
