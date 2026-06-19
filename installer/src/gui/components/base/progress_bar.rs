use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ProgressBarProps {
    percentage: u8,
    #[props(default = String::new())]
    rounded: String,
    #[props(default = String::new())]
    class: String,
    #[props(default = String::new())]
    style: String,
    #[props(default = String::new())]
    bar_style: String,
}

#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    let percentage = props.percentage.min(100);
    let track_style = if props.style.is_empty() {
        format!(
            "background-color: color-mix(in srgb, {} 12%, transparent);",
            crate::gui::components::ThemeColor::Primary.css_var()
        )
    } else {
        format!(
            "background-color: color-mix(in srgb, {} 12%, transparent); {}",
            crate::gui::components::ThemeColor::Primary.css_var(),
            props.style
        )
    };
    let bar_style = if props.bar_style.is_empty() {
        format!(
            "background-color: {}; width: {}%;",
            crate::gui::components::ThemeColor::Primary.css_var(),
            percentage
        )
    } else {
        format!("width: {}%; {}", percentage, props.bar_style)
    };

    rsx! {
        div {
            class: format!("w-full overflow-hidden {} {}", props.rounded, props.class),
            style: track_style,
            div {
                class: "h-full rounded-full transition-[width] duration-300",
                style: bar_style,
            }
        }
    }
}
