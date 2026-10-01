use crate::config::{ColorChoice, Config};
use crate::render::target::RenderTarget;
use sartorial_core::{Capabilities, ColorPolicy, Preset, ResolvedStyle, SymbolMode};
use std::io::IsTerminal;

/// Width categorization according to Biscuit Logic responsiveness guidelines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthCategory {
    Narrow,
    Normal,
    Wide,
}

/// Rendering context: detected capabilities plus resolved style.
///
/// Detection happens once here (terminal width, TTY, `NO_COLOR`); the
/// deterministic [`Capabilities`] and [`ResolvedStyle`] are then passed
/// into core, which never sniffs the environment while rendering.
#[derive(Debug, Clone)]
pub struct RenderContext {
    pub target: RenderTarget,
    pub width: usize,
    pub config: Config,
    pub color_enabled: bool,
    pub symbols: SymbolMode,
    pub style: ResolvedStyle,
    pub caps: Capabilities,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::detect()
    }
}

impl RenderContext {
    /// Automatically detect terminal capabilities and dimensions.
    ///
    /// Detection reads the **stdout** TTY: colour, symbols, and width describe
    /// where results land. Live progress is different — spinners draw on
    /// **stderr**, so progress code sniffs the stderr TTY instead. The split
    /// lets piped results stay clean while an attended terminal still
    /// animates, and vice versa.
    pub fn detect() -> Self {
        let is_tty = std::io::stdout().is_terminal();
        let config = Config::default();
        Self::from_config_with_tty(config, is_tty, RenderTarget::Human)
    }

    /// Build a context from explicit configuration and TTY state.
    /// Deterministic: no environment sniffing beyond the given `is_tty`.
    pub fn from_config_with_tty(config: Config, is_tty: bool, target: RenderTarget) -> Self {
        let width = config.width.unwrap_or_else(detect_width);
        let no_color_env = ColorChoice::no_color_env_present();
        let style = config.resolve_style(is_tty);
        let color_enabled = style.color_enabled;
        let symbols = style.symbols;
        // Unicode safety follows the resolved symbol choice: an explicit
        // Unicode request survives non-TTY detection (tests, pipes with
        // forced glyphs), while Auto still tracks the TTY.
        let unicode = symbols == SymbolMode::Unicode;
        let mut caps = Capabilities::explicit(
            width,
            is_tty,
            map_color_policy(config.color),
            no_color_env,
            unicode,
            false,
            config.motion.should_animate(
                is_tty,
                target.is_agent(),
                target.is_plain(),
                config.accessibility.is_reduced_motion(),
            ),
            config
                .interactive
                .is_interactive(std::io::stdin().is_terminal(), is_tty),
        );
        // Plain/agent targets force pipe-safe capabilities even on a TTY,
        // so tests and redirects stay deterministic.
        if target.is_plain() || target.is_agent() {
            caps.color_enabled = false;
            caps.motion = false;
        }
        if target.is_plain() {
            caps.unicode = false;
        }
        let mut ctx = Self {
            target,
            width,
            config,
            color_enabled,
            symbols,
            style,
            caps,
        };
        ctx.apply_target_overrides();
        ctx
    }

    /// Context explicitly configured for pipe-safe plain output.
    pub fn plain() -> Self {
        Self::plain_preset(Preset::House)
    }

    /// Pipe-safe plain context for an explicit preset: the layout grammar
    /// (casing, markers, density) is preserved, ANSI and animation are off.
    pub fn plain_preset(preset: Preset) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        let config = Config::default().with_preset(preset);
        Self::from_config_with_tty(config, is_tty, RenderTarget::Plain)
    }

    /// Human terminal context for a preset with the default motion policy.
    pub fn human(preset: Preset) -> Self {
        Self::human_motion(preset, crate::motion::MotionMode::Auto)
    }

    /// Human terminal context for a preset with an explicit motion policy
    /// (use `MotionMode::Never` for animation-free output and tests).
    pub fn human_motion(preset: Preset, motion: crate::motion::MotionMode) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        let config = Config::default().with_preset(preset).with_motion(motion);
        Self::from_config_with_tty(config, is_tty, RenderTarget::Human)
    }

    /// Context explicitly configured for structured agent output.
    pub fn agent() -> Self {
        let is_tty = std::io::stdout().is_terminal();
        Self::from_config_with_tty(Config::default(), is_tty, RenderTarget::Agent)
    }

    /// Markdown context for a preset: grammar preserved, no ANSI.
    pub fn markdown(preset: Preset) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        let config = Config::default().with_preset(preset);
        Self::from_config_with_tty(config, is_tty, RenderTarget::Markdown)
    }

    /// Re-apply the invariants of the current target after any mutation.
    fn apply_target_overrides(&mut self) {
        if self.target.is_plain() || self.target.is_agent() {
            self.color_enabled = false;
            self.style.color_enabled = false;
            self.caps.color_enabled = false;
        }
        if self.target.is_plain() {
            self.style.force_plain();
            self.symbols = SymbolMode::Ascii;
            self.caps.unicode = false;
        }
        if self.target.is_agent() {
            self.caps.motion = false;
        }
    }

    /// Set an explicit configuration.
    pub fn with_config(self, config: Config) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        let target = self.target;
        Self::from_config_with_tty(config, is_tty, target)
    }

    /// Set an explicit target.
    pub fn with_target(mut self, target: RenderTarget) -> Self {
        self.target = target;
        self.apply_target_overrides();
        self.caps.motion = self.caps.motion && !target.is_plain() && !target.is_agent();
        if target.is_plain() {
            self.caps.color_enabled = false;
            self.caps.unicode = false;
        }
        if target.is_agent() {
            self.caps.color_enabled = false;
        }
        self
    }

    /// Set an explicit width for testing or formatting.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self.caps.width = width;
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

fn map_color_policy(choice: ColorChoice) -> ColorPolicy {
    match choice {
        ColorChoice::Auto => ColorPolicy::Auto,
        ColorChoice::Always => ColorPolicy::Always,
        ColorChoice::Never => ColorPolicy::Never,
    }
}

fn detect_width() -> usize {
    #[cfg(feature = "terminal")]
    {
        crossterm::terminal::size()
            .map(|(w, _)| w as usize)
            .unwrap_or(80)
    }
    #[cfg(not(feature = "terminal"))]
    {
        std::env::var("COLUMNS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(80)
    }
}
