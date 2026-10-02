use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Block, Document, Notice, Presentable};
use std::io::{self, Write};

/// Renderable notice / callout component. Converges through core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoticeView {
    pub notice: Notice,
}

impl NoticeView {
    pub fn new(notice: Notice) -> Self {
        Self { notice }
    }
}

impl Presentable for NoticeView {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Notices {
            notices: vec![self.notice.clone()],
        })
    }
}

impl RenderHuman for NoticeView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for NoticeView {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}
