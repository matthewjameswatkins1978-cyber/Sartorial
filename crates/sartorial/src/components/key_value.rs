use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Block, Document, Fact, Presentable};
use std::io::{self, Write};

/// A structured list of key-value facts aligned according to BL typography.
/// Converges through the core facts block.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyValueList {
    pub facts: Vec<Fact>,
}

impl KeyValueList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_facts(facts: Vec<Fact>) -> Self {
        Self { facts }
    }

    pub fn add(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.facts.push(Fact::new(name, value));
        self
    }

    pub fn with_fact(mut self, fact: Fact) -> Self {
        self.facts.push(fact);
        self
    }
}

impl Presentable for KeyValueList {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Facts {
            facts: self.facts.clone(),
        })
    }
}

impl RenderHuman for KeyValueList {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for KeyValueList {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}
