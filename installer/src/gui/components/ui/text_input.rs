use crate::gui::components::{ThemeColor, ThemeRadius};
use dioxus::prelude::*;

use crate::gui::components::{Flexbox, Theme};

#[derive(Clone, PartialEq, Props)]
pub struct TextInputProps {
    label: String,
    value: String,
    onchange: EventHandler<FormEvent>,
    #[props(default = None)]
    input_type: Option<String>,
    #[props(default = None)]
    supporting_text: Option<String>,
    #[props(default = None)]
    autocomplete: Option<String>,
    #[props(default = String::new())]
    class: String,
}

#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    let TextInputProps {
        label,
        value,
        onchange,
        input_type,
        supporting_text,
        autocomplete,
        class,
    } = props;
    let input_type = input_type.as_deref().unwrap_or("text");
    let autocomplete = autocomplete.as_deref().unwrap_or("off");
    let theme = use_context::<Theme>();

    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-2".to_string(),
            class: class,
            label {
                class: format!("text-xs font-bold uppercase tracking-[0.12em] {}", theme.color(ThemeColor::TextMuted)),
                "{label}"
            }
            input {
                r#type: "{input_type}",
                value: value.clone(),
                autocomplete: "{autocomplete}",
                class: format!(
                    "block {} border {} {} px-4 py-4 text-[0.98rem] {} shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_10px_24px_rgba(12,34,27,0.04)] outline-none transition {} focus:ring-4 {}",
                    theme.radius(ThemeRadius::Control),
                    theme.color(ThemeColor::Border),
                    theme.color(ThemeColor::Surface),
                    theme.color(ThemeColor::Text),
                    theme.color(ThemeColor::FocusBorder),
                    theme.color(ThemeColor::FocusRing)
                ),
                oninput: move |event| onchange.call(event),
            }
            if let Some(supporting_text) = supporting_text.clone() {
                p {
                    class: format!("m-0 text-sm leading-6 {}", theme.color(ThemeColor::TextMuted)),
                    "{supporting_text}"
                }
            }
        }
    }
}
