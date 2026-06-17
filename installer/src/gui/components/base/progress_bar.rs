use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::Theme;

#[derive(PartialEq, Clone, Props)]
pub struct ProgressBarProps {
    percentage: u8,
    #[props(default = String::new())]
    rounded: String,
    #[props(default = String::new())]
    class: String,
    #[props(default = String::new())]
    bar_class: String,
}

#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    let percentage = props.percentage.min(100);
    let theme = use_context::<Theme>();
    let bar_class = if props.bar_class.is_empty() {
        theme.gradient(ThemeColor::Accent).to_string()
    } else {
        props.bar_class.clone()
    };

    rsx! {
        div {
            class: format!(
                "w-full overflow-hidden {} {} {}",
                theme.track(ThemeColor::Accent),
                props.rounded,
                props.class
            ),
            div {
                class: format!(
                    "h-full rounded-full bg-gradient-to-r transition-[width] duration-300 {}",
                    bar_class
                ),
                style: "width: {percentage}%;",
            }
        }
    }
}
