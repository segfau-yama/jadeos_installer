use dioxus::prelude::*;
use dioxus_material::Chip;

use crate::gui::state::InstallerStep;

#[component]
pub fn ProgressHeader(step: InstallerStep) -> Element {
    let steps = [
        ("Welcome", InstallerStep::Welcome),
        ("User", InstallerStep::User),
        ("Disk", InstallerStep::Disk),
        ("Summary", InstallerStep::Summary),
        ("Install", InstallerStep::Install),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 10px;",
            p {
                style: "margin: 0; color: #4a6157; font-size: 13px; font-weight: 700; letter-spacing: 0.12em; text-transform: uppercase;",
                "Progress"
            }
            div {
                style: "display: flex; flex-wrap: wrap; gap: 12px;",
                for (index, (label, item_step)) in steps.into_iter().enumerate() {
                    div {
                        key: "{label}",
                        Chip {
                            is_selected: Some(item_step == step),
                            onclick: move |_| {},
                            "{index + 1}. {label}"
                        }
                    }
                }
            }
        }
    }
}
