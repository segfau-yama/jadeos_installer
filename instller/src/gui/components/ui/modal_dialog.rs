use dioxus::prelude::*;

#[component]
pub fn ModalDialog(is_visible: bool, children: Element) -> Element {
    if !is_visible {
        return rsx! { Fragment {} };
    }

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-emerald-950/40 px-4 py-8 backdrop-blur-sm",
            div {
                class: "w-full max-w-xl rounded-[2rem] border border-white/50 bg-white/95 p-6 shadow-[0_30px_90px_rgba(15,23,42,0.22)]",
                {children}
            }
        }
    }
}
