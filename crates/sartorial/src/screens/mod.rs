pub mod detail_screen;
pub mod list_screen;
pub mod summary;

pub use detail_screen::DetailScreen;
pub use list_screen::ListScreen;
pub use summary::SummaryScreen;

use crate::render::context::RenderContext;
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use sartorial_core::Document;
use std::io::{self, Write};

/// Shared screen header rhythm: title block (subtitle + grammar rule),
/// then one component gap, then the body document.
///
/// Screens render terminals through this path so the Black Tie title-block
/// rule survives. [`Presentable`](sartorial_core::Presentable) documents
/// (used by Markdown) carry `Title` + `Subtitle` blocks instead, which
/// render the same lines without terminal-only rules.
pub(crate) fn render_screen_human(
    title: &str,
    version: Option<&str>,
    subtitle: Option<&str>,
    body: &Document,
    ctx: &RenderContext,
    out: &mut dyn Write,
) -> io::Result<()> {
    TerminalRenderer::render_title_block(title, version, subtitle, &ctx.style, &ctx.caps, out)?;
    if !body.blocks.is_empty() {
        for _ in 0..ctx.style.component_gap(ctx.caps.is_narrow()) {
            writeln!(out)?;
        }
        TerminalRenderer::render(body, &ctx.style, &ctx.caps, out)?;
    }
    Ok(())
}

pub(crate) fn render_screen_plain(
    title: &str,
    version: Option<&str>,
    subtitle: Option<&str>,
    body: &Document,
    ctx: &RenderContext,
    out: &mut dyn Write,
) -> io::Result<()> {
    PlainRenderer::render_title_block(title, version, subtitle, &ctx.style, &ctx.caps, out)?;
    if !body.blocks.is_empty() {
        for _ in 0..ctx.style.component_gap(ctx.caps.is_narrow()) {
            writeln!(out)?;
        }
        PlainRenderer::render(body, &ctx.style, &ctx.caps, out)?;
    }
    Ok(())
}
