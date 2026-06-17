use dioxus::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeColors {
    pub page_background: &'static str,
    pub surface_base: &'static str,
    pub surface_elevated: &'static str,
    pub surface_muted: &'static str,
    pub surface_accent: &'static str,
    pub surface_inverse: &'static str,
    pub surface_neutral: &'static str,
    pub surface_neutral_hover: &'static str,
    pub surface_disabled: &'static str,
    pub accent_surface: &'static str,
    pub accent_surface_hover: &'static str,
    pub accent_fill_soft: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,
    pub text_inverse: &'static str,
    pub text_inverse_surface: &'static str,
    pub text_accent: &'static str,
    pub text_disabled: &'static str,
    pub border_subtle: &'static str,
    pub border_neutral: &'static str,
    pub border_accent: &'static str,
    pub border_accent_hover: &'static str,
    pub border_danger: &'static str,
    pub accent_bg: &'static str,
    pub accent_bg_hover: &'static str,
    pub accent_fg: &'static str,
    pub danger_bg: &'static str,
    pub danger_fg: &'static str,
    pub overlay_bg: &'static str,
    pub focus_border: &'static str,
    pub focus_ring: &'static str,
    pub focus_visible_ring: &'static str,
    pub track_bg: &'static str,
    pub progress_fill: &'static str,
    pub warning_bg: &'static str,
    pub warning_fg: &'static str,
    pub warning_border: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeShape {
    pub card_radius: &'static str,
    pub control_radius: &'static str,
    pub pill_radius: &'static str,
    pub dialog_radius: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeShadow {
    pub card: &'static str,
    pub interactive: &'static str,
    pub overlay: &'static str,
    pub none: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub shape: ThemeShape,
    pub shadow: ThemeShadow,
}

impl Theme {
    pub const fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ThemeColors {
                page_background: "bg-transparent",
                surface_base: "bg-white/90",
                surface_elevated: "bg-white/95",
                surface_muted: "bg-white/80",
                surface_accent: "bg-emerald-50/75",
                surface_inverse: "bg-jade-950",
                surface_neutral: "bg-slate-50/80",
                surface_neutral_hover: "hover:bg-emerald-50",
                surface_disabled: "bg-slate-100",
                accent_surface: "bg-emerald-100",
                accent_surface_hover: "hover:bg-emerald-100",
                accent_fill_soft: "bg-emerald-300",
                text_primary: "text-jade-950",
                text_secondary: "text-emerald-900/70",
                text_muted: "text-emerald-900/65",
                text_inverse: "text-white",
                text_inverse_surface: "text-emerald-50",
                text_accent: "text-emerald-700",
                text_disabled: "text-slate-500",
                border_subtle: "border-emerald-900/10",
                border_neutral: "border-slate-200",
                border_accent: "border-emerald-600/40",
                border_accent_hover: "hover:border-emerald-400/40",
                border_danger: "border-rose-200",
                accent_bg: "bg-emerald-700",
                accent_bg_hover: "hover:bg-emerald-800",
                accent_fg: "text-emerald-800",
                danger_bg: "bg-rose-50/90",
                danger_fg: "text-rose-700",
                overlay_bg: "bg-emerald-950/40",
                focus_border: "focus:border-emerald-400/60",
                focus_ring: "focus:ring-emerald-100",
                focus_visible_ring: "focus-visible:ring-emerald-400/70",
                track_bg: "bg-emerald-950/10",
                progress_fill: "from-emerald-600 to-teal-500",
                warning_bg: "bg-amber-50/80",
                warning_fg: "text-amber-800",
                warning_border: "border-amber-200",
            },
            shape: ThemeShape {
                card_radius: "rounded-[2rem]",
                control_radius: "rounded-[1.35rem]",
                pill_radius: "rounded-full",
                dialog_radius: "rounded-[2rem]",
            },
            shadow: ThemeShadow {
                card: "shadow",
                interactive: "shadow",
                overlay: "shadow-[0_30px_90px_rgba(15,23,42,0.22)]",
                none: "shadow-none",
            },
        }
    }

    pub const fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ThemeColors {
                page_background: "bg-jade-950",
                surface_base: "bg-emerald-950/90",
                surface_elevated: "bg-emerald-950/95",
                surface_muted: "bg-emerald-950/75",
                surface_accent: "bg-emerald-900/60",
                surface_inverse: "bg-jade-950",
                surface_neutral: "bg-slate-800/80",
                surface_neutral_hover: "hover:bg-emerald-900/70",
                surface_disabled: "bg-slate-800",
                accent_surface: "bg-emerald-800",
                accent_surface_hover: "hover:bg-emerald-700",
                accent_fill_soft: "bg-emerald-600",
                text_primary: "text-emerald-50",
                text_secondary: "text-emerald-100/80",
                text_muted: "text-emerald-100/65",
                text_inverse: "text-jade-950",
                text_inverse_surface: "text-emerald-50",
                text_accent: "text-emerald-200",
                text_disabled: "text-slate-400",
                border_subtle: "border-emerald-100/10",
                border_neutral: "border-slate-700",
                border_accent: "border-emerald-300/40",
                border_accent_hover: "hover:border-emerald-300/40",
                border_danger: "border-rose-300/30",
                accent_bg: "bg-emerald-300",
                accent_bg_hover: "hover:bg-emerald-200",
                accent_fg: "text-emerald-50",
                danger_bg: "bg-rose-950/60",
                danger_fg: "text-rose-100",
                overlay_bg: "bg-jade-950/70",
                focus_border: "focus:border-emerald-300/60",
                focus_ring: "focus:ring-emerald-900/70",
                focus_visible_ring: "focus-visible:ring-emerald-300/55",
                track_bg: "bg-emerald-100/10",
                progress_fill: "from-emerald-400 to-teal-300",
                warning_bg: "bg-amber-900/40",
                warning_fg: "text-amber-100",
                warning_border: "border-amber-300/30",
            },
            shape: ThemeShape {
                card_radius: "rounded-[2rem]",
                control_radius: "rounded-[1.35rem]",
                pill_radius: "rounded-full",
                dialog_radius: "rounded-[2rem]",
            },
            shadow: ThemeShadow {
                card: "shadow",
                interactive: "shadow",
                overlay: "shadow-[0_30px_90px_rgba(15,23,42,0.22)]",
                none: "shadow-none",
            },
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ThemeProviderProps {
    #[props(default)]
    theme: Theme,
    children: Element,
}

#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    use_context_provider(|| props.theme);

    rsx! {
        {props.children}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_theme_populated(theme: Theme) {
        for value in [
            theme.colors.page_background,
            theme.colors.surface_base,
            theme.colors.surface_elevated,
            theme.colors.surface_muted,
            theme.colors.surface_accent,
            theme.colors.surface_inverse,
            theme.colors.surface_neutral,
            theme.colors.surface_neutral_hover,
            theme.colors.surface_disabled,
            theme.colors.accent_surface,
            theme.colors.accent_surface_hover,
            theme.colors.accent_fill_soft,
            theme.colors.text_primary,
            theme.colors.text_secondary,
            theme.colors.text_muted,
            theme.colors.text_inverse,
            theme.colors.text_inverse_surface,
            theme.colors.text_accent,
            theme.colors.text_disabled,
            theme.colors.border_subtle,
            theme.colors.border_neutral,
            theme.colors.border_accent,
            theme.colors.border_accent_hover,
            theme.colors.border_danger,
            theme.colors.accent_bg,
            theme.colors.accent_bg_hover,
            theme.colors.accent_fg,
            theme.colors.danger_bg,
            theme.colors.danger_fg,
            theme.colors.overlay_bg,
            theme.colors.focus_border,
            theme.colors.focus_ring,
            theme.colors.focus_visible_ring,
            theme.colors.track_bg,
            theme.colors.progress_fill,
            theme.colors.warning_bg,
            theme.colors.warning_fg,
            theme.colors.warning_border,
            theme.shape.card_radius,
            theme.shape.control_radius,
            theme.shape.pill_radius,
            theme.shape.dialog_radius,
            theme.shadow.card,
            theme.shadow.interactive,
            theme.shadow.overlay,
            theme.shadow.none,
        ] {
            assert!(!value.is_empty());
        }
    }

    #[test]
    fn light_theme_is_populated() {
        let theme = Theme::light();
        assert_eq!(theme.mode, ThemeMode::Light);
        assert_theme_populated(theme);
    }

    #[test]
    fn dark_theme_is_populated() {
        let theme = Theme::dark();
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_theme_populated(theme);
    }
}
