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

/// Interactivity preference.
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

/// Sartorial configuration embodying the Biscuit Logic CLI Presentation Standard.
#[derive(Debug, Clone)]
pub struct Config {
    /// Restrained accent color (default is subtle Cyan/Slate).
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
    /// Terminal width override (if None, detected automatically).
    pub width: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            accent: anstyle::AnsiColor::Cyan,
            density: Density::Standard,
            border: BorderStyle::Subtle,
            symbols: SymbolMode::Auto,
            color: ColorChoice::Auto,
            interactive: InteractiveMode::Auto,
            width: None,
        }
    }
}

impl Config {
    /// Create a quiet, default configuration.
    pub fn new() -> Self {
        Self::default()
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

    /// Set an explicit terminal width (useful for testing or fixed width rendering).
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }
}
