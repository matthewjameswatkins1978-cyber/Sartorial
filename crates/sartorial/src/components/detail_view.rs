use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Action, Block, Document, Evidence, Presentable};
use std::io::{self, Write};

/// Progressive disclosure detail view for deep evidence, logs, or diagnostics.
/// Converges through the core evidence-detail block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetailView {
    pub title: String,
    pub evidence: Evidence,
    pub actions: Vec<Action>,
}

impl DetailView {
    pub fn new(title: impl Into<String>, evidence: Evidence) -> Self {
        Self {
            title: title.into(),
            evidence,
            actions: Vec::new(),
        }
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
}

impl Presentable for DetailView {
    fn to_document(&self) -> Document {
        let mut doc = Document::new().push(Block::EvidenceDetail {
            section_title: Some(self.title.clone()),
            item: self.evidence.clone(),
        });
        if !self.actions.is_empty() {
            doc.blocks.push(Block::Actions {
                actions: self.actions.clone(),
            });
        }
        doc
    }
}

impl RenderHuman for DetailView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for DetailView {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}
