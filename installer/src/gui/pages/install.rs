use dioxus::prelude::*;

use crate::gui::components::{
    BadgeTone, ButtonVariant, Flexbox, ProgressBar, StatusBadge, Typography, TypographyTag,
    UiButton,
};
use crate::gui::presentation::install_phases;
use crate::gui::routes::Route;
use crate::gui::state::use_installer_state;
use crate::gui::views::{ActionRow, NoticePanel, PageIntro, PageSection};

#[component]
pub fn InstallPage() -> Element {
    let state = use_installer_state();
    let snapshot = state();
    let phases = install_phases();
    let navigator = use_navigator();
    let current_index = phases
        .iter()
        .position(|phase| *phase == snapshot.runtime.install_phase)
        .unwrap_or(0);
    let progress = (((current_index + 1) as f32 / phases.len() as f32) * 100.).round() as u8;

    rsx! {
        PageSection {
            PageIntro {
                title: "Install".to_string(),
                description: "The installer clones the NixOS configuration repository, generates a host module for the selected hostname, and runs nixos-install.".to_string(),
            }
            NoticePanel {
                class: "bg-emerald-50/75 py-5".to_string(),
                Flexbox {
                    wrap: "flex-wrap".to_string(),
                    items: "items-center".to_string(),
                    gap: "gap-3".to_string(),
                    StatusBadge {
                        tone: BadgeTone::Accent,
                        "{snapshot.runtime.install_phase.label()}"
                    }
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0 text-sm font-medium text-emerald-900/70".to_string(),
                        "Current phase"
                    }
                }
                Typography {
                    tag: TypographyTag::P,
                    class: "mt-3 text-base font-semibold text-jade-950".to_string(),
                    {
                        snapshot
                            .runtime
                            .current_command
                            .clone()
                            .unwrap_or_else(|| "No command is running.".to_string())
                    }
                }
                div {
                    class: "mt-4",
                    ProgressBar {
                        percentage: progress,
                        rounded: "rounded-full".to_string(),
                        class: "h-3".to_string(),
                    }
                }
            }
            Flexbox {
                wrap: "flex-wrap".to_string(),
                gap: "gap-3".to_string(),
                for phase in phases {
                    StatusBadge {
                        key: "{phase.label()}",
                        tone: if phase == snapshot.runtime.install_phase {
                            BadgeTone::Accent
                        } else {
                            BadgeTone::Muted
                        },
                        "{phase.label()}"
                    }
                }
            }
            Typography {
                tag: TypographyTag::H3,
                class: "m-0 text-lg font-semibold text-jade-950".to_string(),
                "Install log"
            }
            if snapshot.runtime.install_log.is_empty() {
                Typography {
                    tag: TypographyTag::P,
                    class: "m-0 text-base text-emerald-900/70".to_string(),
                    "No log entries yet."
                }
            } else {
                pre {
                    class: "m-0 overflow-x-auto rounded-[1.5rem] bg-jade-950 px-5 py-4 text-sm leading-6 text-emerald-50",
                    "{snapshot.runtime.install_log.join(\"\\n\")}"
                }
            }
            ActionRow {
                UiButton {
                    variant: ButtonVariant::Ghost,
                    onpress: move |_: MouseEvent| {
                        navigator.push(Route::Summary {});
                    },
                    "Back to summary"
                }
                UiButton {
                    disabled: true,
                    onpress: move |_: MouseEvent| {},
                    "Reboot (not wired yet)"
                }
            }
        }
    }
}
