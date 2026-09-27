use crate::components::action_bar::ActionBar;
use crate::components::section::Section;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::action::Action;
use crate::semantic::evidence::Evidence;
use std::io::{self, Write};

/// Progressive disclosure detail view for deep evidence, logs, or diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetailView {
    pub title: String,
    pub evidence: Evidence,
    pub actions: Vec<Action>,
}

impl DetailView {
    pub fn new(title: impl Into<String>, evidence: Evidence) -> Self {
        Self {
            title: title.into(),
            evidence,
            actions: Vec::new(),
        }
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
}

impl RenderHuman for DetailView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let sec = Section::new(&self.title);
        sec.render_human(ctx, out)?;

        if let Some(ref loc) = self.evidence.location {
            HumanRenderer::write_styled(out, HumanRenderer::muted_style(), loc, ctx.color_enabled)?;
            writeln!(out)?;
        }

        HumanRenderer::write_styled(
            out,
            HumanRenderer::value_style(),
            &self.evidence.summary,
            ctx.color_enabled,
        )?;
        writeln!(out)?;

        if let Some(ref handle) = self.evidence.handle {
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                &format!("Evidence Handle: {handle}"),
                ctx.color_enabled,
            )?;
            writeln!(out)?;
        }

        if let Some(ref details) = self.evidence.details {
            writeln!(out)?;
            HumanRenderer::write_rule(out, ctx, ctx.width.min(60))?;
            for line in details.lines() {
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::muted_style(),
                    line,
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
            }
            HumanRenderer::write_rule(out, ctx, ctx.width.min(60))?;
        }

        if !self.actions.is_empty() {
            writeln!(out)?;
            let action_bar = ActionBar::from_actions(self.actions.clone());
            action_bar.render_human(ctx, out)?;
        }

        Ok(())
    }
}

impl RenderPlain for DetailView {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let sec = Section::new(&self.title);
        sec.render_plain(ctx, out)?;

        if let Some(ref loc) = self.evidence.location {
            writeln!(out, "{loc}")?;
        }
        writeln!(out, "{}", self.evidence.summary)?;

        if let Some(ref handle) = self.evidence.handle {
            writeln!(out, "Evidence Handle: {handle}")?;
        }

        if let Some(ref details) = self.evidence.details {
            writeln!(out)?;
            for line in details.lines() {
                writeln!(out, "{line}")?;
            }
        }

        if !self.actions.is_empty() {
            writeln!(out)?;
            let action_bar = ActionBar::from_actions(self.actions.clone());
            action_bar.render_plain(ctx, out)?;
        }

        Ok(())
    }
}
