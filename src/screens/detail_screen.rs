use crate::components::action_bar::ActionBar;
use crate::components::key_value::KeyValueList;
use crate::components::notice::NoticeView;
use crate::components::section::Section;
use crate::components::title::Title;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::action::Action;
use crate::semantic::evidence::Evidence;
use crate::semantic::fact::Fact;
use crate::semantic::notice::Notice;
use crate::semantic::status::Status;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// An opinionated Detail Screen for progressive disclosure of diagnostics or entity state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetailScreen {
    pub title: String,
    pub status: Status,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facts: Vec<Fact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Evidence>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notices: Vec<Notice>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
}

impl DetailScreen {
    pub fn new(title: impl Into<String>, status: Status) -> Self {
        Self {
            title: title.into(),
            status,
            facts: Vec::new(),
            evidence: None,
            notices: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn fact(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.facts.push(Fact::new(name, value));
        self
    }

    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.evidence = Some(evidence);
        self
    }

    pub fn notice(mut self, notice: Notice) -> Self {
        self.notices.push(notice);
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
}

impl RenderHuman for DetailScreen {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_human(ctx, out)?;
        writeln!(out)?;

        let section = Section::new("Status").with_status(self.status);
        section.render_human(ctx, out)?;

        if !self.facts.is_empty() {
            writeln!(out)?;
            let kv = KeyValueList::from_facts(self.facts.clone());
            kv.render_human(ctx, out)?;
        }

        if let Some(ref ev) = self.evidence {
            writeln!(out)?;
            let ev_sec = Section::new("Evidence");
            ev_sec.render_human(ctx, out)?;

            if let Some(ref loc) = ev.location {
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::muted_style(),
                    loc,
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
            }

            HumanRenderer::write_styled(
                out,
                HumanRenderer::value_style(),
                &ev.summary,
                ctx.color_enabled,
            )?;
            writeln!(out)?;

            if let Some(ref handle) = ev.handle {
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::muted_style(),
                    &format!("Handle: {handle}"),
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
            }

            if let Some(ref det) = ev.details {
                writeln!(out)?;
                HumanRenderer::write_rule(out, ctx, ctx.width.min(60))?;
                for line in det.lines() {
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
        }

        if !self.notices.is_empty() {
            writeln!(out)?;
            for n in &self.notices {
                let nv = NoticeView::new(n.clone());
                nv.render_human(ctx, out)?;
            }
        }

        if !self.actions.is_empty() {
            writeln!(out)?;
            let ab = ActionBar::from_actions(self.actions.clone());
            ab.render_human(ctx, out)?;
        }

        Ok(())
    }
}

impl RenderPlain for DetailScreen {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_plain(ctx, out)?;
        writeln!(out)?;

        let section = Section::new("Status").with_status(self.status);
        section.render_plain(ctx, out)?;

        if !self.facts.is_empty() {
            writeln!(out)?;
            let kv = KeyValueList::from_facts(self.facts.clone());
            kv.render_plain(ctx, out)?;
        }

        if let Some(ref ev) = self.evidence {
            writeln!(out)?;
            let ev_sec = Section::new("Evidence");
            ev_sec.render_plain(ctx, out)?;

            if let Some(ref loc) = ev.location {
                writeln!(out, "{loc}")?;
            }
            writeln!(out, "{}", ev.summary)?;
            if let Some(ref handle) = ev.handle {
                writeln!(out, "Handle: {handle}")?;
            }
            if let Some(ref det) = ev.details {
                writeln!(out)?;
                for line in det.lines() {
                    writeln!(out, "{line}")?;
                }
            }
        }

        if !self.notices.is_empty() {
            writeln!(out)?;
            for n in &self.notices {
                let nv = NoticeView::new(n.clone());
                nv.render_plain(ctx, out)?;
            }
        }

        if !self.actions.is_empty() {
            writeln!(out)?;
            let ab = ActionBar::from_actions(self.actions.clone());
            ab.render_plain(ctx, out)?;
        }

        Ok(())
    }
}
