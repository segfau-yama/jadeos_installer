use dioxus::prelude::*;

const THEME_COLORS: [ThemeColor; 8] = [
    ThemeColor::Surface,
    ThemeColor::BackGround,
    ThemeColor::Primary,
    ThemeColor::Secondary,
    ThemeColor::Error,
    ThemeColor::Info,
    ThemeColor::Success,
    ThemeColor::Warning,
];
const CSS_VAR_NAMES: [&str; 8] = [
    "--theme-surface",
    "--theme-background",
    "--theme-primary",
    "--theme-secondary",
    "--theme-error",
    "--theme-info",
    "--theme-success",
    "--theme-warning",
];
const CSS_VAR_REFERENCES: [&str; 8] = [
    "var(--theme-surface)",
    "var(--theme-background)",
    "var(--theme-primary)",
    "var(--theme-secondary)",
    "var(--theme-error)",
    "var(--theme-info)",
    "var(--theme-success)",
    "var(--theme-warning)",
];
const LIGHT_PALETTE: [&str; 8] = [
    "#ffffff", "#edf5f1", "#047857", "#475569", "#e11d48", "#0284c7", "#059669", "#d97706",
];
const DARK_PALETTE: [&str; 8] = [
    "#12312a", "#0b1d19", "#6ee7b7", "#cbd5e1", "#fda4af", "#7dd3fc", "#34d399", "#fcd34d",
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeColor {
    Surface,
    BackGround,
    Primary,
    Secondary,
    Error,
    Info,
    Success,
    Warning,
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
        palette(self.mode)[color.index()]
    }
}

impl ThemeColor {
    const fn index(self) -> usize {
        match self {
            ThemeColor::Surface => 0,
            ThemeColor::BackGround => 1,
            ThemeColor::Primary => 2,
            ThemeColor::Secondary => 3,
            ThemeColor::Error => 4,
            ThemeColor::Info => 5,
            ThemeColor::Success => 6,
            ThemeColor::Warning => 7,
        }
    }

    pub const fn css_var_name(self) -> &'static str {
        CSS_VAR_NAMES[self.index()]
    }

    pub const fn css_var(self) -> &'static str {
        CSS_VAR_REFERENCES[self.index()]
    }
}

const fn palette(mode: ThemeMode) -> [&'static str; 8] {
    match mode {
        ThemeMode::Light => LIGHT_PALETTE,
        ThemeMode::Dark => DARK_PALETTE,
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
    let style = theme_scope_style(props.theme);

    rsx! {
        div {
            style: "{style}",
            {props.children}
        }
    }
}

fn theme_scope_style(theme: Theme) -> String {
    THEME_COLORS
        .iter()
        .map(|color| format!("{}: {};", color.css_var_name(), theme.color(*color)))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_theme_populated(theme: Theme) {
        for color in THEME_COLORS {
            assert!(!theme.color(color).is_empty());
            assert!(!color.css_var_name().is_empty());
            assert!(!color.css_var().is_empty());
        }
    }

    #[test]
    fn light_theme_uses_light_mode() {
        let theme = Theme::light();
        assert_eq!(theme.mode, ThemeMode::Light);
        assert_theme_populated(theme);
    }

    #[test]
    fn dark_theme_uses_dark_mode() {
        let theme = Theme::dark();
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_theme_populated(theme);
    }
}
