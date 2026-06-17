use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::{Col, Row, Theme, Typography, TypographyTag};

#[derive(PartialEq, Clone, Props)]
pub struct PageIntroProps {
    title: String,
    description: String,
    #[props(default = String::new())]
    class: String,
}

#[component]
pub fn PageIntro(props: PageIntroProps) -> Element {
    let theme = use_context::<Theme>();

    rsx! {
        Row {
            cols: "grid-cols-1 xl:grid-cols-2".to_string(),
            gap: "gap-3".to_string(),
            class: props.class,
            Col {
                Typography {
                    tag: TypographyTag::H2,
                    class: format!(
                        "m-0 text-3xl font-semibold tracking-[-0.03em] {} sm:text-4xl",
                        theme.text(ThemeColor::Text)
                    ),
                    "{props.title}"
                }
            }
            Col {
                Typography {
                    tag: TypographyTag::P,
                    class: format!(
                        "m-0 text-base leading-7 {} sm:text-lg",
                        theme.text(ThemeColor::Muted)
                    ),
                    "{props.description}"
                }
            }
        }
    }
}
