use dioxus::prelude::*;
use dioxus_material::{Dialog, TextButton};

#[component]
pub fn ErrorBanner(message: Option<String>, on_dismiss: EventHandler<()>) -> Element {
    match message {
        Some(message) => rsx! {
            Dialog {
                is_visible: true,
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    p {
                        style: "margin: 0; color: #8a3f09; font-size: 13px; font-weight: 700; letter-spacing: 0.12em; text-transform: uppercase;",
                        "Installer notice"
                    }
                    h2 {
                        style: "margin: 0; color: #15211b; font-size: 1.45rem;",
                        "Something needs attention"
                    }
                    p {
                        style: "margin: 0; color: #43554d;",
                        "{message}"
                    }
                    div {
                        style: "display: flex; justify-content: flex-end;",
                        TextButton {
                            onpress: move |_| on_dismiss.call(()),
                            "Close"
                        }
                    }
                }
            }
        },
        None => rsx! { Fragment {} },
    }
}
