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
                class: "text-xs font-bold uppercase tracking-[0.12em] text-emerald-900/65",
                "{label}"
            }
            input {
                r#type: "{input_type}",
                value: value.clone(),
                autocomplete: "{autocomplete}",
                class: "block rounded-[1.35rem] border border-emerald-900/12 bg-white px-4 py-4 text-[0.98rem] text-emerald-950 shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_10px_24px_rgba(12,34,27,0.04)] outline-none transition focus:border-emerald-400/60 focus:ring-4 focus:ring-emerald-100",
                oninput: move |event| onchange.call(event),
            }
            if let Some(supporting_text) = supporting_text.clone() {
                p {
                    class: "m-0 text-sm leading-6 text-emerald-900/65",
                    "{supporting_text}"
                }
            }
        }
    }
}
