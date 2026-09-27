use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use std::io::{self, Write};

/// Typo suggestion presentation component.
/// Never silently executes suggested commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypoSuggestion {
    pub unknown: String,
    pub candidates: Vec<String>,
}

impl TypoSuggestion {
    pub fn new(unknown: impl Into<String>, candidates: Vec<String>) -> Self {
        Self {
            unknown: unknown.into(),
            candidates,
        }
    }

    pub fn single(unknown: impl Into<String>, candidate: impl Into<String>) -> Self {
        Self {
            unknown: unknown.into(),
            candidates: vec![candidate.into()],
        }
    }
}

impl RenderHuman for TypoSuggestion {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        HumanRenderer::write_styled(
            out,
            HumanRenderer::title_style(anstyle::AnsiColor::Red),
            &format!("Unknown command: {}", self.unknown),
            ctx.color_enabled,
        )?;
        writeln!(out)?;

        if !self.candidates.is_empty() {
            writeln!(out)?;
            HumanRenderer::write_styled(
                out,
                HumanRenderer::section_style(),
                "Did you mean:",
                ctx.color_enabled,
            )?;
            writeln!(out)?;
            for cand in &self.candidates {
                write!(out, "  ")?;
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::key_char_style(ctx.config.accent),
                    cand,
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
            }
        }
        Ok(())
    }
}

impl RenderPlain for TypoSuggestion {
    fn render_plain(&self, _ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "Unknown command: {}", self.unknown)?;
        if !self.candidates.is_empty() {
            writeln!(out, "\nDid you mean:")?;
            for cand in &self.candidates {
                writeln!(out, "  {cand}")?;
            }
        }
        Ok(())
    }
}
