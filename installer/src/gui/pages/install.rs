use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::api::install::{InstallPhase, InstallationReport};
use crate::gui::components::{
    BadgeTone, ButtonVariant, Flexbox, ProgressBar, StatusBadge, Typography, TypographyTag,
    UiButton,
};
use crate::gui::routes::{previous_route, Route};
use crate::gui::state::{InstallRuntime, InstallerContext, InstallerUiState};
use crate::gui::views::{ActionRow, NoticePanel, PageIntro, PageSection};

#[component]
pub fn InstallPage() -> Element {
    let installer = use_context::<InstallerContext>();
    let mut runtime = installer.runtime;
    #[cfg(not(target_arch = "wasm32"))]
    let mut ui = installer.ui;
    #[cfg(not(target_arch = "wasm32"))]
    let mut install_progress = installer.install_progress;
    #[cfg(not(target_arch = "wasm32"))]
    use_future(move || async move {
        let Some(mut progress_rx) = install_progress.write().take() else {
            return;
        };

        while let Some(report) = progress_rx.recv().await {
            apply_install_report(&mut runtime, &mut ui, report);
        }
    });
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
                description: "The installer clones the NixOS configuration repository, generates host, user, and hardware modules for the selected machine, and runs nixos-install.".to_string(),
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
                        class: "m-0 text-sm font-medium".to_string(),
                        style: format!("color: {};", ThemeColor::Secondary.css_var()),
                        "Current phase"
                    }
                }
                Typography {
                    tag: TypographyTag::P,
                    class: "mt-3 text-base font-semibold".to_string(),
                    style: format!("color: {};", ThemeColor::Secondary.css_var()),
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
                class: "m-0 text-lg font-semibold".to_string(),
                style: format!("color: {};", ThemeColor::Secondary.css_var()),
                "Install log"
            }
            if runtime_snapshot.install_log.is_empty() {
                Typography {
                    tag: TypographyTag::P,
                    class: "m-0 text-base".to_string(),
                    style: format!("color: {};", ThemeColor::Secondary.css_var()),
                    "No log entries yet."
                }
            } else {
                pre {
                    class: "m-0 overflow-x-auto rounded-[1.5rem] border px-5 py-4 text-sm leading-6".to_string(),
                    style: format!(
                        "background-color: color-mix(in srgb, {} 8%, {}); border-color: color-mix(in srgb, {} 22%, transparent); color: {};",
                        ThemeColor::Secondary.css_var(),
                        ThemeColor::Surface.css_var(),
                        ThemeColor::Secondary.css_var(),
                        ThemeColor::Secondary.css_var(),
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

fn apply_install_report(
    runtime: &mut Signal<InstallRuntime>,
    ui: &mut Signal<InstallerUiState>,
    report: InstallationReport,
) {
    let mut runtime_state = runtime.write();
    runtime_state.install_phase = report.final_phase;
    runtime_state.current_command = report.current_command;
    runtime_state.install_log = report.log;
    ui.write().error_message = report.error_message;
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
