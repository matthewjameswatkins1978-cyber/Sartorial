use crate::components::action_bar::ActionBar;
use crate::components::key_value::KeyValueList;
use crate::components::notice::NoticeView;
use crate::components::status_badge::StatusBadge;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::receipt::Receipt;
use std::io::{self, Write};

impl RenderHuman for Receipt {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let badge = StatusBadge::new(self.status);
        badge.render_human(ctx, out)?;
        write!(out, " ")?;
        HumanRenderer::write_styled(out, ctx.style.title_style(), &self.title, ctx.color_enabled)?;
        writeln!(out)?;
        writeln!(out)?;

        if !self.changes.is_empty() {
            let mut kv = KeyValueList::new();
            for f in &self.changes {
                kv.add(&f.name, &f.value);
            }
            kv.render_human(ctx, out)?;
        }

        if !self.unchanged.is_empty() {
            writeln!(out)?;
            HumanRenderer::write_styled(
                out,
                ctx.style.muted_style(),
                "Unchanged:",
                ctx.color_enabled,
            )?;
            writeln!(out)?;
            let mut kv = KeyValueList::new();
            for f in &self.unchanged {
                kv.add(&f.name, &f.value);
            }
            kv.render_human(ctx, out)?;
        }

        for warn in &self.warnings {
            writeln!(out)?;
            let nv = NoticeView::new(warn.clone());
            nv.render_human(ctx, out)?;
        }

        if let Some(ref guidance) = self.guidance {
            writeln!(out)?;
            HumanRenderer::write_styled(out, ctx.style.value_style(), guidance, ctx.color_enabled)?;
            writeln!(out)?;
        }

        if let Some(ref handle) = self.evidence_handle {
            writeln!(out)?;
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                &format!("Evidence reference: {handle}"),
                ctx.color_enabled,
            )?;
            writeln!(out)?;
        }

        if !self.actions.is_empty() {
            writeln!(out)?;
            let bar = ActionBar::from_actions(self.actions.clone());
            bar.render_human(ctx, out)?;
        }

        Ok(())
    }
}

impl RenderPlain for Receipt {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let badge = StatusBadge::new(self.status);
        badge.render_plain(ctx, out)?;
        writeln!(out, " {}", self.title)?;
        writeln!(out)?;

        if !self.changes.is_empty() {
            let mut kv = KeyValueList::new();
            for f in &self.changes {
                kv.add(&f.name, &f.value);
            }
            kv.render_plain(ctx, out)?;
        }

        if !self.unchanged.is_empty() {
            writeln!(out, "\nUnchanged:")?;
            let mut kv = KeyValueList::new();
            for f in &self.unchanged {
                kv.add(&f.name, &f.value);
            }
            kv.render_plain(ctx, out)?;
        }

        for warn in &self.warnings {
            writeln!(out)?;
            let nv = NoticeView::new(warn.clone());
            nv.render_plain(ctx, out)?;
        }

        if let Some(ref guidance) = self.guidance {
            writeln!(out, "\n{guidance}")?;
        }

        if let Some(ref handle) = self.evidence_handle {
            writeln!(out, "\nEvidence reference: {handle}")?;
        }

        if !self.actions.is_empty() {
            writeln!(out)?;
            let bar = ActionBar::from_actions(self.actions.clone());
            bar.render_plain(ctx, out)?;
        }

        Ok(())
    }
}
