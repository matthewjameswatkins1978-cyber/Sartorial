use crate::config::{Config, SymbolMode};
use crate::render::target::RenderTarget;
use crate::style::ResolvedStyle;
use std::io::IsTerminal;

/// Width categorization according to Biscuit Logic responsiveness guidelines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthCategory {
    /// Terminal width < 60 columns. Optional detail should be pruned.
    Narrow,
    /// Terminal width 60..=100 columns. Balanced standard layout.
    Normal,
    /// Terminal width > 100 columns. Generous room for side-by-side data.
    Wide,
}

/// Rendering context providing display boundaries, capabilities, settings, and resolved style.
#[derive(Debug, Clone)]
pub struct RenderContext {
    pub target: RenderTarget,
    pub width: usize,
    pub config: Config,
    pub color_enabled: bool,
    pub symbols: SymbolMode,
    pub style: ResolvedStyle,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::detect()
    }
}

impl RenderContext {
    /// Automatically detect terminal capabilities and current terminal dimensions.
    pub fn detect() -> Self {
        let is_tty = std::io::stdout().is_terminal();
        let config = Config::default();
        let style = config.resolve_style(is_tty);
        let color_enabled = style.color_enabled;
        let symbols = style.symbols;
        let width = config.width.unwrap_or_else(|| {
            crossterm::terminal::size()
                .map(|(w, _)| w as usize)
                .unwrap_or(80)
        });

        Self {
            target: RenderTarget::Human,
            width,
            config,
            color_enabled,
            symbols,
            style,
        }
    }

    /// Context explicitly configured for pipe-safe plain output.
    pub fn plain() -> Self {
        let mut ctx = Self::detect();
        ctx.target = RenderTarget::Plain;
        ctx.color_enabled = false;
        ctx.symbols = SymbolMode::Ascii;
        ctx.style.color_enabled = false;
        ctx.style.symbols = SymbolMode::Ascii;
        ctx
    }

    /// Context explicitly configured for structured agent output.
    pub fn agent() -> Self {
        let mut ctx = Self::detect();
        ctx.target = RenderTarget::Agent;
        ctx.color_enabled = false;
        ctx.style.color_enabled = false;
        ctx
    }

    /// Set an explicit configuration.
    pub fn with_config(mut self, config: Config) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        self.style = config.resolve_style(is_tty);
        self.color_enabled = self.style.color_enabled;
        self.symbols = self.style.symbols;
        if let Some(w) = config.width {
            self.width = w;
        }
        self.config = config;
        self
    }

    /// Set an explicit target.
    pub fn with_target(mut self, target: RenderTarget) -> Self {
        self.target = target;
        if target.is_plain() || target.is_agent() {
            self.color_enabled = false;
            self.style.color_enabled = false;
        }
        if target.is_plain() {
            self.symbols = SymbolMode::Ascii;
            self.style.symbols = SymbolMode::Ascii;
        }
        self
    }

    /// Set an explicit width for testing or formatting.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Categorize current width into Narrow, Normal, or Wide.
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

    /// Single authority deciding whether motion/progress animation is permitted.
    ///
    /// Motion eligibility depends strictly on:
    /// - Progress output stream being a real attended TTY (`is_tty`)
    /// - `RenderTarget` being human (suppressed on Agent and Plain targets)
    /// - `AccessibilityMode` allowing motion (suppressed by reduced motion)
    /// - `MotionMode` policy
    /// - `InteractiveMode::Off` acting as an explicit veto
    ///
    /// NOTE: Does not require stdin to be a TTY, as progress streaming normally
    /// arrives via piped stdin to an attended terminal output.
    pub fn should_animate(&self, is_tty: bool) -> bool {
        if !is_tty {
            return false;
        }
        if self.config.interactive == crate::config::InteractiveMode::Off {
            return false;
        }
        let reduced_motion = self.config.accessibility.is_reduced_motion();
        self.config.motion.should_animate(
            is_tty,
            self.target.is_agent(),
            self.target.is_plain(),
            reduced_motion,
        )
    }
}
