use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Block, Document, Presentable, TableModel};
use std::io::{self, Write};

/// Table component following the restrained Biscuit Logic visual standard.
/// Grid rendering converges through the core table block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableView {
    pub model: TableModel,
}

impl TableView {
    pub fn new(model: TableModel) -> Self {
        Self { model }
    }

    /// Truncate text to fit within max_width display cells using ellipsis.
    pub fn truncate_with_ellipsis(text: &str, max_width: usize, is_ascii: bool) -> String {
        sartorial_core::text::truncate_with_ellipsis(
            text,
            max_width,
            if is_ascii { "..." } else { "…" },
        )
    }
}

impl Presentable for TableView {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Table {
            table: self.model.clone(),
        })
    }
}

impl RenderHuman for TableView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for TableView {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}
