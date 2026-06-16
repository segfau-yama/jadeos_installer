use crate::use_theme;
use dioxus::prelude::*;

#[component]
pub fn NavigationRail(children: Element) -> Element {
    rsx! {
        nav {
            width: "100%",
            ul {
                display: "flex",
                flex_direction: "row",
                flex_wrap: "wrap",
                align_items: "center",
                gap: "12px",
                width: "100%",
                list_style: "none",
                margin: 0,
                padding: 0,
                {children}
            }
        }
    }
}

#[component]
pub fn NavigationRailItem(
    icon: Element,
    label: Element,
    is_selected: bool,
    onselect: EventHandler<MouseEvent>,
) -> Element {
    let theme = use_theme();

    rsx! {
        li {
            display: "block",
            list_style: "none",
            button {
                r#type: "button",
                display: "inline-flex",
                align_items: "center",
                gap: "10px",
                padding: "10px 14px",
                border: "1px solid",
                border_color: if is_selected { theme.primary_color.as_ref() } else { theme.outline_color.as_ref() },
                border_radius: "{theme.border_radius_medium}",
                background: if is_selected { theme.secondary_container_color.as_ref() } else { "transparent" },
                color: if is_selected { theme.primary_color.as_ref() } else { theme.on_surface_color.as_ref() },
                cursor: "pointer",
                font_family: "system-ui, sans-serif",
                onclick: move |event| onselect.call(event),
                {icon}
                {label}
            }
        }
    }
}
