use crate::config::{Config, SymbolMode};
use crate::render::target::RenderTarget;
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

/// Rendering context providing display boundaries, capabilities, and settings.
#[derive(Debug, Clone)]
pub struct RenderContext {
    pub target: RenderTarget,
    pub width: usize,
    pub config: Config,
    pub color_enabled: bool,
    pub symbols: SymbolMode,
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
        let color_enabled = config.color.should_render_color(is_tty);
        let symbols = config.symbols.resolve(is_tty);
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
        }
    }

    /// Context explicitly configured for pipe-safe plain output.
    pub fn plain() -> Self {
        let mut ctx = Self::detect();
        ctx.target = RenderTarget::Plain;
        ctx.color_enabled = false;
        ctx.symbols = SymbolMode::Ascii;
        ctx
    }

    /// Context explicitly configured for structured agent output.
    pub fn agent() -> Self {
        let mut ctx = Self::detect();
        ctx.target = RenderTarget::Agent;
        ctx.color_enabled = false;
        ctx
    }

    /// Set an explicit configuration.
    pub fn with_config(mut self, config: Config) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        self.color_enabled = config.color.should_render_color(is_tty);
        self.symbols = config.symbols.resolve(is_tty);
        if let Some(w) = config.width {
            self.width = w;
        }
        self.config = config;
        self
    }

    /// Set an explicit target.
    pub fn with_target(mut self, target: RenderTarget) -> Self {
        self.target = target;
        if target.is_plain() {
            self.color_enabled = false;
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
}
