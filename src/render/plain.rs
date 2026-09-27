use crate::render::context::RenderContext;
use crate::semantic::status::Status;
use std::io::{self, Write};

/// Plain text rendering utilities (pipe-safe, no escape sequences).
pub struct PlainRenderer;

impl PlainRenderer {
    pub fn write_status(
        out: &mut dyn Write,
        status: Status,
        _ctx: &RenderContext,
    ) -> io::Result<()> {
        let glyph = status.ascii_glyph();
        let label = status.display_label();
        write!(out, "{glyph} {label}")
    }

    pub fn write_rule(out: &mut dyn Write, ctx: &RenderContext, len: usize) -> io::Result<()> {
        let line = "-".repeat(len.min(ctx.width));
        writeln!(out, "{line}")
    }

    /// Blank lines between major components per the preset grammar.
    pub fn write_component_gap(out: &mut dyn Write, ctx: &RenderContext) -> io::Result<()> {
        for _ in 0..ctx.style.component_gap(ctx.is_narrow()) {
            writeln!(out)?;
        }
        Ok(())
    }
}
