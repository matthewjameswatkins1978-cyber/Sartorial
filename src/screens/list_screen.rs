use crate::components::action_bar::ActionBar;
use crate::components::notice::NoticeView;
use crate::components::section::Section;
use crate::components::table::TableView;
use crate::components::title::Title;
use crate::render::context::RenderContext;
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
        title_comp.render_human(ctx, out)?;
        writeln!(out)?;

        if let Some(ref badge) = self.count_badge {
            let sec = Section::new("Collection").with_badge(badge);
            sec.render_human(ctx, out)?;
        }

        let tv = TableView::new(self.table.clone());
        tv.render_human(ctx, out)?;

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

impl RenderPlain for ListScreen {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_plain(ctx, out)?;
        writeln!(out)?;

        if let Some(ref badge) = self.count_badge {
            let sec = Section::new("Collection").with_badge(badge);
            sec.render_plain(ctx, out)?;
        }

        let tv = TableView::new(self.table.clone());
        tv.render_plain(ctx, out)?;

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
