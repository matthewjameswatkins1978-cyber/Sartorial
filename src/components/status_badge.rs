use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::plain::PlainRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::status::Status;
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

impl RenderHuman for StatusBadge {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        HumanRenderer::write_status(out, self.status, ctx)
    }
}

impl RenderPlain for StatusBadge {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::write_status(out, self.status, ctx)
    }
}
