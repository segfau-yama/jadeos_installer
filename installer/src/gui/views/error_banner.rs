use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{
    ButtonVariant, Flexbox, ModalDialog, Theme, Typography, TypographyTag, UiButton,
};

#[component]
pub fn ErrorBanner(message: Option<String>, on_dismiss: EventHandler<()>) -> Element {
    let theme = use_context::<Theme>();

    match message {
        Some(message) => rsx! {
            ModalDialog {
                is_visible: true,
                Flexbox {
                    direction: "flex-col".to_string(),
                    gap: "gap-4".to_string(),
                    Typography {
                        tag: TypographyTag::P,
                        class: format!(
                            "m-0 text-xs font-bold uppercase tracking-[0.16em] {}",
                            theme.text(ThemeColor::Warning)
                        ),
                        "Installer notice"
                    }
                    Typography {
                        tag: TypographyTag::H2,
                        class: format!(
                            "m-0 text-2xl font-semibold tracking-[-0.03em] {}",
                            theme.text(ThemeColor::Text)
                        ),
                        "Something needs attention"
                    }
                    Typography {
                        tag: TypographyTag::P,
                        class: format!("m-0 text-base leading-7 {}", theme.text(ThemeColor::Muted)),
                        "{message}"
                    }
                    Flexbox {
                        justify: "justify-end".to_string(),
                        UiButton {
                            variant: ButtonVariant::Ghost,
                            onpress: move |_| on_dismiss.call(()),
                            "Close"
                        }
                    }
                }
            }
        },
        None => rsx! { Fragment {} },
    }
}
