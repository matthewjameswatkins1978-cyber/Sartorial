use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::plain::PlainRenderer;
use crate::render::{RenderHuman, RenderPlain};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// Program title component: Visually strongest textual identifier.
///
/// Casing and structural markers come from the resolved preset grammar
/// (`ResolvedStyle::format_title`); the application-supplied name is never
/// modified semantically, only presented.
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

    /// Title plus optional subtitle block with preset rhythm: the subtitle is
    /// muted, indented under a structural marker when one is present, and a
    /// restrained rule follows the block where the grammar calls for it.
    pub fn render_block_human(
        &self,
        subtitle: Option<&str>,
        ctx: &RenderContext,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        self.render_human(ctx, out)?;
        if let Some(sub) = subtitle {
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                &format!("{}{sub}", subtitle_indent(ctx)),
                ctx.color_enabled,
            )?;
            writeln!(out)?;
        }
        if ctx.style.title_block_rule {
            HumanRenderer::write_rule(out, ctx, ctx.style.title_rule_len(ctx.width))?;
        }
        Ok(())
    }

    /// Pipe-safe title block: same structure, no ANSI, never animated.
    pub fn render_block_plain(
        &self,
        subtitle: Option<&str>,
        ctx: &RenderContext,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        self.render_plain(ctx, out)?;
        if let Some(sub) = subtitle {
            writeln!(out, "{}{sub}", subtitle_indent(ctx))?;
        }
        if ctx.style.title_block_rule {
            PlainRenderer::write_rule(out, ctx, ctx.style.title_rule_len(ctx.width))?;
        }
        Ok(())
    }
}

/// Subtitle indent aligns under a structural marker (`» DEMO` / `  /repo`).
fn subtitle_indent(ctx: &RenderContext) -> String {
    " ".repeat(ctx.style.title_marker.width())
}

impl RenderHuman for Title {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_text = ctx.style.format_title(&self.name);
        HumanRenderer::write_styled(out, ctx.style.title_style(), &title_text, ctx.color_enabled)?;

        if let Some(ref ver) = self.version {
            write!(out, " ")?;
            HumanRenderer::write_styled(out, ctx.style.muted_style(), ver, ctx.color_enabled)?;
        }
        writeln!(out)
    }
}

impl RenderPlain for Title {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_text = ctx.style.format_title(&self.name);
        if let Some(ref ver) = self.version {
            writeln!(out, "{title_text} {ver}")
        } else {
            writeln!(out, "{title_text}")
        }
    }
}
