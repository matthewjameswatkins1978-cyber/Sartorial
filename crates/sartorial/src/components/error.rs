use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Block, Document, ErrorModel, Presentable};
use std::io::{self, Write};

/// Error display component conforming to BL error design:
/// WHAT HAPPENED? WHY? WHAT CAN I DO NEXT?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorView {
    pub error: ErrorModel,
}

impl ErrorView {
    pub fn new(error: ErrorModel) -> Self {
        Self { error }
    }
}

impl Presentable for ErrorView {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Error {
            error: self.error.clone(),
        })
    }
}

impl RenderHuman for ErrorView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for ErrorView {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}
