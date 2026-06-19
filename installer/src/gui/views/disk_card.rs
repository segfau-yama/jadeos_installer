use crate::gui::components::ThemeColor;
use dioxus::prelude::*;

use crate::api::disk::DiskDeviceInfo;
use crate::gui::components::{
    BadgeTone, ButtonVariant, Card, CardBody, CardFooter, CardHeader, Col, Flexbox, Row,
    StatusBadge, Typography, TypographyTag, UiButton,
};

use super::{
    info_tile::InfoTile,
    notice_panel::{NoticePanel, PanelTone},
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
    let card_style = if is_selected {
        format!(
            "background-color: color-mix(in srgb, {} 10%, {}); border-color: color-mix(in srgb, {} 24%, transparent);",
            ThemeColor::Primary.css_var(),
            ThemeColor::Surface.css_var(),
            ThemeColor::Primary.css_var(),
        )
    } else {
        format!(
            "background-color: {}; border-color: color-mix(in srgb, {} 22%, transparent);",
            ThemeColor::Surface.css_var(),
            ThemeColor::Secondary.css_var(),
        )
    };

    rsx! {
        Card {
            key: "{disk_path}",
            class: "rounded-[1.75rem] shadow-none".to_string(),
            style: card_style,
            CardHeader {
                class: "gap-4".to_string(),
                Flexbox {
                    wrap: "flex-wrap".to_string(),
                    items: "items-start".to_string(),
                    justify: "justify-between".to_string(),
                    gap: "gap-4".to_string(),
                    Flexbox {
                        direction: "flex-col".to_string(),
                        gap: "gap-1".to_string(),
                        Typography {
                            tag: TypographyTag::H3,
                            class: "m-0 text-xl font-semibold".to_string(),
                            style: format!("color: {};", ThemeColor::Secondary.css_var()),
                            "{disk_path}"
                        }
                        Typography {
                            tag: TypographyTag::P,
                            class: "m-0 text-sm".to_string(),
                            style: format!("color: {};", ThemeColor::Secondary.css_var()),
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
                Row {
                    cols: "grid-cols-1 sm:grid-cols-3".to_string(),
                    gap: "gap-3".to_string(),
                    Col {
                        InfoTile {
                            label: "Size".to_string(),
                            value: format!("{:.1} GiB", disk.size_gib()),
                            class: "rounded-[1.25rem] px-4 py-3".to_string(),
                        }
                    }
                    Col {
                        InfoTile {
                            label: "Removable".to_string(),
                            value: removable_label.to_string(),
                            class: "rounded-[1.25rem] px-4 py-3".to_string(),
                        }
                    }
                    Col {
                        InfoTile {
                            label: "Mounted".to_string(),
                            value: mounted_label.to_string(),
                            class: "rounded-[1.25rem] px-4 py-3".to_string(),
                        }
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
