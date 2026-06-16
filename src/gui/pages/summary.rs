use dioxus::prelude::*;
use dioxus_material::{Button, Chip, TextButton};

use crate::api::install::InstallPlan;
use crate::gui::components::{PlanCommandList, ValidationList};
use crate::gui::controller::navigate_to;
use crate::gui::presentation::{ERASE_CONFIRMATION_COPY, SUMMARY_FIXED_SETTINGS};
use crate::gui::state::{InstallerState, InstallerStep};
use crate::gui::validation::summary_validation_errors;

#[component]
pub fn SummaryPage(
    mut state: Signal<InstallerState>,
    plan_preview: Option<InstallPlan>,
    on_install: EventHandler<()>,
) -> Element {
    let snapshot = state();
    let validation_errors = summary_validation_errors(&snapshot.config, &snapshot.user);
    let install_ready = validation_errors.is_empty() && plan_preview.is_some();

    rsx! {
        section {
            style: "display: flex; flex-direction: column; gap: 20px;",
            div {
                style: "display: flex; flex-direction: column; gap: 10px;",
                h2 {
                    style: "margin: 0; color: #10201b; font-size: 2rem;",
                    "Summary"
                }
                p {
                    style: "margin: 0; color: #51625a; max-width: 60ch;",
                    "Review the chosen inputs and the fixed MVP settings before starting the install flow."
                }
            }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 14px;",
                div {
                    style: "padding: 16px; border-radius: 22px; background: #f6fbf8; border: 1px solid #dbe7e0;",
                    p { style: "margin: 0 0 6px; color: #51625a; font-size: 13px; text-transform: uppercase; letter-spacing: 0.08em;", "Hostname" }
                    p { style: "margin: 0; color: #10201b; font-weight: 600;", "{snapshot.config.hostname}" }
                }
                div {
                    style: "padding: 16px; border-radius: 22px; background: #f6fbf8; border: 1px solid #dbe7e0;",
                    p { style: "margin: 0 0 6px; color: #51625a; font-size: 13px; text-transform: uppercase; letter-spacing: 0.08em;", "Username" }
                    p { style: "margin: 0; color: #10201b; font-weight: 600;", "{snapshot.config.username}" }
                }
                div {
                    style: "padding: 16px; border-radius: 22px; background: #f6fbf8; border: 1px solid #dbe7e0;",
                    p { style: "margin: 0 0 6px; color: #51625a; font-size: 13px; text-transform: uppercase; letter-spacing: 0.08em;", "Target disk" }
                    p { style: "margin: 0; color: #10201b; font-weight: 600;", "{snapshot.config.target_disk}" }
                }
                for (label, value) in SUMMARY_FIXED_SETTINGS {
                    div {
                        key: "{label}",
                        style: "padding: 16px; border-radius: 22px; background: #f6fbf8; border: 1px solid #dbe7e0;",
                        p { style: "margin: 0 0 6px; color: #51625a; font-size: 13px; text-transform: uppercase; letter-spacing: 0.08em;", "{label}" }
                        p { style: "margin: 0; color: #10201b; font-weight: 600;", "{value}" }
                    }
                }
            }
            div {
                style: "display: flex; flex-direction: column; gap: 12px; padding: 18px; border-radius: 24px; background: #fff8ee; border: 1px solid #f3d3ac;",
                p {
                    style: "margin: 0; color: #8a3f09; font-weight: 700;",
                    "Destructive confirmation"
                }
                p {
                    style: "margin: 0; color: #51625a;",
                    "{ERASE_CONFIRMATION_COPY}"
                }
                div {
                    Chip {
                        is_selected: Some(snapshot.config.disk_erase_confirmed),
                        onclick: move |_| {
                        let mut draft = state.write();
                        draft.config.disk_erase_confirmed = !draft.config.disk_erase_confirmed;
                        draft.ui.error_message = None;
                        },
                        if snapshot.config.disk_erase_confirmed {
                            "Erase confirmed"
                        } else {
                            "Tap to confirm disk erase"
                        }
                    }
                }
            }
            ValidationList { messages: validation_errors }
            if let Some(plan) = plan_preview {
                PlanCommandList { title: "Planned commands".to_string(), commands: plan.commands }
            }
            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px; margin-top: 8px;",
                TextButton {
                    onpress: move |_| navigate_to(state, InstallerStep::Disk),
                    "Back"
                }
                Button {
                    disabled: !install_ready,
                    onpress: move |_| {
                        on_install.call(());
                    },
                    "Install"
                }
            }
        }
    }
}
