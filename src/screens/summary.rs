use crate::components::action_bar::ActionBar;
use crate::components::key_value::KeyValueList;
use crate::components::notice::NoticeView;
use crate::components::section::Section;
use crate::components::table::TableView;
use crate::components::title::Title;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::plain::PlainRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::action::Action;
use crate::semantic::fact::Fact;
use crate::semantic::notice::Notice;
use crate::semantic::outcome::Outcome;
use crate::semantic::status::Status;
use crate::semantic::table::TableModel;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// An opinionated Summary Screen presenting high-level status, metrics/tables, and actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryScreen {
    pub title: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facts: Vec<Fact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableModel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notices: Vec<Notice>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
}

impl SummaryScreen {
    pub fn new(title: impl Into<String>, status: Status) -> Self {
        Self {
            title: title.into(),
            status,
            subtitle: None,
            facts: Vec::new(),
            table: None,
            notices: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn fact(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.facts.push(Fact::new(name, value));
        self
    }

    pub fn with_table(mut self, table: TableModel) -> Self {
        self.table = Some(table);
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

    /// Convert into semantic Outcome for unified machine representation.
    pub fn into_outcome(self) -> Outcome {
        let mut outcome = Outcome::new(self.status, self.title);
        outcome.facts = self.facts;
        outcome.warnings = self.notices;
        outcome.actions = self.actions;
        outcome
    }
}

impl RenderHuman for SummaryScreen {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        // Title block (preset casing, marker, subtitle rhythm, block rule)
        let title_comp = Title::new(&self.title);
        title_comp.render_block_human(self.subtitle.as_deref(), ctx, out)?;
        HumanRenderer::write_component_gap(out, ctx)?;

        // Top Status Section
        let section = Section::new("Status").with_status(self.status);
        section.render_human(ctx, out)?;

        // Facts (if any)
        if !self.facts.is_empty() {
            HumanRenderer::write_component_gap(out, ctx)?;
            let kv = KeyValueList::from_facts(self.facts.clone());
            kv.render_human(ctx, out)?;
        }

        // Table (if any)
        if let Some(ref tbl) = self.table {
            HumanRenderer::write_component_gap(out, ctx)?;
            let tv = TableView::new(tbl.clone());
            tv.render_human(ctx, out)?;
        }

        // Notices
        if !self.notices.is_empty() {
            HumanRenderer::write_component_gap(out, ctx)?;
            for n in &self.notices {
                let nv = NoticeView::new(n.clone());
                nv.render_human(ctx, out)?;
            }
        }

        // Action footer
        if !self.actions.is_empty() {
            HumanRenderer::write_component_gap(out, ctx)?;
            let ab = ActionBar::from_actions(self.actions.clone());
            ab.render_human(ctx, out)?;
        }

        Ok(())
    }
}

impl RenderPlain for SummaryScreen {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_block_plain(self.subtitle.as_deref(), ctx, out)?;
        PlainRenderer::write_component_gap(out, ctx)?;

        let section = Section::new("Status").with_status(self.status);
        section.render_plain(ctx, out)?;

        if !self.facts.is_empty() {
            PlainRenderer::write_component_gap(out, ctx)?;
            let kv = KeyValueList::from_facts(self.facts.clone());
            kv.render_plain(ctx, out)?;
        }

        if let Some(ref tbl) = self.table {
            PlainRenderer::write_component_gap(out, ctx)?;
            let tv = TableView::new(tbl.clone());
            tv.render_plain(ctx, out)?;
        }

        if !self.notices.is_empty() {
            PlainRenderer::write_component_gap(out, ctx)?;
            for n in &self.notices {
                let nv = NoticeView::new(n.clone());
                nv.render_plain(ctx, out)?;
            }
        }

        if !self.actions.is_empty() {
            PlainRenderer::write_component_gap(out, ctx)?;
            let ab = ActionBar::from_actions(self.actions.clone());
            ab.render_plain(ctx, out)?;
        }

        Ok(())
    }
}

use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};

#[derive(Serialize)]
struct AgentSummaryRepresentation<'a> {
    schema_version: &'static str,
    status: Status,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: &'a Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    facts: &'a Vec<Fact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table: &'a Option<TableModel>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    warnings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    next_actions: Vec<String>,
}

impl RenderAgent for SummaryScreen {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentSummaryRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            status: self.status,
            title: &self.title,
            subtitle: &self.subtitle,
            facts: &self.facts,
            table: &self.table,
            warnings: self.notices.iter().map(|n| n.message.clone()).collect(),
            next_actions: self.actions.iter().map(|a| a.id.clone()).collect(),
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}
