use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Block, Document, Presentable};
use std::io::{self, Write};

/// Program title component: visually strongest textual identifier.
///
/// Casing and structural markers come from the resolved preset grammar;
/// the application-supplied name is never modified semantically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title {
    pub name: String,
    pub version: Option<String>,
}

impl Title {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Title plus optional subtitle block with preset rhythm, converging
    /// through the core title-block renderers.
    pub fn render_block_human(
        &self,
        subtitle: Option<&str>,
        ctx: &RenderContext,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        TerminalRenderer::render_title_block(
            &self.name,
            self.version.as_deref(),
            subtitle,
            &ctx.style,
            &ctx.caps,
            out,
        )
    }

    /// Pipe-safe title block: same structure, no ANSI, never animated.
    pub fn render_block_plain(
        &self,
        subtitle: Option<&str>,
        ctx: &RenderContext,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        PlainRenderer::render_title_block(
            &self.name,
            self.version.as_deref(),
            subtitle,
            &ctx.style,
            &ctx.caps,
            out,
        )
    }
}

impl Presentable for Title {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Title {
            text: self.name.clone(),
            version: self.version.clone(),
        })
    }
}

impl RenderHuman for Title {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for Title {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}
