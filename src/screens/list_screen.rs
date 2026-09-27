use crate::components::action_bar::ActionBar;
use crate::components::notice::NoticeView;
use crate::components::section::Section;
use crate::components::table::TableView;
use crate::components::title::Title;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::plain::PlainRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::action::Action;
use crate::semantic::notice::Notice;
use crate::semantic::table::TableModel;
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
}

impl RenderHuman for ListScreen {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_block_human(None, ctx, out)?;
        HumanRenderer::write_component_gap(out, ctx)?;

        if let Some(ref badge) = self.count_badge {
            let sec = Section::new("Collection").with_badge(badge);
            sec.render_human(ctx, out)?;
        }

        let tv = TableView::new(self.table.clone());
        tv.render_human(ctx, out)?;

        if !self.notices.is_empty() {
            HumanRenderer::write_component_gap(out, ctx)?;
            for n in &self.notices {
                let nv = NoticeView::new(n.clone());
                nv.render_human(ctx, out)?;
            }
        }

        if !self.actions.is_empty() {
            HumanRenderer::write_component_gap(out, ctx)?;
            let ab = ActionBar::from_actions(self.actions.clone());
            ab.render_human(ctx, out)?;
        }

        Ok(())
    }
}

impl RenderPlain for ListScreen {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_block_plain(None, ctx, out)?;
        PlainRenderer::write_component_gap(out, ctx)?;

        if let Some(ref badge) = self.count_badge {
            let sec = Section::new("Collection").with_badge(badge);
            sec.render_plain(ctx, out)?;
        }

        let tv = TableView::new(self.table.clone());
        tv.render_plain(ctx, out)?;

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
use crate::semantic::status::Status;

#[derive(Serialize)]
struct AgentListRepresentation<'a> {
    schema_version: &'static str,
    status: Status,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    count_badge: &'a Option<String>,
    table: &'a TableModel,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    warnings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    next_actions: Vec<String>,
}

impl RenderAgent for ListScreen {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentListRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            status: Status::Ready,
            title: &self.title,
            count_badge: &self.count_badge,
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
