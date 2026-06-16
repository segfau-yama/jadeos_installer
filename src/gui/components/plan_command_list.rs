use dioxus::prelude::*;
use dioxus_material::Chip;

use crate::api::install::InstallCommand;

#[component]
pub fn PlanCommandList(title: String, commands: Vec<InstallCommand>) -> Element {
    if commands.is_empty() {
        return rsx! { Fragment {} };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 14px; margin-top: 20px;",
            h3 {
                style: "margin: 0; color: #10201b; font-size: 1.15rem;",
                "{title}"
            }
            div {
                style: "display: flex; flex-direction: column; gap: 12px;",
                for command in commands {
                    article {
                        key: "{command.description}",
                        style: "display: flex; flex-direction: column; gap: 10px; padding: 16px; border: 1px solid #dbe7e0; border-radius: 20px; background: #fcfefd;",
                        div {
                            style: "display: flex; flex-wrap: wrap; gap: 10px; align-items: center;",
                            Chip {
                                is_selected: Some(command.destructive),
                                onclick: move |_| {},
                                "{command.phase.label()}"
                            }
                            if command.destructive {
                                span {
                                    style: "color: #8a3f09; font-size: 13px; font-weight: 600;",
                                    "Destructive"
                                }
                            }
                        }
                        code {
                            style: "display: block; padding: 12px 14px; border-radius: 16px; background: #13211c; color: #ebfff5; font-family: ui-monospace, SFMono-Regular, monospace; overflow-x: auto;",
                            "{command.render_command()}"
                        }
                    }
                }
            }
        }
    }
}
