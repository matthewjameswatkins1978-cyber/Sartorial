use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use crate::screens::{render_screen_human, render_screen_plain};
use sartorial_core::{Action, Block, Document, Notice, Presentable, TableModel};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// An opinionated List Screen displaying inventory, records, or multi-item collections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListScreen {
    pub title: String,
    pub count_badge: Option<String>,
    pub table: TableModel,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notices: Vec<Notice>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
}

impl ListScreen {
    pub fn new(title: impl Into<String>, table: TableModel) -> Self {
        Self {
            title: title.into(),
            count_badge: None,
            table,
            notices: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.count_badge = Some(badge.into());
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
        let mut doc = Document::new();
        if let Some(badge) = &self.count_badge {
            doc.blocks.push(Block::BadgeSection {
                label: "Collection".to_string(),
                badge: badge.clone(),
            });
        }
        doc.blocks.push(Block::Table {
            table: self.table.clone(),
        });
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

impl Presentable for ListScreen {
    fn to_document(&self) -> Document {
        let mut doc = Document::new().push(Block::Title {
            text: self.title.clone(),
            version: None,
        });
        doc.blocks.extend(self.body_document().blocks);
        doc
    }
}

impl RenderHuman for ListScreen {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        render_screen_human(&self.title, None, None, &self.body_document(), ctx, out)
    }
}

impl RenderPlain for ListScreen {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        render_screen_plain(&self.title, None, None, &self.body_document(), ctx, out)
    }
}

#[cfg(feature = "wire")]
use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};

#[cfg(feature = "wire")]
impl RenderAgent for ListScreen {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = crate::render::agent_impls::AgentListRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            status: sartorial_core::Status::Ready,
            title: &self.title,
            count_badge: &self.count_badge,
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
