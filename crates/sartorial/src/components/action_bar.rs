use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Action, Block, Document, Presentable};
use std::io::{self, Write};

/// Keyboard action footer teaching hotkeys and available interactions.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ActionBar {
    pub actions: Vec<Action>,
}

impl ActionBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_actions(actions: Vec<Action>) -> Self {
        Self { actions }
    }

    pub fn add(&mut self, action: Action) -> &mut Self {
        self.actions.push(action);
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
}

impl Presentable for ActionBar {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Actions {
            actions: self.actions.clone(),
        })
    }
}

impl RenderHuman for ActionBar {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        TerminalRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for ActionBar {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::render(&self.to_document(), &ctx.style, &ctx.caps, out)
    }
}
