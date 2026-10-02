use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Block, Document, Presentable};
use std::io::{self, Write};

/// An ordered or bulleted list component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    pub items: Vec<String>,
    pub ordered: bool,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            ordered: false,
        }
    }

    pub fn ordered() -> Self {
        Self {
            items: Vec::new(),
            ordered: true,
        }
    }

    pub fn unordered() -> Self {
        Self {
            items: Vec::new(),
            ordered: false,
        }
    }

    pub fn item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl Presentable for List {
    fn to_document(&self) -> Document {
        Document::new().push(Block::List {
            ordered: self.ordered,
            items: self.items.clone(),
        })
    }
}

impl RenderHuman for List {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for List {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}
