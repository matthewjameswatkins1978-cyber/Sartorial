use crate::config::{ColorChoice, Config};
use crate::motion::MotionMode;
use crate::render::target::RenderTarget;
use crate::style::Preset;
use crate::verbosity::Verbosity;
use clap::{Args, ValueEnum};

/// Output format choice for CLI flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum OutputFormat {
    /// Human-facing styled terminal presentation
    #[default]
    Human,
    /// Pipe-safe plain text (no ANSI sequences)
    Plain,
    /// Machine-readable structured JSON
    Json,
}

/// Visual preset choice for CLI flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum StyleArg {
    /// Biscuit Logic default: quiet, cool, precise, restrained
    #[default]
    House,
    /// Formal, nearly monochrome, elegant, controlled
    BlackTie,
    /// Dense, fast, practical, operator-focused
    Workwear,
    /// Slightly more expressive, roomier, presentation-friendly
    Studio,
}

impl From<StyleArg> for Preset {
    fn from(val: StyleArg) -> Self {
        match val {
            StyleArg::House => Preset::House,
            StyleArg::BlackTie => Preset::BlackTie,
            StyleArg::Workwear => Preset::Workwear,
            StyleArg::Studio => Preset::Studio,
        }
    }
}

/// Motion mode choice for CLI flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum MotionArg {
    /// Animate only when attached to an attended interactive TTY
    #[default]
    Auto,
    /// Force animations on
    Always,
    /// Suppress all animations and spinners
    Never,
}

impl From<MotionArg> for MotionMode {
    fn from(val: MotionArg) -> Self {
        match val {
            MotionArg::Auto => MotionMode::Auto,
            MotionArg::Always => MotionMode::Always,
            MotionArg::Never => MotionMode::Never,
        }
    }
}

/// Ready-to-use CLI arguments for applications integrating with `clap`.
///
/// Simply add `#[command(flatten)] pub sartorial: sartorial::clap_ext::SartorialArgs`
/// to your CLI struct.
#[derive(Debug, Clone, Args)]
pub struct SartorialArgs {
    /// Emit machine-readable JSON output for AI agents
    #[arg(long, global = true, conflicts_with = "plain")]
    pub json: bool,

    /// Emit plain pipe-safe text without ANSI colors or formatting
    #[arg(long, global = true, conflicts_with = "json")]
    pub plain: bool,

    /// Visual presentation preset
    #[arg(
        long,
        global = true,
        value_enum,
        default_value = "house",
        alias = "preset"
    )]
    pub style: StyleArg,

    /// Motion and progress animation policy
    #[arg(long, global = true, value_enum, default_value = "auto")]
    pub motion: MotionArg,

    /// Color policy: auto, always, never
    #[arg(long, global = true, value_enum, default_value = "auto")]
    pub color: ColorChoiceArg,

    /// Essential output and critical errors only
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Detailed informative output
    #[arg(short, long, global = true, conflicts_with = "quiet")]
    pub verbose: bool,

    /// Exhaustive internal diagnostic traces
    #[arg(long, global = true, conflicts_with = "quiet")]
    pub debug: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum ColorChoiceArg {
    #[default]
    Auto,
    Always,
    Never,
}

impl From<ColorChoiceArg> for ColorChoice {
    fn from(val: ColorChoiceArg) -> Self {
        match val {
            ColorChoiceArg::Auto => ColorChoice::Auto,
            ColorChoiceArg::Always => ColorChoice::Always,
            ColorChoiceArg::Never => ColorChoice::Never,
        }
    }
}

impl SartorialArgs {
    /// Resolve the target render destination.
    pub fn target(&self) -> RenderTarget {
        if self.json {
            RenderTarget::Agent
        } else if self.plain {
            RenderTarget::Plain
        } else {
            RenderTarget::Human
        }
    }

    /// Resolve verbosity from CLI flags.
    pub fn verbosity(&self) -> Verbosity {
        if self.quiet {
            Verbosity::Quiet
        } else if self.debug {
            Verbosity::Debug
        } else if self.verbose {
            Verbosity::Verbose
        } else {
            Verbosity::Normal
        }
    }

    /// Construct a matching `Config` from CLI flags.
    pub fn to_config(&self) -> Config {
        Config::new()
            .with_preset(self.style.into())
            .with_motion(self.motion.into())
            .with_color(self.color.into())
            .with_verbosity(self.verbosity())
    }
}
