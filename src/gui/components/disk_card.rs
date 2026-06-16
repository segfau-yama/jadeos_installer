use dioxus::prelude::*;

use crate::api::disk::DiskDeviceInfo;
use crate::gui::components::{
    BadgeTone, ButtonVariant, Card, CardBody, CardFooter, CardHeader, Flexbox, InfoTile,
    NoticePanel, PanelTone, StatusBadge, Typography, TypographyTag, UiButton,
};

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

    rsx! {
        Card {
            key: "{disk_path}",
            color: if is_selected {
                "bg-emerald-50/85".to_string()
            } else {
                "bg-white".to_string()
            },
            shadow: "shadow-none".to_string(),
            rounded: "rounded-[1.75rem]".to_string(),
            CardHeader {
                class: "gap-4".to_string(),
                Flexbox {
                    wrap: "flex-wrap".to_string(),
                    items: "items-start".to_string(),
                    justify: "justify-between".to_string(),
                    gap: "gap-4".to_string(),
                    div {
                        class: "flex flex-col gap-1",
                        Typography {
                            tag: TypographyTag::H3,
                            class: "m-0 text-xl font-semibold text-jade-950".to_string(),
                            "{disk_path}"
                        }
                        Typography {
                            tag: TypographyTag::P,
                            class: "m-0 text-sm text-emerald-900/65".to_string(),
                            "{disk.model}"
                        }
                    }
                    Flexbox {
                        wrap: "flex-wrap".to_string(),
                        gap: "gap-2".to_string(),
                        StatusBadge {
                            tone: if is_selected {
                                BadgeTone::Accent
                            } else {
                                BadgeTone::Muted
                            },
                            if is_selected { "Current choice" } else { "Available" }
                        }
                        if disk.removable {
                            StatusBadge {
                                tone: BadgeTone::Muted,
                                "Removable"
                            }
                        }
                        if disk.mounted {
                            StatusBadge {
                                tone: BadgeTone::Warning,
                                "Mounted"
                            }
                        }
                    }
                }
            }
            CardBody {
                class: "pt-0".to_string(),
                div {
                    class: "grid grid-cols-1 gap-3 sm:grid-cols-3",
                    InfoTile {
                        label: "Size".to_string(),
                        value: format!("{:.1} GiB", disk.size_gib()),
                        class: "rounded-[1.25rem] px-4 py-3".to_string(),
                    }
                    InfoTile {
                        label: "Removable".to_string(),
                        value: removable_label.to_string(),
                        class: "rounded-[1.25rem] px-4 py-3".to_string(),
                    }
                    InfoTile {
                        label: "Mounted".to_string(),
                        value: mounted_label.to_string(),
                        class: "rounded-[1.25rem] px-4 py-3".to_string(),
                    }
                }
                if disk.mounted {
                    NoticePanel {
                        tone: PanelTone::Warning,
                        class: "rounded-[1.25rem] px-4 py-3".to_string(),
                        Typography {
                            tag: TypographyTag::P,
                            class: "m-0 text-sm font-medium".to_string(),
                            "Warning: this disk currently has mounted filesystems."
                        }
                    }
                }
            }
            CardFooter {
                UiButton {
                    onpress: move |_| on_select.call(()),
                    variant: if is_selected {
                        ButtonVariant::Secondary
                    } else {
                        ButtonVariant::Primary
                    },
                    "{selection_label}"
                }
            }
        }
    }
}
