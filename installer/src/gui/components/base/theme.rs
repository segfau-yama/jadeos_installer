use dioxus::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeColor {
    Page,
    Surface,
    SurfaceRaised,
    SurfaceAccent,
    SurfaceInverse,
    SurfaceHover,
    SurfaceDisabled,
    Accent,
    AccentHover,
    AccentSoft,
    Text,
    TextMuted,
    TextInverse,
    TextAccent,
    TextDisabled,
    Border,
    BorderAccent,
    BorderHover,
    BorderDanger,
    DangerBg,
    DangerText,
    Overlay,
    FocusBorder,
    FocusRing,
    FocusVisible,
    Track,
    Progress,
    WarningBg,
    WarningText,
    WarningBorder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeRadius {
    Card,
    Control,
    Pill,
    Dialog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeShadow {
    Card,
    Interactive,
    Overlay,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub mode: ThemeMode,
}

impl Theme {
    pub const fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
        }
    }

    pub const fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
        }
    }

    pub const fn color(self, color: ThemeColor) -> &'static str {
        match self.mode {
            ThemeMode::Light => light_color(color),
            ThemeMode::Dark => dark_color(color),
        }
    }

    pub const fn radius(self, radius: ThemeRadius) -> &'static str {
        match radius {
            ThemeRadius::Card => "rounded-[2rem]",
            ThemeRadius::Control => "rounded-[1.35rem]",
            ThemeRadius::Pill => "rounded-full",
            ThemeRadius::Dialog => "rounded-[2rem]",
        }
    }

    pub const fn shadow(self, shadow: ThemeShadow) -> &'static str {
        match shadow {
            ThemeShadow::Card => "shadow",
            ThemeShadow::Interactive => "shadow",
            ThemeShadow::Overlay => "shadow-[0_30px_90px_rgba(15,23,42,0.22)]",
            ThemeShadow::None => "shadow-none",
        }
    }
}

const fn light_color(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Page => "bg-transparent",
        ThemeColor::Surface => "bg-white/90",
        ThemeColor::SurfaceRaised => "bg-white/95",
        ThemeColor::SurfaceAccent => "bg-emerald-50/75",
        ThemeColor::SurfaceInverse => "bg-jade-950",
        ThemeColor::SurfaceHover => "hover:bg-emerald-50",
        ThemeColor::SurfaceDisabled => "bg-slate-100",
        ThemeColor::Accent => "bg-emerald-700",
        ThemeColor::AccentHover => "hover:bg-emerald-800",
        ThemeColor::AccentSoft => "bg-emerald-300",
        ThemeColor::Text => "text-jade-950",
        ThemeColor::TextMuted => "text-emerald-900/70",
        ThemeColor::TextInverse => "text-white",
        ThemeColor::TextAccent => "text-emerald-700",
        ThemeColor::TextDisabled => "text-slate-500",
        ThemeColor::Border => "border-emerald-900/10",
        ThemeColor::BorderAccent => "border-emerald-600/40",
        ThemeColor::BorderHover => "hover:border-emerald-400/40",
        ThemeColor::BorderDanger => "border-rose-200",
        ThemeColor::DangerBg => "bg-rose-50/90",
        ThemeColor::DangerText => "text-rose-700",
        ThemeColor::Overlay => "bg-emerald-950/40",
        ThemeColor::FocusBorder => "focus:border-emerald-400/60",
        ThemeColor::FocusRing => "focus:ring-emerald-100",
        ThemeColor::FocusVisible => "focus-visible:ring-emerald-400/70",
        ThemeColor::Track => "bg-emerald-950/10",
        ThemeColor::Progress => "from-emerald-600 to-teal-500",
        ThemeColor::WarningBg => "bg-amber-50/80",
        ThemeColor::WarningText => "text-amber-800",
        ThemeColor::WarningBorder => "border-amber-200",
    }
}

const fn dark_color(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Page => "bg-jade-950",
        ThemeColor::Surface => "bg-emerald-950/90",
        ThemeColor::SurfaceRaised => "bg-emerald-950/95",
        ThemeColor::SurfaceAccent => "bg-emerald-900/60",
        ThemeColor::SurfaceInverse => "bg-jade-950",
        ThemeColor::SurfaceHover => "hover:bg-emerald-900/70",
        ThemeColor::SurfaceDisabled => "bg-slate-800",
        ThemeColor::Accent => "bg-emerald-300",
        ThemeColor::AccentHover => "hover:bg-emerald-200",
        ThemeColor::AccentSoft => "bg-emerald-600",
        ThemeColor::Text => "text-emerald-50",
        ThemeColor::TextMuted => "text-emerald-100/75",
        ThemeColor::TextInverse => "text-jade-950",
        ThemeColor::TextAccent => "text-emerald-200",
        ThemeColor::TextDisabled => "text-slate-400",
        ThemeColor::Border => "border-emerald-100/10",
        ThemeColor::BorderAccent => "border-emerald-300/40",
        ThemeColor::BorderHover => "hover:border-emerald-300/40",
        ThemeColor::BorderDanger => "border-rose-300/30",
        ThemeColor::DangerBg => "bg-rose-950/60",
        ThemeColor::DangerText => "text-rose-100",
        ThemeColor::Overlay => "bg-jade-950/70",
        ThemeColor::FocusBorder => "focus:border-emerald-300/60",
        ThemeColor::FocusRing => "focus:ring-emerald-900/70",
        ThemeColor::FocusVisible => "focus-visible:ring-emerald-300/55",
        ThemeColor::Track => "bg-emerald-100/10",
        ThemeColor::Progress => "from-emerald-400 to-teal-300",
        ThemeColor::WarningBg => "bg-amber-900/40",
        ThemeColor::WarningText => "text-amber-100",
        ThemeColor::WarningBorder => "border-amber-300/30",
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

    fn assert_tokens_populated(theme: Theme) {
        for value in [
            theme.color(ThemeColor::Page),
            theme.color(ThemeColor::Surface),
            theme.color(ThemeColor::Text),
            theme.color(ThemeColor::TextMuted),
            theme.color(ThemeColor::Border),
            theme.color(ThemeColor::WarningBg),
            theme.radius(ThemeRadius::Card),
            theme.radius(ThemeRadius::Control),
            theme.radius(ThemeRadius::Pill),
            theme.shadow(ThemeShadow::Card),
            theme.shadow(ThemeShadow::None),
        ] {
            assert!(!value.is_empty());
        }
    }

    #[test]
    fn light_theme_uses_light_mode() {
        let theme = Theme::light();
        assert_eq!(theme.mode, ThemeMode::Light);
        assert_tokens_populated(theme);
    }

    #[test]
    fn dark_theme_uses_dark_mode() {
        let theme = Theme::dark();
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_tokens_populated(theme);
    }
}
