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
    #[props(default = false)] disabled: bool,
    onselect: EventHandler<MouseEvent>,
) -> Element {
    let theme = use_theme();
    let border_color = if is_selected {
        theme.primary_color.as_ref()
    } else {
        theme.outline_color.as_ref()
    };
    let background = if is_selected {
        theme.secondary_container_color.as_ref()
    } else {
        "transparent"
    };
    let color = if disabled {
        theme.on_surface_variant.as_ref()
    } else if is_selected {
        theme.primary_color.as_ref()
    } else {
        theme.on_surface_color.as_ref()
    };

    rsx! {
        li {
            display: "block",
            list_style: "none",
            button {
                r#type: "button",
                disabled,
                display: "inline-flex",
                align_items: "center",
                gap: "10px",
                padding: "10px 14px",
                border: "1px solid",
                border_color,
                border_radius: "{theme.border_radius_medium}",
                background,
                color: "{color}",
                opacity: if disabled { "0.58" } else { "1" },
                cursor: if disabled { "not-allowed" } else { "pointer" },
                font_family: "system-ui, sans-serif",
                onclick: move |event| {
                    if !disabled {
                        onselect.call(event);
                    }
                },
                {icon}
                {label}
            }
        }
    }
}
