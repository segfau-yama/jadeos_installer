use dioxus::prelude::*;

use crate::gui::components::{ButtonVariant, Flexbox, Typography, TypographyTag, UiButton};
use crate::gui::controller::{continue_from_disk, refresh_disks};
use crate::gui::routes::Route;
use crate::gui::state::use_installer_state;
use crate::gui::validation::disk_validation_errors;
use crate::gui::views::{ActionRow, DiskCard, NoticePanel, PageIntro, PageSection, ValidationList};

#[component]
pub fn DiskPage() -> Element {
    let mut state = use_installer_state();
    let snapshot = state();
    let validation_errors = disk_validation_errors(&snapshot.config);
    let selected_disk = snapshot.config.target_disk.clone();
    let navigator = use_navigator();
    let back_navigator = navigator.clone();
    let continue_navigator = navigator.clone();

    rsx! {
        PageSection {
            PageIntro {
                title: "Disk".to_string(),
                description: "Choose the whole disk that should be erased and installed.".to_string(),
            }
            UiButton {
                variant: ButtonVariant::Ghost,
                onpress: move |_: MouseEvent| refresh_disks(state),
                class: "self-start".to_string(),
                "Refresh disks"
            }
            if snapshot.ui.available_disks.is_empty() {
                NoticePanel {
                    Typography {
                        tag: TypographyTag::P,
                        class: "m-0".to_string(),
                        "No disks loaded yet. Press \"Refresh disks\" to inspect available devices."
                    }
                }
            } else {
                Flexbox {
                    direction: "flex-col".to_string(),
                    gap: "gap-4".to_string(),
                    for disk in snapshot.ui.available_disks.iter().cloned() {
                        {
                            let disk_path = disk.path.clone();
                            let is_selected = selected_disk == disk_path;

                            rsx! {
                                DiskCard {
                                    disk: disk,
                                    is_selected: is_selected,
                                    on_select: move |_| {
                                        let mut draft = state.write();
                                        draft.config.target_disk = disk_path.clone();
                                        draft.ui.error_message = None;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !snapshot.config.target_disk.is_empty() {
                Typography {
                    tag: TypographyTag::P,
                    class: "m-0 text-sm font-semibold text-emerald-700".to_string(),
                    "Current selection: {snapshot.config.target_disk}"
                }
            }
            ValidationList { messages: validation_errors }
            ActionRow {
                UiButton {
                    variant: ButtonVariant::Ghost,
                    onpress: move |_: MouseEvent| {
                        back_navigator.push(Route::User {});
                    },
                    "Back"
                }
                UiButton {
                    onpress: move |_: MouseEvent| {
                        if continue_from_disk(state) {
                            continue_navigator.push(Route::Summary {});
                        }
                    },
                    "Continue"
                }
            }
        }
    }
}
