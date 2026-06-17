use crate::gui::components::{ThemeColor, ThemeRadius, ThemeShadow};
use dioxus::prelude::*;

use crate::gui::components::Theme;

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
    let theme = use_context::<Theme>();
    let variant_class = match variant {
        ButtonVariant::Primary => format!(
            "border border-transparent {} {} {} {} disabled:{} disabled:{} disabled:{} disabled:{}",
            theme.color(ThemeColor::Accent),
            theme.color(ThemeColor::TextInverse),
            theme.shadow(ThemeShadow::Interactive),
            theme.color(ThemeColor::AccentHover),
            theme.color(ThemeColor::Border),
            theme.color(ThemeColor::SurfaceDisabled),
            theme.color(ThemeColor::TextDisabled),
            theme.shadow(ThemeShadow::None)
        ),
        ButtonVariant::Secondary => format!(
            "border {} {} {} {} disabled:{} disabled:{} disabled:{}",
            theme.color(ThemeColor::Border),
            theme.color(ThemeColor::SurfaceAccent),
            theme.color(ThemeColor::TextAccent),
            theme.color(ThemeColor::SurfaceHover),
            theme.color(ThemeColor::Border),
            theme.color(ThemeColor::SurfaceDisabled),
            theme.color(ThemeColor::TextDisabled)
        ),
        ButtonVariant::Ghost => format!(
            "border border-transparent bg-transparent {} {} disabled:bg-transparent disabled:{}",
            theme.color(ThemeColor::TextAccent),
            theme.color(ThemeColor::SurfaceHover),
            theme.color(ThemeColor::TextDisabled)
        ),
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            class: format!(
                "inline-flex min-h-11 items-center justify-center {} px-5 text-sm font-semibold tracking-[0.01em] transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 {} focus-visible:ring-offset-2 disabled:cursor-not-allowed {} {}",
                theme.radius(ThemeRadius::Pill),
                theme.color(ThemeColor::FocusVisible),
                variant_class,
                class
            ),
            onclick: move |event| {
                if !disabled {
                    onpress.call(event);
                }
            },
            {children}
        }
    }
}
