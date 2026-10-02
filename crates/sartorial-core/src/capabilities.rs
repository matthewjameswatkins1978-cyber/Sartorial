use serde::{Deserialize, Serialize};

/// Colour policy. Single authority; nothing else decides whether paint exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ColorPolicy {
    #[default]
    Auto,
    Always,
    Never,
}

impl ColorPolicy {
    pub fn should_render_color(&self, is_tty: bool, no_color_env: bool) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Auto => is_tty && !no_color_env,
        }
    }
}

/// Glyph/symbol mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SymbolMode {
    #[default]
    Auto,
    Unicode,
    Ascii,
}

impl SymbolMode {
    pub fn resolve(&self, unicode: bool) -> Self {
        match self {
            Self::Unicode => Self::Unicode,
            Self::Ascii => Self::Ascii,
            Self::Auto => {
                if unicode {
                    Self::Unicode
                } else {
                    Self::Ascii
                }
            }
        }
    }
}

/// Interactivity authority input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Interactive {
    #[default]
    Auto,
    On,
    Off,
}

impl Interactive {
    pub fn is_interactive(&self, stdin_is_tty: bool, stdout_is_tty: bool) -> bool {
        match self {
            Self::On => true,
            Self::Off => false,
            Self::Auto => stdin_is_tty && stdout_is_tty,
        }
    }
}

/// Explicit terminal capabilities. Core consumes these; it never sniffs
/// the environment while rendering. The batteries-included crate detects
/// once, builds this, and passes it down for deterministic output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capabilities {
    /// Total columns available for layout.
    pub width: usize,
    /// Whether stdout is an attended terminal.
    pub is_tty: bool,
    /// Colour policy outcome already folded with NO_COLOR.
    pub color_enabled: bool,
    /// Whether Unicode glyphs are safe.
    pub unicode: bool,
    /// Whether OSC-8 hyperlinks are safe.
    pub hyperlinks: bool,
    /// Whether motion/animation is permitted.
    pub motion: bool,
    /// Whether interactive prompts are permitted.
    pub interactive: bool,
    /// NO_COLOR was present in the environment at detection time.
    pub no_color_env: bool,
}

impl Capabilities {
    /// Deterministic test-friendly constructor: everything explicit.
    #[allow(clippy::too_many_arguments)]
    pub fn explicit(
        width: usize,
        is_tty: bool,
        color: ColorPolicy,
        no_color_env: bool,
        unicode: bool,
        hyperlinks: bool,
        motion: bool,
        interactive: bool,
    ) -> Self {
        Self {
            width,
            is_tty,
            color_enabled: color.should_render_color(is_tty, no_color_env),
            unicode,
            hyperlinks,
            motion,
            interactive,
            no_color_env,
        }
    }

    /// Attended terminal defaults at a width.
    pub fn tty(width: usize) -> Self {
        Self::explicit(
            width,
            true,
            ColorPolicy::Auto,
            false,
            true,
            false,
            true,
            true,
        )
    }

    /// Piped/redirected defaults: no colour, ASCII, no motion, non-interactive.
    pub fn piped(width: usize) -> Self {
        Self {
            width,
            is_tty: false,
            color_enabled: false,
            unicode: false,
            hyperlinks: false,
            motion: false,
            interactive: false,
            no_color_env: false,
        }
    }

    pub fn width_category(&self) -> WidthCategory {
        if self.width < 60 {
            WidthCategory::Narrow
        } else if self.width <= 100 {
            WidthCategory::Normal
        } else {
            WidthCategory::Wide
        }
    }

    pub fn is_narrow(&self) -> bool {
        self.width_category() == WidthCategory::Narrow
    }

    pub fn is_wide(&self) -> bool {
        self.width_category() == WidthCategory::Wide
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_color(mut self, enabled: bool) -> Self {
        self.color_enabled = enabled;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthCategory {
    Narrow,
    Normal,
    Wide,
}
