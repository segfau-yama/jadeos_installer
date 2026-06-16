use dioxus::prelude::*;

use crate::gui::components::{
    ButtonVariant, Flexbox, ModalDialog, Typography, TypographyTag, UiButton,
};

#[component]
pub fn ErrorBanner(message: Option<String>, on_dismiss: EventHandler<()>) -> Element {
    match message {
        Some(message) => rsx! {
            ModalDialog {
                is_visible: true,
                Flexbox {
                    direction: "flex-col".to_string(),
                    gap: "gap-4".to_string(),
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0 text-xs font-bold uppercase tracking-[0.16em] text-amber-700".to_string(),
                        "Installer notice"
                    }
                    Typography {
                        tag: TypographyTag::H2,
                        class: "m-0 text-2xl font-semibold tracking-[-0.03em] text-jade-950".to_string(),
                        "Something needs attention"
                    }
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0 text-base leading-7 text-emerald-900/72".to_string(),
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
