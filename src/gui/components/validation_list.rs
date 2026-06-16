use dioxus::prelude::*;

#[component]
pub fn ValidationList(messages: Vec<String>) -> Element {
    if messages.is_empty() {
        return rsx! { Fragment {} };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 10px;",
            for message in messages {
                div {
                    key: "{message}",
                    style: "padding: 12px 14px; border-radius: 18px; border: 1px solid #f1c4ba; background: #fff5f2; color: #9a2312;",
                    "{message}"
                }
            }
        }
    }
}
