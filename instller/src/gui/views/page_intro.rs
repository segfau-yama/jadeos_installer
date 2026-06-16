use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Typography, TypographyTag};

#[derive(PartialEq, Clone, Props)]
pub struct PageIntroProps {
    title: String,
    description: String,
    #[props(default = String::new())]
    class: String,
}

#[component]
pub fn PageIntro(props: PageIntroProps) -> Element {
    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-3".to_string(),
            class: props.class,
            Typography {
                tag: TypographyTag::H2,
                class: "m-0 text-3xl font-semibold tracking-[-0.03em] text-jade-950 sm:text-4xl".to_string(),
                "{props.title}"
            }
            Typography {
                tag: TypographyTag::P,
                class: "m-0 max-w-3xl text-base leading-7 text-emerald-900/70 sm:text-lg".to_string(),
                "{props.description}"
            }
        }
    }
}
