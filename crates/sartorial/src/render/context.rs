use crate::config::{ColorChoice, Config};
use crate::hyperlink::Hyperlink;
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

#[derive(Debug, Clone, Copy)]
struct EnvironmentSnapshot {
    stdout_is_tty: bool,
    stdin_is_tty: bool,
    no_color_env: bool,
    width: usize,
    hyperlinks: bool,
}

impl EnvironmentSnapshot {
    fn detect() -> Self {
        Self {
            stdout_is_tty: std::io::stdout().is_terminal(),
            stdin_is_tty: std::io::stdin().is_terminal(),
            no_color_env: ColorChoice::no_color_env_present(),
            width: detect_width(),
            hyperlinks: Hyperlink::is_supported(),
        }
    }

    /// Deterministic synthetic environment for tests and explicit callers.
    fn explicit(is_tty: bool, width: usize) -> Self {
        Self {
            stdout_is_tty: is_tty,
            stdin_is_tty: is_tty,
            no_color_env: false,
            width,
            hyperlinks: false,
        }
    }
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
    environment: EnvironmentSnapshot,
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
        let environment = EnvironmentSnapshot::detect();
        Self::from_config_with_environment(
            Config::default(),
            RenderTarget::Human,
            environment,
        )
    }

    /// Build a context from explicit configuration and TTY state.
    ///
    /// This constructor is deterministic: it does not read NO_COLOR, terminal
    /// width, stdin TTY state, hyperlink support, or any other process state.
    /// Use detect() when live environment detection is desired.
    pub fn from_config_with_tty(config: Config, is_tty: bool, target: RenderTarget) -> Self {
        let width = config.width.unwrap_or(80);
        let environment = EnvironmentSnapshot::explicit(is_tty, width);
        Self::from_config_with_environment(config, target, environment)
    }

    fn from_config_with_environment(
        config: Config,
        target: RenderTarget,
        environment: EnvironmentSnapshot,
    ) -> Self {
        let width = config.width.unwrap_or(environment.width);
        let mut style = config.resolve_style_with_environment(
            environment.stdout_is_tty,
            environment.no_color_env,
        );
        let symbols = style.symbols;
        let unicode = symbols == SymbolMode::Unicode;
        let static_target = target.is_plain() || target.is_agent() || target.is_markdown();
        let mut caps = Capabilities::explicit(
            width,
            environment.stdout_is_tty,
            map_color_policy(config.color),
            environment.no_color_env,
            unicode,
            environment.hyperlinks,
            if static_target {
                false
            } else {
                config.motion.should_animate(
                    environment.stdout_is_tty,
                    false,
                    false,
                    config.accessibility.is_reduced_motion(),
                )
            },
            config
                .interactive
                .is_interactive(environment.stdin_is_tty, environment.stdout_is_tty),
        );
        // Accessibility and other resolved style policy is authoritative.
        caps.color_enabled = style.color_enabled;

        if static_target {
            caps.color_enabled = false;
            caps.motion = false;
            style.color_enabled = false;
        }
        if target.is_plain() {
            caps.unicode = false;
            style.force_plain();
        }

        let mut ctx = Self {
            target,
            width,
            config,
            color_enabled: style.color_enabled,
            symbols: style.symbols,
            style,
            caps,
            environment,
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
        let environment = EnvironmentSnapshot::detect();
        let config = Config::default().with_preset(preset);
        Self::from_config_with_environment(config, RenderTarget::Plain, environment)
    }

    /// Human terminal context for a preset with the default motion policy.
    pub fn human(preset: Preset) -> Self {
        Self::human_motion(preset, crate::motion::MotionMode::Auto)
    }

    /// Human terminal context for a preset with an explicit motion policy
    /// (use `MotionMode::Never` for animation-free output and tests).
    pub fn human_motion(preset: Preset, motion: crate::motion::MotionMode) -> Self {
        let environment = EnvironmentSnapshot::detect();
        let config = Config::default().with_preset(preset).with_motion(motion);
        Self::from_config_with_environment(config, RenderTarget::Human, environment)
    }

    /// Context explicitly configured for structured agent output.
    pub fn agent() -> Self {
        let environment = EnvironmentSnapshot::detect();
        Self::from_config_with_environment(Config::default(), RenderTarget::Agent, environment)
    }

    /// Markdown context for a preset: grammar preserved, no ANSI.
    pub fn markdown(preset: Preset) -> Self {
        let environment = EnvironmentSnapshot::detect();
        let config = Config::default().with_preset(preset);
        Self::from_config_with_environment(config, RenderTarget::Markdown, environment)
    }

    /// Re-apply the invariants of the current target after any mutation.
    fn apply_target_overrides(&mut self) {
        let static_target =
            self.target.is_plain() || self.target.is_agent() || self.target.is_markdown();
        if static_target {
            self.color_enabled = false;
            self.style.color_enabled = false;
            self.caps.color_enabled = false;
            self.caps.motion = false;
        }
        if self.target.is_plain() {
            self.style.force_plain();
            self.symbols = SymbolMode::Ascii;
            self.caps.unicode = false;
        }
    }

    /// Set an explicit configuration without re-sniffing the process environment.
    pub fn with_config(self, config: Config) -> Self {
        Self::from_config_with_environment(config, self.target, self.environment)
    }

    /// Set an explicit target by re-resolving from the original environment
    /// snapshot. Target changes are therefore reversible and order-independent.
    pub fn with_target(self, target: RenderTarget) -> Self {
        Self::from_config_with_environment(self.config, target, self.environment)
    }

    /// Set an explicit width for testing or formatting.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self.config.width = Some(width);
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
        if !is_tty
            || self.target.is_plain()
            || self.target.is_agent()
            || self.target.is_markdown()
        {
            return false;
        }
        if self.config.interactive == crate::config::InteractiveMode::Off {
            return false;
        }
        let reduced_motion = self.config.accessibility.is_reduced_motion();
        self.config
            .motion
            .should_animate(is_tty, false, false, reduced_motion)
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
