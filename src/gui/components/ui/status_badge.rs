use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BadgeTone {
    Muted,
    Accent,
    Success,
    Warning,
}

#[component]
pub fn StatusBadge(
    children: Element,
    #[props(default = BadgeTone::Muted)] tone: BadgeTone,
    #[props(default = String::new())] class: String,
) -> Element {
    let tone_class = match tone {
        BadgeTone::Muted => "border-emerald-900/10 bg-white/80 text-emerald-900",
        BadgeTone::Accent => "border-emerald-600/30 bg-emerald-100 text-emerald-800",
        BadgeTone::Success => "border-emerald-600/25 bg-emerald-100 text-emerald-700",
        BadgeTone::Warning => "border-amber-300 bg-amber-50 text-amber-800",
    };

    rsx! {
        span {
            class: "inline-flex items-center gap-2 rounded-full border px-3 py-1.5 text-sm font-semibold {tone_class} {class}",
            {children}
        }
    }
}
