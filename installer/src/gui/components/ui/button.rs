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
            theme.fill(ThemeColor::Accent),
            theme.text(ThemeColor::Inverse),
            theme.shadow(ThemeShadow::Interactive),
            theme.hover_fill(ThemeColor::Accent),
            theme.border(ThemeColor::Surface),
            theme.bg(ThemeColor::Muted),
            theme.text(ThemeColor::Muted),
            theme.shadow(ThemeShadow::None)
        ),
        ButtonVariant::Secondary => format!(
            "border {} {} {} {} disabled:{} disabled:{} disabled:{}",
            theme.border(ThemeColor::Surface),
            theme.bg(ThemeColor::Accent),
            theme.text(ThemeColor::Accent),
            theme.hover_bg(ThemeColor::Surface),
            theme.border(ThemeColor::Surface),
            theme.bg(ThemeColor::Muted),
            theme.text(ThemeColor::Muted)
        ),
        ButtonVariant::Ghost => format!(
            "border border-transparent bg-transparent {} {} disabled:bg-transparent disabled:{}",
            theme.text(ThemeColor::Accent),
            theme.hover_bg(ThemeColor::Surface),
            theme.text(ThemeColor::Muted)
        ),
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            class: format!(
                "inline-flex min-h-11 items-center justify-center {} px-5 text-sm font-semibold tracking-[0.01em] transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 {} focus-visible:ring-offset-2 disabled:cursor-not-allowed {} {}",
                theme.radius(ThemeRadius::Pill),
                theme.focus_visible(ThemeColor::Accent),
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
