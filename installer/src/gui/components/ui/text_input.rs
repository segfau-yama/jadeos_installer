use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::gui::components::Flexbox;

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

    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-2".to_string(),
            class: class,
            label {
                class: "text-xs font-bold uppercase tracking-[0.12em]",
                style: "color: var(--theme-secondary);",
                "{label}"
            }
            input {
                r#type: "{input_type}",
                value: value.clone(),
                autocomplete: "{autocomplete}",
                class: "block rounded-[1.35rem] border px-4 py-4 text-[0.98rem] shadow-none transition-opacity focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2",
                style: format!(
                    "border-color: color-mix(in srgb, {} 22%, transparent); background-color: color-mix(in srgb, {} 10%, {}); color: {}; outline-color: {};",
                    ThemeColor::Secondary.css_var(),
                    ThemeColor::Secondary.css_var(),
                    ThemeColor::Surface.css_var(),
                    ThemeColor::Secondary.css_var(),
                    ThemeColor::Primary.css_var(),
                ),
                oninput: move |event| onchange.call(event),
            }
            if let Some(supporting_text) = supporting_text.clone() {
                p {
                    class: "m-0 text-sm leading-6",
                    style: "color: var(--theme-secondary);",
                    "{supporting_text}"
                }
            }
        }
    }
}
