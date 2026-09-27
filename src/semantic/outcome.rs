use super::action::Action;
use super::evidence::Evidence;
use super::fact::Fact;
use super::notice::Notice;
use super::status::Status;
use serde::{Deserialize, Serialize};

/// High-level semantic result representing the single authority of application truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    /// Overall operational status.
    pub status: Status,
    /// Title or headline (e.g. "Environment", "Clippy Check").
    pub title: String,
    /// Brief summary of what occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Structured facts / attributes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facts: Vec<Fact>,
    /// Bounded evidence or references.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<Evidence>,
    /// Any warnings or notices encountered.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Notice>,
    /// Actionable next steps or commands.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
    /// Optional deeper details (for progressive disclosure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl Outcome {
    /// Create a new outcome with status and title.
    pub fn new(status: Status, title: impl Into<String>) -> Self {
        Self {
            status,
            title: title.into(),
            summary: None,
            facts: Vec::new(),
            evidence: Vec::new(),
            warnings: Vec::new(),
            actions: Vec::new(),
            details: None,
        }
    }

    /// Set summary.
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Add a fact.
    pub fn with_fact(mut self, fact: Fact) -> Self {
        self.facts.push(fact);
        self
    }

    /// Add a key-value fact directly.
    pub fn fact(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.with_fact(Fact::new(name, value))
    }

    /// Add an evidence item.
    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    /// Add a warning or notice.
    pub fn with_warning(mut self, warning: Notice) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add an available action.
    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    /// Attach progressive disclosure details.
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

use crate::components::action_bar::ActionBar;
use crate::components::key_value::KeyValueList;
use crate::components::notice::NoticeView;
use crate::components::section::Section;
use crate::components::title::Title;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use std::io::{self, Write};

impl RenderHuman for Outcome {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_human(ctx, out)?;
        writeln!(out)?;

        let section = Section::new("Status").with_status(self.status);
        section.render_human(ctx, out)?;

        if let Some(ref sum) = self.summary {
            writeln!(out)?;
            HumanRenderer::write_styled(out, HumanRenderer::value_style(), sum, ctx.color_enabled)?;
            writeln!(out)?;
        }

        if !self.facts.is_empty() {
            writeln!(out)?;
            let kv = KeyValueList::from_facts(self.facts.clone());
            kv.render_human(ctx, out)?;
        }

        if !self.evidence.is_empty() {
            writeln!(out)?;
            for ev in &self.evidence {
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
            }
        }

        if !self.warnings.is_empty() {
            writeln!(out)?;
            for w in &self.warnings {
                let nv = NoticeView::new(w.clone());
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

impl RenderPlain for Outcome {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_plain(ctx, out)?;
        writeln!(out)?;

        let section = Section::new("Status").with_status(self.status);
        section.render_plain(ctx, out)?;

        if let Some(ref sum) = self.summary {
            writeln!(out, "\n{sum}")?;
        }

        if !self.facts.is_empty() {
            writeln!(out)?;
            let kv = KeyValueList::from_facts(self.facts.clone());
            kv.render_plain(ctx, out)?;
        }

        if !self.evidence.is_empty() {
            writeln!(out)?;
            for ev in &self.evidence {
                if let Some(ref loc) = ev.location {
                    writeln!(out, "{loc}")?;
                }
                writeln!(out, "{}", ev.summary)?;
            }
        }

        if !self.warnings.is_empty() {
            writeln!(out)?;
            for w in &self.warnings {
                let nv = NoticeView::new(w.clone());
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

use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};

#[derive(Serialize)]
struct AgentOutcomeRepresentation<'a> {
    schema_version: &'static str,
    status: Status,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: &'a Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    facts: &'a Vec<Fact>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    evidence: &'a Vec<Evidence>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    warnings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    next_actions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: &'a Option<String>,
}

impl RenderAgent for Outcome {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentOutcomeRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            status: self.status,
            title: &self.title,
            summary: &self.summary,
            facts: &self.facts,
            evidence: &self.evidence,
            warnings: self.warnings.iter().map(|w| w.message.clone()).collect(),
            next_actions: self.actions.iter().map(|a| a.id.clone()).collect(),
            details: &self.details,
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}
