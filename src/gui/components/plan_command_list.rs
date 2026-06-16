use dioxus::prelude::*;

use crate::api::install::InstallCommand;
use crate::gui::components::{
    BadgeTone, Card, CardBody, Flexbox, StatusBadge, Typography, TypographyTag,
};

#[component]
pub fn PlanCommandList(title: String, commands: Vec<InstallCommand>) -> Element {
    if commands.is_empty() {
        return rsx! { Fragment {} };
    }

    rsx! {
        Flexbox {
            direction: "flex-col".to_string(),
            gap: "gap-4".to_string(),
            class: "mt-5".to_string(),
            Typography {
                tag: TypographyTag::H3,
                class: "m-0 text-lg font-semibold text-jade-950".to_string(),
                "{title}"
            }
            Flexbox {
                direction: "flex-col".to_string(),
                gap: "gap-3".to_string(),
                for command in commands {
                    Card {
                        key: "{command.description}",
                        color: "bg-white".to_string(),
                        shadow: "shadow-none".to_string(),
                        rounded: "rounded-[1.5rem]".to_string(),
                        CardBody {
                            class: "gap-4".to_string(),
                            Flexbox {
                                wrap: "flex-wrap".to_string(),
                                items: "items-center".to_string(),
                                gap: "gap-3".to_string(),
                                StatusBadge {
                                    tone: if command.destructive {
                                        BadgeTone::Warning
                                    } else {
                                        BadgeTone::Accent
                                    },
                                    "{command.phase.label()}"
                                }
                                if command.destructive {
                                    Typography {
                                        tag: TypographyTag::Span,
                                        class: "text-sm font-semibold text-amber-700".to_string(),
                                        "Destructive"
                                    }
                                }
                            }
                            Typography {
                                tag: TypographyTag::Code,
                                class: "block overflow-x-auto rounded-[1.25rem] bg-jade-950 px-4 py-3 font-mono text-sm leading-6 text-emerald-50".to_string(),
                                "{command.render_command()}"
                            }
                        }
                    }
                }
            }
        }
    }
}
