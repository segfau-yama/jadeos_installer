use dioxus::prelude::*;
use dioxus_material::{Button, Chip};

use crate::api::disk::DiskDeviceInfo;

#[component]
pub fn DiskCard(disk: DiskDeviceInfo, is_selected: bool, on_select: EventHandler<()>) -> Element {
    let disk_path = disk.path.clone();
    let removable_label = if disk.removable { "yes" } else { "no" };
    let mounted_label = if disk.mounted { "yes" } else { "no" };
    let selection_label = if is_selected {
        "Selected disk"
    } else {
        "Use this disk"
    };
    let action_color = if is_selected { "#00695c" } else { "#275a4f" }.to_string();

    rsx! {
        article {
            key: "{disk_path}",
            style: "display: flex; flex-direction: column; gap: 16px; border: 1px solid #dbe7e0; border-radius: 24px; padding: 20px; background: #fcfefd;",
            div {
                style: "display: flex; flex-wrap: wrap; justify-content: space-between; gap: 16px; align-items: flex-start;",
                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    h3 {
                        style: "margin: 0; color: #10201b; font-size: 1.25rem;",
                        "{disk_path}"
                    }
                    p {
                        style: "margin: 0; color: #51625a;",
                        "{disk.model}"
                    }
                }
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 8px;",
                    div {
                        Chip {
                            is_selected: Some(is_selected),
                            onclick: move |_| on_select.call(()),
                            if is_selected { "Current choice" } else { "Available" }
                        }
                    }
                    if disk.removable {
                        div {
                            Chip {
                                is_selected: Some(false),
                                onclick: move |_| {},
                                "Removable"
                            }
                        }
                    }
                    if disk.mounted {
                        div {
                            Chip {
                                is_selected: Some(true),
                                onclick: move |_| {},
                                "Mounted"
                            }
                        }
                    }
                }
            }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 12px;",
                div {
                    style: "padding: 12px 14px; border-radius: 18px; background: #f4faf6;",
                    p { style: "margin: 0 0 4px; color: #51625a; font-size: 13px; text-transform: uppercase; letter-spacing: 0.08em;", "Size" }
                    p { style: "margin: 0; color: #10201b; font-weight: 600;", "{disk.size_gib():.1} GiB" }
                }
                div {
                    style: "padding: 12px 14px; border-radius: 18px; background: #f4faf6;",
                    p { style: "margin: 0 0 4px; color: #51625a; font-size: 13px; text-transform: uppercase; letter-spacing: 0.08em;", "Removable" }
                    p { style: "margin: 0; color: #10201b; font-weight: 600;", "{removable_label}" }
                }
                div {
                    style: "padding: 12px 14px; border-radius: 18px; background: #f4faf6;",
                    p { style: "margin: 0 0 4px; color: #51625a; font-size: 13px; text-transform: uppercase; letter-spacing: 0.08em;", "Mounted" }
                    p { style: "margin: 0; color: #10201b; font-weight: 600;", "{mounted_label}" }
                }
            }
            if disk.mounted {
                p {
                    style: "margin: 0; padding: 12px 14px; border-radius: 18px; background: #fff4ea; color: #8a3f09;",
                    "Warning: this disk currently has mounted filesystems."
                }
            }
            div {
                Button {
                    onpress: move |_| on_select.call(()),
                    background_color: Some(action_color.clone()),
                    "{selection_label}"
                }
            }
        }
    }
}
