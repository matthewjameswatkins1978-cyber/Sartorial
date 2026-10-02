use std::env;

use crate::accessibility::AccessibilityMode;
use crate::motion::MotionMode;
use crate::pager::PagerMode;
use crate::verbosity::Verbosity;
use sartorial_core::{Preset, ResolvedStyle};

pub use sartorial_core::{BorderStyle, Density, SymbolMode};

use sartorial_core::Theme;

/// Grammar + density + symbol re-exports under their historic paths.
pub use sartorial_core::capabilities::Interactive as InteractiveMode;

/// Colour policy for output rendering.
///
/// Live-sniffing side of the policy: [`ColorPolicy`](sartorial_core::ColorPolicy)
/// in core takes an explicit environment snapshot for determinism; this enum
/// preserves the historic `should_render_color(is_tty)` helper that reads
/// `NO_COLOR` from the process environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorChoice {
    #[default]
    Auto,
    Always,
    Never,
}

impl ColorChoice {
    pub fn should_render_color(&self, is_tty: bool) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Auto => {
                if !is_tty {
                    return false;
                }
                if env::var_os("NO_COLOR").is_some_and(|val| !val.is_empty()) {
                    return false;
                }
                true
            }
        }
    }

    pub fn no_color_env_present() -> bool {
        env::var_os("NO_COLOR").is_some_and(|val| !val.is_empty())
    }
}

/// Sartorial configuration: preset grammar plus theme identity plus
/// runtime policies. Resolution delegates to core so renderers consume
/// one resolved authority.
///
/// ```rust
/// use sartorial::{Config, Preset, Theme};
/// use anstyle::AnsiColor;
/// let theme = Theme::builder("Terrorbats").accent(AnsiColor::Red).build();
/// let config = Config::new().with_preset(Preset::Workwear).with_theme(theme);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Active presentation grammar (default House).
    pub preset: Preset,
    /// Explicit visual identity. When `None`, the preset's familiar default
    /// visuals are used so existing presets keep their personalities.
    /// Once set, changing the preset never silently drops the theme.
    pub theme: Option<Theme>,
    /// Restrained accent colour (mirrors the active theme's accent).
    pub accent: anstyle::AnsiColor,
    /// Layout density.
    pub density: Density,
    /// Border style.
    pub border: BorderStyle,
    /// Symbol rendering mode.
    pub symbols: SymbolMode,
    /// Colour policy.
    pub color: ColorChoice,
    /// Interactive mode.
    pub interactive: InteractiveMode,
    /// Motion policy.
    pub motion: MotionMode,
    /// Accessibility mode.
    pub accessibility: AccessibilityMode,
    /// Verbosity policy.
    pub verbosity: Verbosity,
    /// Paging policy.
    pub pager: PagerMode,
    /// Terminal width override (if None, detected automatically).
    pub width: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        let preset = Preset::House;
        Self {
            preset,
            theme: None,
            accent: Theme::preset_default(preset).accent,
            density: preset.default_density(),
            border: BorderStyle::Subtle,
            symbols: SymbolMode::Auto,
            color: ColorChoice::Auto,
            interactive: InteractiveMode::Auto,
            motion: MotionMode::Auto,
            accessibility: AccessibilityMode::Auto,
            verbosity: Verbosity::Normal,
            pager: PagerMode::Auto,
            width: None,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_preset(mut self, preset: Preset) -> Self {
        self.preset = preset;
        // An explicitly selected theme survives preset changes; only the
        // implicit default follows the grammar.
        if self.theme.is_none() {
            self.accent = Theme::preset_default(preset).accent;
        }
        self.density = preset.default_density();
        self
    }

    /// Select an explicit visual identity. Combinations such as
    /// `Workwear + Terrorbats theme` need no new preset.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.accent = theme.accent;
        self.theme = Some(theme);
        self
    }

    /// Set a custom accent colour (escape hatch for brand adaptation).
    /// Applied on top of the active (or default) theme.
    pub fn with_accent(mut self, accent: anstyle::AnsiColor) -> Self {
        let mut theme = self
            .theme
            .take()
            .unwrap_or_else(|| Theme::preset_default(self.preset));
        theme.accent = accent;
        self.accent = accent;
        self.theme = Some(theme);
        self
    }

    pub fn with_density(mut self, density: Density) -> Self {
        self.density = density;
        self
    }

    pub fn with_border(mut self, border: BorderStyle) -> Self {
        self.border = border;
        self
    }

    pub fn with_symbols(mut self, symbols: SymbolMode) -> Self {
        self.symbols = symbols;
        self
    }

    pub fn with_color(mut self, color: ColorChoice) -> Self {
        self.color = color;
        self
    }

    pub fn with_interactive(mut self, interactive: InteractiveMode) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn with_motion(mut self, motion: MotionMode) -> Self {
        self.motion = motion;
        self
    }

    pub fn with_accessibility(mut self, accessibility: AccessibilityMode) -> Self {
        self.accessibility = accessibility;
        self
    }

    pub fn with_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.verbosity = verbosity;
        self
    }

    pub fn with_pager(mut self, pager: PagerMode) -> Self {
        self.pager = pager;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Active theme: explicit selection, else the preset's familiar default.
    pub fn active_theme(&self) -> Theme {
        self.theme
            .clone()
            .unwrap_or_else(|| Theme::preset_default(self.preset))
    }

    /// Single authority for interactivity based on configuration and stream capabilities.
    pub fn is_interactive(&self) -> bool {
        use std::io::IsTerminal;
        self.interactive.is_interactive(
            std::io::stdin().is_terminal(),
            std::io::stdout().is_terminal(),
        )
    }

    /// Resolves configuration into a concrete, consistent presentation authority.
    pub fn resolve_style(&self, is_tty: bool) -> ResolvedStyle {
        let plain_accessibility = self.accessibility.is_plain();
        let unicode = is_tty && !plain_accessibility;
        let symbols = if plain_accessibility {
            SymbolMode::Ascii
        } else {
            self.symbols.resolve(unicode)
        };
        let color_enabled = self.color.should_render_color(is_tty) && !plain_accessibility;
        let theme = self.active_theme();
        ResolvedStyle::resolve(
            self.preset,
            &theme,
            self.density,
            self.border,
            symbols,
            color_enabled,
        )
    }
}
