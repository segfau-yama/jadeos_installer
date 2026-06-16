use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ProgressBarProps {
    percentage: u8,
    #[props(default = String::new())]
    rounded: String,
    #[props(default = String::new())]
    class: String,
    #[props(default = "from-emerald-600 to-teal-500".to_string())]
    bar_class: String,
}

#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    let percentage = props.percentage.min(100);

    rsx! {
        div {
            class: "w-full overflow-hidden bg-emerald-950/10 {props.rounded} {props.class}",
            div {
                class: "h-full rounded-full bg-gradient-to-r transition-[width] duration-300 {props.bar_class}",
                style: "width: {percentage}%;",
            }
        }
    }
}
