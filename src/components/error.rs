use crate::components::action_bar::ActionBar;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::error::ErrorModel;
use std::io::{self, Write};

/// Error display component conforming to BL error design:
/// WHAT HAPPENED? WHY? WHAT CAN I DO NEXT?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorView {
    pub error: ErrorModel,
}

impl ErrorView {
    pub fn new(error: ErrorModel) -> Self {
        Self { error }
    }
}

impl RenderHuman for ErrorView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        // 1. WHAT HAPPENED? (Strong title; casing follows the preset grammar.)
        let what = match ctx.style.title_case {
            crate::style::TitleCase::Upper => self.error.what.to_uppercase(),
            crate::style::TitleCase::Preserve => self.error.what.clone(),
        };
        let err_style = anstyle::Style::new()
            .fg_color(Some(anstyle::AnsiColor::Red.into()))
            .effects(anstyle::Effects::BOLD);

        HumanRenderer::write_styled(out, err_style, &what, ctx.color_enabled)?;
        writeln!(out)?;
        writeln!(out)?;

        // 2. WHY? (Established cause or preserved uncertainty)
        if let Some(ref why) = self.error.why {
            HumanRenderer::write_styled(out, ctx.style.value_style(), why, ctx.color_enabled)?;
            writeln!(out)?;
        } else {
            HumanRenderer::write_styled(
                out,
                ctx.style.muted_style(),
                "Cause undetermined (no conclusive root cause established).",
                ctx.color_enabled,
            )?;
            writeln!(out)?;
        }

        // Supporting evidence (if any)
        if !self.error.evidence.is_empty() {
            writeln!(out)?;
            for ev in &self.error.evidence {
                if let Some(ref loc) = ev.location {
                    HumanRenderer::write_styled(
                        out,
                        ctx.style.muted_style(),
                        loc,
                        ctx.color_enabled,
                    )?;
                    writeln!(out)?;
                }
                HumanRenderer::write_styled(
                    out,
                    ctx.style.value_style(),
                    &ev.summary,
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
                if let Some(ref handle) = ev.handle {
                    HumanRenderer::write_styled(
                        out,
                        ctx.style.muted_style(),
                        &format!("Ref: {handle}"),
                        ctx.color_enabled,
                    )?;
                    writeln!(out)?;
                }
            }
        }

        // 3. WHAT CAN I DO NEXT? (ActionBar)
        if !self.error.next_actions.is_empty() {
            writeln!(out)?;
            let action_bar = ActionBar::from_actions(self.error.next_actions.clone());
            action_bar.render_human(ctx, out)?;
        }

        Ok(())
    }
}

impl RenderPlain for ErrorView {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let what = match ctx.style.title_case {
            crate::style::TitleCase::Upper => self.error.what.to_uppercase(),
            crate::style::TitleCase::Preserve => self.error.what.clone(),
        };
        writeln!(out, "{what}\n")?;

        if let Some(ref why) = self.error.why {
            writeln!(out, "{why}")?;
        } else {
            writeln!(
                out,
                "Cause undetermined (no conclusive root cause established)."
            )?;
        }

        if !self.error.evidence.is_empty() {
            writeln!(out)?;
            for ev in &self.error.evidence {
                if let Some(ref loc) = ev.location {
                    writeln!(out, "{loc}")?;
                }
                writeln!(out, "{}", ev.summary)?;
                if let Some(ref handle) = ev.handle {
                    writeln!(out, "Ref: {handle}")?;
                }
            }
        }

        if !self.error.next_actions.is_empty() {
            writeln!(out)?;
            let action_bar = ActionBar::from_actions(self.error.next_actions.clone());
            action_bar.render_plain(ctx, out)?;
        }

        Ok(())
    }
}
