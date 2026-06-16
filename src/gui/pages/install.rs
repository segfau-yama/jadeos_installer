use dioxus::prelude::*;
use dioxus_material::{Button, Chip, TextButton};

use crate::gui::components::PlanCommandList;
use crate::gui::controller::navigate_to;
use crate::gui::presentation::install_phases;
use crate::gui::state::{InstallerState, InstallerStep};

#[component]
pub fn InstallPage(state: Signal<InstallerState>) -> Element {
    let snapshot = state();
    let phases = install_phases();

    rsx! {
        section {
            style: "display: flex; flex-direction: column; gap: 20px;",
            div {
                style: "display: flex; flex-direction: column; gap: 10px;",
                h2 {
                    style: "margin: 0; color: #10201b; font-size: 2rem;",
                    "Install"
                }
                p {
                    style: "margin: 0; color: #51625a; max-width: 60ch;",
                    "This scaffold stops at plan reporting. The api layer already owns the install commands, but actual command execution is intentionally left for the next step."
                }
            }
            div {
                style: "display: flex; flex-direction: column; gap: 14px; padding: 18px; border-radius: 24px; border: 1px solid #dbe7e0; background: #f6fbf8;",
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 10px; align-items: center;",
                    Chip {
                        is_selected: Some(true),
                        onclick: move |_| {},
                        "{snapshot.runtime.install_phase.label()}"
                    }
                    p {
                        style: "margin: 0; color: #51625a;",
                        "Current phase"
                    }
                }
                p {
                    style: "margin: 0; color: #10201b; font-weight: 600;",
                    {
                        snapshot
                            .runtime
                            .current_command
                            .clone()
                            .unwrap_or_else(|| "No command is running in scaffold mode.".to_string())
                    }
                }
            }
            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px;",
                for phase in phases {
                    div {
                        key: "{phase.label()}",
                        Chip {
                            is_selected: Some(phase == snapshot.runtime.install_phase),
                            onclick: move |_| {},
                            "{phase.label()}"
                        }
                    }
                }
            }
            if let Some(plan) = snapshot.runtime.install_plan.clone() {
                PlanCommandList { title: "Install plan".to_string(), commands: plan.commands }
            }
            h3 {
                style: "margin: 0; color: #10201b; font-size: 1.15rem;",
                "Install log"
            }
            if snapshot.runtime.install_log.is_empty() {
                p {
                    style: "margin: 0; color: #51625a;",
                    "No log entries yet."
                }
            } else {
                pre {
                    style: "margin: 0; background: #13211c; color: #e8fff3; padding: 18px; border-radius: 22px; overflow-x: auto;",
                    "{snapshot.runtime.install_log.join(\"\\n\")}"
                }
            }
            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px; margin-top: 8px;",
                TextButton {
                    onpress: move |_| navigate_to(state, InstallerStep::Summary),
                    "Back to summary"
                }
                Button {
                    disabled: true,
                    onpress: move |_| {},
                    "Reboot (not wired yet)"
                }
            }
        }
    }
}
