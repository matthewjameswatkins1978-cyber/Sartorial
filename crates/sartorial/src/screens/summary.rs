use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use crate::screens::{render_screen_human, render_screen_plain};
use sartorial_core::{Action, Block, Document, Fact, Notice, Presentable, Status, TableModel};
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
        let mut outcome = sartorial_core::Outcome::new(self.status, self.title);
        outcome.facts = self.facts;
        outcome.warnings = self.notices;
        outcome.actions = self.actions;
        outcome
    }

    fn body_document(&self) -> Document {
        let mut doc = Document::new().push(Block::StatusSection {
            label: "Status".to_string(),
            status: self.status,
        });
        if !self.facts.is_empty() {
            doc.blocks.push(Block::Facts {
                facts: self.facts.clone(),
            });
        }
        if let Some(tbl) = &self.table {
            doc.blocks.push(Block::Table { table: tbl.clone() });
        }
        if !self.notices.is_empty() {
            doc.blocks.push(Block::Notices {
                notices: self.notices.clone(),
            });
        }
        if !self.actions.is_empty() {
            doc.blocks.push(Block::Actions {
                actions: self.actions.clone(),
            });
        }
        doc
    }
}

use sartorial_core::Outcome;

impl Presentable for SummaryScreen {
    fn to_document(&self) -> Document {
        let mut doc = Document::new().push(Block::Title {
            text: self.title.clone(),
            version: None,
        });
        if let Some(sub) = &self.subtitle {
            doc.blocks.push(Block::Subtitle { text: sub.clone() });
        }
        doc.blocks.extend(self.body_document().blocks);
        doc
    }
}

impl RenderHuman for SummaryScreen {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        render_screen_human(
            &self.title,
            None,
            self.subtitle.as_deref(),
            &self.body_document(),
            ctx,
            out,
        )
    }
}

impl RenderPlain for SummaryScreen {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        render_screen_plain(
            &self.title,
            None,
            self.subtitle.as_deref(),
            &self.body_document(),
            ctx,
            out,
        )
    }
}

#[cfg(feature = "wire")]
use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};

#[cfg(feature = "wire")]
impl RenderAgent for SummaryScreen {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = crate::render::agent_impls::AgentSummaryRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            status: self.status,
            title: &self.title,
            subtitle: &self.subtitle,
            facts: &self.facts,
            table: &self.table,
            warnings: self.notices.iter().map(|n| n.message.clone()).collect(),
            next_actions: crate::render::agent_impls::next_action_ids(&self.actions),
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}
