use dioxus::prelude::*;

#[component]
pub fn ValidationList(messages: Vec<String>) -> Element {
    if messages.is_empty() {
        return rsx! { Fragment {} };
    }

    rsx! {
        div {
            class: "flex flex-col gap-3",
            for message in messages {
                div {
                    key: "{message}",
                    class: "rounded-3xl border border-rose-200 bg-rose-50/90 px-4 py-3 text-sm font-medium text-rose-700",
                    "{message}"
                }
            }
        }
    }
}
