use dioxus::prelude::*;
use std::{borrow::Cow, rc::Rc};

/// Theme component.
///
/// This component provides access to [`UseTheme`](UseTheme) to its children.
#[component]
pub fn Theme(
    /// Primary color.
    #[props(into, default = Cow::Borrowed("#006e4e"))]
    primary_color: Cow<'static, str>,

    /// Background color.
    #[props(into, default = Cow::Borrowed("#eef4f0"))]
    background_color: Cow<'static, str>,

    /// Secondary container color.
    #[props(into, default = Cow::Borrowed("#d9f2e6"))]
    secondary_container_color: Cow<'static, str>,

    /// Surface color.
    #[props(into, default = Cow::Borrowed("#ffffff"))]
    surface_color: Cow<'static, str>,

    /// Surface variant color.
    #[props(into, default = Cow::Borrowed("#f4faf6"))]
    surface_variant_color: Cow<'static, str>,

    /// Outline color.
    #[props(into, default = Cow::Borrowed("#a6b8ae"))]
    outline_color: Cow<'static, str>,

    /// On-surface color.
    #[props(into, default = Cow::Borrowed("#15211b"))]
    on_surface_color: Cow<'static, str>,

    /// Muted on-surface color.
    #[props(into, default = Cow::Borrowed("#55645c"))]
    on_surface_variant: Cow<'static, str>,

    /// Color on primary surfaces.
    #[props(into, default = Cow::Borrowed("#ffffff"))]
    on_primary_color: Cow<'static, str>,

    /// Border radius medium.
    #[props(into, default = Cow::Borrowed("24px"))]
    border_radius_medium: Cow<'static, str>,

    /// Border radius.
    #[props(into, default = Cow::Borrowed("12px"))]
    border_radius_small: Cow<'static, str>,

    /// Small label font size.
    #[props(default = 12.)]
    label_small: f32,

    /// Medium label font size.
    #[props(default = 16.)]
    label_medium: f32,

    children: Element,
) -> Element {
    use_context_provider(move || {
        Rc::new(UseTheme {
            primary_color: primary_color.clone(),
            background_color: background_color.clone(),
            secondary_container_color: secondary_container_color.clone(),
            surface_color: surface_color.clone(),
            surface_variant_color: surface_variant_color.clone(),
            outline_color: outline_color.clone(),
            on_surface_color: on_surface_color.clone(),
            on_surface_variant: on_surface_variant.clone(),
            on_primary_color: on_primary_color.clone(),
            border_radius_medium: border_radius_medium.clone(),
            border_radius_small: border_radius_small.clone(),
            label_small,
            label_medium,
        })
    });

    children
}

pub struct UseTheme {
    pub primary_color: Cow<'static, str>,
    pub background_color: Cow<'static, str>,
    pub secondary_container_color: Cow<'static, str>,
    pub surface_color: Cow<'static, str>,
    pub surface_variant_color: Cow<'static, str>,
    pub outline_color: Cow<'static, str>,
    pub on_surface_color: Cow<'static, str>,
    pub on_surface_variant: Cow<'static, str>,
    pub on_primary_color: Cow<'static, str>,
    pub border_radius_medium: Cow<'static, str>,
    pub border_radius_small: Cow<'static, str>,
    pub label_small: f32,
    pub label_medium: f32,
}

pub fn use_theme() -> Rc<UseTheme> {
    use_context()
}
