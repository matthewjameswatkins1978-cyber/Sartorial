use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Block, Document, Presentable, Status};
use std::io::{self, Write};

/// Section header with clear typography and optional status or badge.
/// Placement comes from the resolved grammar via core section blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub title: String,
    pub status: Option<Status>,
    pub badge: Option<String>,
}

impl Section {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            status: None,
            badge: None,
        }
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    fn to_block(&self) -> Block {
        if let Some(status) = self.status {
            Block::StatusSection {
                label: self.title.clone(),
                status,
            }
        } else {
            Block::BadgeSection {
                label: self.title.clone(),
                badge: self.badge.clone().unwrap_or_default(),
            }
        }
    }

    fn to_document(&self) -> Document {
        Document::new().push(self.to_block())
    }
}

impl Presentable for Section {
    fn to_document(&self) -> Document {
        Document::new().push(self.to_block())
    }
}

impl RenderHuman for Section {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for Section {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}
