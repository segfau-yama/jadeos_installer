use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
}

#[component]
pub fn UiButton(
    onpress: EventHandler<MouseEvent>,
    children: Element,
    #[props(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[props(default = false)] disabled: bool,
    #[props(default = String::new())] class: String,
) -> Element {
    let variant_class = match variant {
        ButtonVariant::Primary => {
            "border border-transparent bg-emerald-700 text-white shadow hover:bg-emerald-800 disabled:border-slate-200 disabled:bg-slate-200 disabled:text-slate-500 disabled:shadow-none"
        }
        ButtonVariant::Secondary => {
            "border border-emerald-900/10 bg-emerald-50 text-emerald-900 hover:bg-emerald-100 disabled:border-slate-200 disabled:bg-slate-100 disabled:text-slate-500"
        }
        ButtonVariant::Ghost => {
            "border border-transparent bg-transparent text-emerald-800 hover:bg-emerald-50 disabled:bg-transparent disabled:text-slate-400"
        }
    };

    rsx! {
        button {
            r#type: "button",
            disabled,
            class: "inline-flex min-h-11 items-center justify-center rounded-full px-5 text-sm font-semibold tracking-[0.01em] transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-emerald-400/70 focus-visible:ring-offset-2 disabled:cursor-not-allowed {variant_class} {class}",
            onclick: move |event| {
                if !disabled {
                    onpress.call(event);
                }
            },
            {children}
        }
    }
}
