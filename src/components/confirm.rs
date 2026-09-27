use crate::interaction::{read_key, TerminalGuard};
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::action::KeyTrigger;
use std::io::{self, stdout, IsTerminal, Write};
use std::time::Duration;

/// Interactive or non-interactive confirmation prompt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Confirm {
    pub prompt: String,
    pub default_value: bool,
}

impl Confirm {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            default_value: true,
        }
    }

    pub fn with_default(mut self, default_value: bool) -> Self {
        self.default_value = default_value;
        self
    }

    /// Prompt interactively if supported, or fall back to default deterministically.
    pub fn prompt_interactive(&self) -> io::Result<bool> {
        if !stdout().is_terminal() {
            return Ok(self.default_value);
        }

        let ctx = RenderContext::detect();
        self.render_human(&ctx, &mut stdout())?;
        stdout().flush()?;

        let _guard = TerminalGuard::enter()?;
        loop {
            if let Some(key) = read_key(Duration::from_millis(200))? {
                match key {
                    KeyTrigger::Enter => return Ok(self.default_value),
                    KeyTrigger::Char('y') | KeyTrigger::Char('Y') => return Ok(true),
                    KeyTrigger::Char('n') | KeyTrigger::Char('N') | KeyTrigger::Esc => {
                        return Ok(false)
                    }
                    _ => {}
                }
            }
        }
    }
}

impl RenderHuman for Confirm {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        HumanRenderer::write_styled(
            out,
            HumanRenderer::section_style(),
            &self.prompt,
            ctx.color_enabled,
        )?;
        write!(out, " ")?;
        let hint = if self.default_value { "[Y/n]" } else { "[y/N]" };
        HumanRenderer::write_styled(
            out,
            HumanRenderer::key_char_style(ctx.config.accent),
            hint,
            ctx.color_enabled,
        )?;
        write!(out, " ")
    }
}

impl RenderPlain for Confirm {
    fn render_plain(&self, _ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let hint = if self.default_value { "[Y/n]" } else { "[y/N]" };
        write!(out, "{} {hint} ", self.prompt)
    }
}
