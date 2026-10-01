use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Block, Document, Presentable, Status};
use std::io::{self, Write};

/// Status badge component representing an operational state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusBadge {
    pub status: Status,
}

impl StatusBadge {
    pub fn new(status: Status) -> Self {
        Self { status }
    }
}

impl Presentable for StatusBadge {
    fn to_document(&self) -> Document {
        Document::new().push(Block::StatusSection {
            label: "Status".to_string(),
            status: self.status,
        })
    }
}

impl RenderHuman for StatusBadge {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::write_status(out, self.status, &ctx.style, &ctx.caps)
    }
}

impl RenderPlain for StatusBadge {
    fn render_plain(&self, _ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::write_status(out, self.status)
    }
}
