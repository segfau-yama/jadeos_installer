use dioxus::prelude::*;
use dioxus_material::{Button, TextButton};

use crate::gui::components::{DiskCard, ValidationList};
use crate::gui::controller::{continue_from_disk, navigate_to, refresh_disks};
use crate::gui::state::{InstallerState, InstallerStep};
use crate::gui::validation::disk_validation_errors;

#[component]
pub fn DiskPage(mut state: Signal<InstallerState>) -> Element {
    let snapshot = state();
    let validation_errors = disk_validation_errors(&snapshot.config);
    let selected_disk = snapshot.config.target_disk.clone();

    rsx! {
        section {
            style: "display: flex; flex-direction: column; gap: 20px;",
            div {
                style: "display: flex; flex-wrap: wrap; justify-content: space-between; gap: 16px; align-items: flex-end;",
                div {
                    style: "display: flex; flex-direction: column; gap: 10px;",
                    h2 {
                        style: "margin: 0; color: #10201b; font-size: 2rem;",
                        "Disk"
                    }
                    p {
                        style: "margin: 0; color: #51625a; max-width: 60ch;",
                        "Choose the whole disk that should be erased and installed."
                    }
                }
                TextButton {
                    onpress: move |_| refresh_disks(state),
                    "Refresh disks"
                }
            }
            if snapshot.ui.available_disks.is_empty() {
                div {
                    style: "padding: 18px; border-radius: 22px; background: #f4faf6; color: #51625a; border: 1px solid #dbe7e0;",
                    "No disks loaded yet. Press \"Refresh disks\" to inspect available devices."
                }
            } else {
                div {
                    style: "display: grid; gap: 16px;",
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
                p {
                    style: "margin: 0; color: #275a4f; font-weight: 600;",
                    "Current selection: {snapshot.config.target_disk}"
                }
            }
            ValidationList { messages: validation_errors }
            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px; margin-top: 8px;",
                TextButton {
                    onpress: move |_| navigate_to(state, InstallerStep::User),
                    "Back"
                }
                Button {
                    onpress: move |_| continue_from_disk(state),
                    "Continue"
                }
            }
        }
    }
}
