use crate::config::{ColorChoice, Config};
use crate::render::target::RenderTarget;
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

/// Ready-to-use CLI arguments for applications integrating with `clap`.
///
/// Simply add `#[command(flatten)] pub sartorial: sartorial::clap_ext::SartorialArgs`
/// to your CLI struct.
#[derive(Debug, Clone, Args)]
pub struct SartorialArgs {
    /// Emit machine-readable JSON output for AI agents
    #[arg(long, conflicts_with = "plain")]
    pub json: bool,

    /// Emit plain pipe-safe text without ANSI colors or formatting
    #[arg(long, conflicts_with = "json")]
    pub plain: bool,

    /// Color policy: auto, always, never
    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorChoiceArg,
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

    /// Construct a matching `Config` from CLI flags.
    pub fn to_config(&self) -> Config {
        Config::new().with_color(self.color.into())
    }
}
