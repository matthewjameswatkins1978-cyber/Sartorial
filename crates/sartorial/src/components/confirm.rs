#[cfg(feature = "interactive")]
use crate::config::Config;
#[cfg(feature = "interactive")]
use crate::interaction::{read_key, TerminalGuard};
use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::TerminalRenderer;
#[cfg(feature = "interactive")]
use sartorial_core::KeyTrigger;
use sartorial_core::{Action, Block, Document, Presentable};
use serde::{Deserialize, Serialize};
#[cfg(feature = "interactive")]
use std::io::stdout;
use std::io::{self, Write};
#[cfg(feature = "interactive")]
use std::time::Duration;

/// Explicit result of a confirmation prompt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmOutcome {
    /// Explicitly accepted by user.
    Confirmed,
    /// Explicitly rejected by user.
    Denied,
    /// Cancelled via Esc or Ctrl+C.
    Cancelled,
    /// Denied automatically because environment was non-interactive and no fallback was authorized.
    NonInteractiveDenied,
    /// Resolved via explicitly authorized non-interactive fallback.
    NonInteractiveFallback(bool),
}

impl ConfirmOutcome {
    /// Convenience helper returning true only if confirmed or resolved by true fallback.
    pub fn is_confirmed(&self) -> bool {
        matches!(self, Self::Confirmed | Self::NonInteractiveFallback(true))
    }
}

/// Confirmation prompt adhering to Biscuit Logic interaction authority and fail-closed safety.
///
/// Live prompting requires the `interactive` feature; static display and
/// fail-closed fallback resolution always work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Confirm {
    pub prompt: String,
    pub default_value: bool,
    pub non_interactive_fallback: Option<bool>,
}

impl Confirm {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            default_value: true,
            non_interactive_fallback: None,
        }
    }

    /// Set default value chosen when Enter is pressed interactively.
    pub fn with_default(mut self, default_value: bool) -> Self {
        self.default_value = default_value;
        self
    }

    /// Explicitly authorize a fallback value when executed non-interactively.
    /// Without this, non-interactive execution will strictly fail-closed (`NonInteractiveDenied`).
    pub fn with_non_interactive_fallback(mut self, fallback: bool) -> Self {
        self.non_interactive_fallback = Some(fallback);
        self
    }

    /// Run confirmation using default configuration.
    #[cfg(feature = "interactive")]
    pub fn prompt(&self) -> io::Result<ConfirmOutcome> {
        self.prompt_with_config(&Config::default())
    }

    /// Run confirmation using the single authority of the provided configuration.
    #[cfg(feature = "interactive")]
    pub fn prompt_with_config(&self, config: &Config) -> io::Result<ConfirmOutcome> {
        if !config.is_interactive() {
            return match self.non_interactive_fallback {
                Some(fb) => Ok(ConfirmOutcome::NonInteractiveFallback(fb)),
                None => Ok(ConfirmOutcome::NonInteractiveDenied),
            };
        }

        let ctx = RenderContext::detect().with_config(config.clone());
        self.render_human(&ctx, &mut stdout())?;
        stdout().flush()?;

        let _guard = TerminalGuard::enter()?;
        loop {
            if let Some(key) = read_key(Duration::from_millis(200))? {
                match key {
                    KeyTrigger::Enter => {
                        let outcome = if self.default_value {
                            ConfirmOutcome::Confirmed
                        } else {
                            ConfirmOutcome::Denied
                        };
                        writeln!(stdout())?;
                        return Ok(outcome);
                    }
                    KeyTrigger::Char('y') | KeyTrigger::Char('Y') => {
                        writeln!(stdout())?;
                        return Ok(ConfirmOutcome::Confirmed);
                    }
                    KeyTrigger::Char('n') | KeyTrigger::Char('N') => {
                        writeln!(stdout())?;
                        return Ok(ConfirmOutcome::Denied);
                    }
                    KeyTrigger::Esc => {
                        writeln!(stdout())?;
                        return Ok(ConfirmOutcome::Cancelled);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Resolve without terminal interaction (always available).
    pub fn resolve_non_interactive(&self) -> ConfirmOutcome {
        match self.non_interactive_fallback {
            Some(fb) => ConfirmOutcome::NonInteractiveFallback(fb),
            None => ConfirmOutcome::NonInteractiveDenied,
        }
    }

    fn hint(&self, ctx: &RenderContext) -> String {
        let (open, close) = ctx.style.key_delimiters();
        let inner = if self.default_value { "Y/n" } else { "y/N" };
        format!("{open}{inner}{close}")
    }

    fn hint_text(&self) -> String {
        if self.default_value {
            "Confirm [Y/n]".to_string()
        } else {
            "Confirm [y/N]".to_string()
        }
    }
}

impl Presentable for Confirm {
    fn to_document(&self) -> Document {
        // Presented as prompt plus its key-hint action for Markdown targets.
        let action = Action::new('y', "confirm", self.hint_text());
        Document::new()
            .push(Block::Summary {
                text: self.prompt.clone(),
            })
            .push(Block::Actions {
                actions: vec![action],
            })
    }
}

impl RenderHuman for Confirm {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::write_styled(
            out,
            ctx.style.section_style(),
            &self.prompt,
            ctx.color_enabled,
        )?;
        write!(out, " ")?;
        let hint = self.hint(ctx);
        TerminalRenderer::write_styled(out, ctx.style.key_char_style(), &hint, ctx.color_enabled)?;
        write!(out, " ")
    }
}

impl RenderPlain for Confirm {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        write!(out, "{} {} ", self.prompt, self.hint(ctx))
    }
}
