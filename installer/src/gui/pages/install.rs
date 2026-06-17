use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::api::install::InstallPhase;
use crate::gui::components::{
    BadgeTone, ButtonVariant, Flexbox, ProgressBar, StatusBadge, Theme, Typography, TypographyTag,
    UiButton,
};
use crate::gui::routes::{previous_route, Route};
use crate::gui::state::InstallerContext;
use crate::gui::views::{ActionRow, NoticePanel, PageIntro, PageSection};

#[component]
pub fn InstallPage() -> Element {
    let runtime = use_context::<InstallerContext>().runtime;
    let theme = use_context::<Theme>();
    let runtime_snapshot = runtime();
    let phases = install_phases();
    let navigator = use_navigator();
    let current_index = phases
        .iter()
        .position(|phase| *phase == runtime_snapshot.install_phase)
        .unwrap_or(0);
    let progress = (((current_index + 1) as f32 / phases.len() as f32) * 100.).round() as u8;

    rsx! {
        PageSection {
            PageIntro {
                title: "Install".to_string(),
                description: "The installer clones the NixOS configuration repository, generates a host module for the selected hostname, and runs nixos-install.".to_string(),
            }
            NoticePanel {
                class: "py-5".to_string(),
                Flexbox {
                    wrap: "flex-wrap".to_string(),
                    items: "items-center".to_string(),
                    gap: "gap-3".to_string(),
                    StatusBadge {
                        tone: BadgeTone::Accent,
                        "{runtime_snapshot.install_phase.label()}"
                    }
                    Typography {
                        tag: TypographyTag::P,
                        class: format!("m-0 text-sm font-medium {}", theme.color(ThemeColor::TextMuted)),
                        "Current phase"
                    }
                }
                Typography {
                    tag: TypographyTag::P,
                    class: format!("mt-3 text-base font-semibold {}", theme.color(ThemeColor::Text)),
                    {
                        runtime_snapshot
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
                        tone: if phase == runtime_snapshot.install_phase {
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
                class: format!("m-0 text-lg font-semibold {}", theme.color(ThemeColor::Text)),
                "Install log"
            }
            if runtime_snapshot.install_log.is_empty() {
                Typography {
                    tag: TypographyTag::P,
                    class: format!("m-0 text-base {}", theme.color(ThemeColor::TextMuted)),
                    "No log entries yet."
                }
            } else {
                pre {
                    class: format!(
                        "m-0 overflow-x-auto rounded-[1.5rem] {} px-5 py-4 text-sm leading-6 {}",
                        theme.color(ThemeColor::SurfaceInverse),
                        theme.color(ThemeColor::TextInverse)
                    ),
                    "{runtime_snapshot.install_log.join(\"\\n\")}"
                }
            }
            ActionRow {
                UiButton {
                    variant: ButtonVariant::Ghost,
                    onpress: move |_: MouseEvent| {
                        if let Some(route) = previous_route(&Route::Install {}) {
                            navigator.push(route);
                        }
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

fn install_phases() -> [InstallPhase; 8] {
    [
        InstallPhase::Validate,
        InstallPhase::Partition,
        InstallPhase::Format,
        InstallPhase::Mount,
        InstallPhase::GenerateConfig,
        InstallPhase::InstallSystem,
        InstallPhase::SetPassword,
        InstallPhase::Finish,
    ]
}
