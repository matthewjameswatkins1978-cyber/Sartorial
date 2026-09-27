use std::env;

/// Color policy for output rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorChoice {
    /// Automatically enable color if stdout is a TTY and NO_COLOR is not set.
    #[default]
    Auto,
    /// Force ANSI colors always.
    Always,
    /// Strip or omit ANSI colors.
    Never,
}

impl ColorChoice {
    /// Determine if color should be rendered based on choice, TTY status, and environment variables.
    pub fn should_render_color(&self, is_tty: bool) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Auto => {
                if !is_tty {
                    return false;
                }
                // Respect NO_COLOR standard (https://no-color.org)
                if env::var_os("NO_COLOR").is_some_and(|val| !val.is_empty()) {
                    return false;
                }
                true
            }
        }
    }
}

/// Spacing and visual density.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Density {
    /// No extra vertical padding, minimal column margins.
    Compact,
    /// Default balanced spacing following Biscuit Logic standard.
    #[default]
    Standard,
    /// Extra vertical breathing room for long-form reading.
    Roomy,
}

/// Border styling for separators and table headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    /// No separator lines.
    None,
    /// Subtle restrained single horizontal lines (`───` or `---`).
    #[default]
    Subtle,
}

/// Glyph/symbol mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolMode {
    /// Unicode when supported by the terminal, ASCII fallback otherwise.
    #[default]
    Auto,
    /// Always use clean Unicode glyphs (✓, !, ×, etc.).
    Unicode,
    /// ASCII-only glyphs ([OK], [!], [X], etc.) for maximum legacy/pipe portability.
    Ascii,
}

impl SymbolMode {
    pub fn resolve(&self, is_tty: bool) -> Self {
        match self {
            Self::Unicode => Self::Unicode,
            Self::Ascii => Self::Ascii,
            Self::Auto => {
                // On Windows legacy console or when piped, default safely
                if !is_tty {
                    Self::Ascii
                } else {
                    Self::Unicode
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InteractiveMode {
    /// Interactive only when stdin & stdout are interactive terminals.
    #[default]
    Auto,
    /// Force interactive mode.
    On,
    /// Force non-interactive / deterministic fallback mode.
    Off,
}

impl InteractiveMode {
    /// Evaluate interactivity using both stdin and stdout suitability.
    pub fn is_interactive(&self, stdin_is_tty: bool, stdout_is_tty: bool) -> bool {
        match self {
            Self::On => true,
            Self::Off => false,
            Self::Auto => stdin_is_tty && stdout_is_tty,
        }
    }
}

use crate::accessibility::AccessibilityMode;
use crate::motion::MotionMode;
use crate::pager::PagerMode;
use crate::style::{Preset, ResolvedStyle};
use crate::verbosity::Verbosity;

/// Sartorial configuration embodying the Biscuit Logic CLI Presentation Standard.
#[derive(Debug, Clone)]
pub struct Config {
    /// Active visual preset (default is House).
    pub preset: Preset,
    /// Restrained accent color.
    pub accent: anstyle::AnsiColor,
    /// Layout density.
    pub density: Density,
    /// Border style.
    pub border: BorderStyle,
    /// Symbol rendering mode.
    pub symbols: SymbolMode,
    /// Color policy.
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
            accent: preset.default_accent(),
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
    /// Create a quiet, default configuration (House preset).
    pub fn new() -> Self {
        Self::default()
    }

    /// Select one of the four first-party presets: House, BlackTie, Workwear, Studio.
    pub fn with_preset(mut self, preset: Preset) -> Self {
        self.preset = preset;
        self.accent = preset.default_accent();
        self.density = preset.default_density();
        self
    }

    /// Set a custom accent color (escape hatch for brand adaptation).
    pub fn with_accent(mut self, accent: anstyle::AnsiColor) -> Self {
        self.accent = accent;
        self
    }

    /// Set layout density.
    pub fn with_density(mut self, density: Density) -> Self {
        self.density = density;
        self
    }

    /// Set border style.
    pub fn with_border(mut self, border: BorderStyle) -> Self {
        self.border = border;
        self
    }

    /// Set symbol mode.
    pub fn with_symbols(mut self, symbols: SymbolMode) -> Self {
        self.symbols = symbols;
        self
    }

    /// Set color choice.
    pub fn with_color(mut self, color: ColorChoice) -> Self {
        self.color = color;
        self
    }

    /// Set interactive mode.
    pub fn with_interactive(mut self, interactive: InteractiveMode) -> Self {
        self.interactive = interactive;
        self
    }

    /// Set motion policy.
    pub fn with_motion(mut self, motion: MotionMode) -> Self {
        self.motion = motion;
        self
    }

    /// Set accessibility policy.
    pub fn with_accessibility(mut self, accessibility: AccessibilityMode) -> Self {
        self.accessibility = accessibility;
        self
    }

    /// Set verbosity policy.
    pub fn with_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.verbosity = verbosity;
        self
    }

    /// Set paging policy.
    pub fn with_pager(mut self, pager: PagerMode) -> Self {
        self.pager = pager;
        self
    }

    /// Set an explicit terminal width (useful for testing or fixed width rendering).
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
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
        let symbols = if self.accessibility.is_plain() {
            SymbolMode::Ascii
        } else {
            self.symbols.resolve(is_tty)
        };

        let color_enabled =
            self.color.should_render_color(is_tty) && !self.accessibility.is_plain();

        // Markers depend on the resolved symbol set so ASCII contexts never
        // leak non-ASCII glyphs into the layout grammar.
        let (title_marker, section_marker) = if symbols == SymbolMode::Ascii {
            (
                self.preset.ascii_structural_marker(),
                self.preset.ascii_structural_marker(),
            )
        } else {
            (
                self.preset.structural_marker(),
                self.preset.structural_marker(),
            )
        };

        ResolvedStyle {
            preset: self.preset,
            accent: self.accent,
            density: self.density,
            border: self.border,
            symbols,
            action_spacing: self.preset.action_spacing(),
            rule_char: self.preset.rule_char(symbols),
            progress_treatment: self.preset.progress_treatment(),
            color_enabled,
            title_case: self.preset.title_case(),
            title_marker,
            title_block_rule: self.preset.title_block_rule(),
            status_layout: self.preset.status_layout(),
            status_gap: self.preset.status_gap(),
            section_marker,
            section_rule: self.preset.section_rule(),
            component_gap: self.preset.component_gap(),
            fact_uppercase: self.preset.fact_uppercase(),
            fact_colon: self.preset.fact_colon(),
            notice_compact: self.preset.notice_compact(),
            table_headers: self.preset.table_headers(),
        }
    }
}
