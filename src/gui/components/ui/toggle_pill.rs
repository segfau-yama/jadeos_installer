use dioxus::prelude::*;

#[component]
pub fn TogglePill(
    onpress: EventHandler<MouseEvent>,
    children: Element,
    #[props(default = false)] selected: bool,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
) -> Element {
    let selected_class = if selected {
        "border-emerald-600/40 bg-emerald-100 text-emerald-800"
    } else {
        "border-emerald-900/10 bg-white/80 text-emerald-900"
    };
    let disabled_class = if disabled {
        "cursor-not-allowed opacity-60"
    } else {
        "cursor-pointer hover:border-emerald-400/40 hover:bg-emerald-50"
    };
    let dot_class = if selected {
        "bg-emerald-700"
    } else {
        "bg-emerald-300"
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            class: "inline-flex items-center gap-3 rounded-full border px-4 py-2 text-sm font-semibold transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-emerald-400/70 focus-visible:ring-offset-2 {selected_class} {disabled_class} {class}",
            onclick: move |event| {
                if !disabled {
                    onpress.call(event);
                }
            },
            span {
                aria_hidden: "true",
                class: "inline-flex h-2.5 w-2.5 rounded-full {dot_class}",
            }
            {children}
        }
    }
}
