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
    Accent,
    Text,
    Muted,
    Inverse,
    Danger,
    Warning,
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

    pub const fn bg(self, color: ThemeColor) -> &'static str {
        match self.mode {
            ThemeMode::Light => light_bg(color),
            ThemeMode::Dark => dark_bg(color),
        }
    }

    pub const fn fill(self, color: ThemeColor) -> &'static str {
        match self.mode {
            ThemeMode::Light => light_fill(color),
            ThemeMode::Dark => dark_fill(color),
        }
    }

    pub const fn soft_fill(self, color: ThemeColor) -> &'static str {
        match self.mode {
            ThemeMode::Light => light_soft_fill(color),
            ThemeMode::Dark => dark_soft_fill(color),
        }
    }

    pub const fn hover_bg(self, color: ThemeColor) -> &'static str {
        match self.mode {
            ThemeMode::Light => light_hover_bg(color),
            ThemeMode::Dark => dark_hover_bg(color),
        }
    }

    pub const fn hover_fill(self, color: ThemeColor) -> &'static str {
        match self.mode {
            ThemeMode::Light => light_hover_fill(color),
            ThemeMode::Dark => dark_hover_fill(color),
        }
    }

    pub const fn text(self, color: ThemeColor) -> &'static str {
        match self.mode {
            ThemeMode::Light => light_text(color),
            ThemeMode::Dark => dark_text(color),
        }
    }

    pub const fn border(self, color: ThemeColor) -> &'static str {
        match self.mode {
            ThemeMode::Light => light_border(color),
            ThemeMode::Dark => dark_border(color),
        }
    }

    pub const fn hover_border(self, color: ThemeColor) -> &'static str {
        match self.mode {
            ThemeMode::Light => light_hover_border(color),
            ThemeMode::Dark => dark_hover_border(color),
        }
    }

    pub const fn focus_border(self, color: ThemeColor) -> &'static str {
        match color {
            ThemeColor::Accent => match self.mode {
                ThemeMode::Light => "focus:border-emerald-400/60",
                ThemeMode::Dark => "focus:border-emerald-300/60",
            },
            _ => "",
        }
    }

    pub const fn focus_ring(self, color: ThemeColor) -> &'static str {
        match color {
            ThemeColor::Accent => match self.mode {
                ThemeMode::Light => "focus:ring-emerald-100",
                ThemeMode::Dark => "focus:ring-emerald-900/70",
            },
            _ => "",
        }
    }

    pub const fn focus_visible(self, color: ThemeColor) -> &'static str {
        match color {
            ThemeColor::Accent => match self.mode {
                ThemeMode::Light => "focus-visible:ring-emerald-400/70",
                ThemeMode::Dark => "focus-visible:ring-emerald-300/55",
            },
            _ => "",
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
        match self.mode {
            ThemeMode::Light => match shadow {
                ThemeShadow::Card => {
                    "shadow-[inset_0_1px_0_rgba(255,255,255,0.8),0_10px_24px_rgba(12,34,27,0.04)]"
                }
                ThemeShadow::Interactive => "shadow",
                ThemeShadow::Overlay => "shadow-[0_30px_90px_rgba(15,23,42,0.22)]",
                ThemeShadow::None => "shadow-none",
            },
            ThemeMode::Dark => match shadow {
                ThemeShadow::Card => "shadow-none",
                ThemeShadow::Interactive => "shadow-none",
                ThemeShadow::Overlay => "shadow-[0_30px_90px_rgba(15,23,42,0.22)]",
                ThemeShadow::None => "shadow-none",
            },
        }
    }

    pub const fn overlay(self) -> &'static str {
        match self.mode {
            ThemeMode::Light => "bg-emerald-950/40",
            ThemeMode::Dark => "bg-jade-950/70",
        }
    }

    pub const fn track(self, color: ThemeColor) -> &'static str {
        match color {
            ThemeColor::Accent => match self.mode {
                ThemeMode::Light => "bg-emerald-950/10",
                ThemeMode::Dark => "bg-emerald-100/10",
            },
            _ => self.bg(ThemeColor::Surface),
        }
    }

    pub const fn gradient(self, color: ThemeColor) -> &'static str {
        match color {
            ThemeColor::Accent => match self.mode {
                ThemeMode::Light => "from-emerald-600 to-teal-500",
                ThemeMode::Dark => "from-emerald-400 to-teal-300",
            },
            _ => "",
        }
    }
}

const fn light_bg(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Page => "bg-transparent",
        ThemeColor::Surface => "bg-white",
        ThemeColor::Accent => "bg-emerald-50/75",
        ThemeColor::Text => "bg-transparent",
        ThemeColor::Muted => "bg-slate-200",
        ThemeColor::Inverse => "bg-jade-950",
        ThemeColor::Danger => "bg-rose-50/90",
        ThemeColor::Warning => "bg-amber-50/80",
    }
}

const fn dark_bg(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Page => "bg-jade-950/70",
        ThemeColor::Surface => "bg-emerald-950/40",
        ThemeColor::Accent => "bg-emerald-100/10",
        ThemeColor::Text => "bg-transparent",
        ThemeColor::Muted => "bg-slate-800",
        ThemeColor::Inverse => "bg-emerald-900/60",
        ThemeColor::Danger => "bg-rose-950/60",
        ThemeColor::Warning => "bg-amber-900/40",
    }
}

const fn light_fill(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "bg-emerald-700",
        ThemeColor::Danger => "bg-rose-600",
        ThemeColor::Warning => "bg-amber-600",
        _ => light_bg(color),
    }
}

const fn dark_fill(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "bg-emerald-700",
        ThemeColor::Danger => "bg-rose-600",
        ThemeColor::Warning => "bg-amber-600",
        _ => dark_bg(color),
    }
}

const fn light_soft_fill(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "bg-emerald-300",
        _ => light_bg(color),
    }
}

const fn dark_soft_fill(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "bg-emerald-600",
        _ => dark_bg(color),
    }
}

const fn light_hover_bg(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Surface | ThemeColor::Accent => "hover:bg-emerald-50",
        _ => "",
    }
}

const fn dark_hover_bg(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Surface | ThemeColor::Accent => "hover:bg-emerald-900/70",
        _ => "",
    }
}

const fn light_hover_fill(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "hover:bg-emerald-800",
        ThemeColor::Danger => "hover:bg-rose-700",
        ThemeColor::Warning => "hover:bg-amber-700",
        _ => "",
    }
}

const fn dark_hover_fill(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "hover:bg-emerald-800",
        ThemeColor::Danger => "hover:bg-rose-700",
        ThemeColor::Warning => "hover:bg-amber-700",
        _ => "",
    }
}

const fn light_text(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "text-emerald-700",
        ThemeColor::Text | ThemeColor::Page | ThemeColor::Surface => "text-jade-950",
        ThemeColor::Muted => "text-emerald-900/70",
        ThemeColor::Inverse => "text-white",
        ThemeColor::Danger => "text-rose-700",
        ThemeColor::Warning => "text-amber-800",
    }
}

const fn dark_text(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "text-emerald-200",
        ThemeColor::Text | ThemeColor::Page | ThemeColor::Surface => "text-emerald-50",
        ThemeColor::Muted => "text-emerald-100/75",
        ThemeColor::Inverse => "text-white",
        ThemeColor::Danger => "text-rose-100",
        ThemeColor::Warning => "text-amber-100",
    }
}

const fn light_border(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "border-emerald-600/40",
        ThemeColor::Danger => "border-rose-200",
        ThemeColor::Warning => "border-amber-200",
        _ => "border-emerald-600/40",
    }
}

const fn dark_border(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "border-emerald-300/40",
        ThemeColor::Danger => "border-rose-300/30",
        ThemeColor::Warning => "border-amber-300/30",
        _ => "border-emerald-300/40",
    }
}

const fn light_hover_border(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "hover:border-emerald-400/40",
        _ => "",
    }
}

const fn dark_hover_border(color: ThemeColor) -> &'static str {
    match color {
        ThemeColor::Accent => "hover:border-emerald-300/40",
        _ => "",
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
            theme.bg(ThemeColor::Page),
            theme.bg(ThemeColor::Surface),
            theme.text(ThemeColor::Text),
            theme.text(ThemeColor::Muted),
            theme.border(ThemeColor::Surface),
            theme.bg(ThemeColor::Warning),
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
