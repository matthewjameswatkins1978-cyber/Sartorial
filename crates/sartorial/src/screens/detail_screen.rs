use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use crate::screens::{render_screen_human, render_screen_plain};
use sartorial_core::{Action, Block, Document, Evidence, Fact, Notice, Presentable, Status};
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
        if let Some(ev) = &self.evidence {
            doc.blocks.push(Block::EvidenceDetail {
                section_title: Some("Evidence".to_string()),
                item: ev.clone(),
            });
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

impl Presentable for DetailScreen {
    fn to_document(&self) -> Document {
        let mut doc = Document::new().push(Block::Title {
            text: self.title.clone(),
            version: None,
        });
        doc.blocks.extend(self.body_document().blocks);
        doc
    }
}

impl RenderHuman for DetailScreen {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        render_screen_human(&self.title, None, None, &self.body_document(), ctx, out)
    }
}

impl RenderPlain for DetailScreen {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        render_screen_plain(&self.title, None, None, &self.body_document(), ctx, out)
    }
}

#[cfg(feature = "wire")]
use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};

#[cfg(feature = "wire")]
impl RenderAgent for DetailScreen {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = crate::render::agent_impls::AgentDetailRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            status: self.status,
            title: &self.title,
            facts: &self.facts,
            evidence: &self.evidence,
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
