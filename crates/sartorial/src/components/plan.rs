//! [`Plan`] rendering converges through its core [`Document`] form.
//!
//! The title line plus plan-body composition (description, `+`/`-`/`~`
//! changes, warnings, consequences, reversibility, actions) is owned by
//! core; these impls only route the batteries-included context into it.

use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::{Document, Plan, Presentable};
use std::io::{self, Write};

impl RenderHuman for Plan {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let doc: Document = self.to_document();
        TerminalRenderer::render(&doc, &ctx.style, &ctx.caps, out)
    }
}

impl RenderPlain for Plan {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let doc: Document = self.to_document();
        PlainRenderer::render(&doc, &ctx.style, &ctx.caps, out)
    }
}
