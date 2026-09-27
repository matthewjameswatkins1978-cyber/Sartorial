use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::plain::PlainRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::status::Status;
use crate::style::{SectionRule, StatusLayout};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// Section header with clear typography and optional status or badge.
///
/// Placement comes from the resolved grammar: inline headings keep a bounded
/// label/badge gap regardless of terminal width, while stacked headings earn
/// a short rule and give the status its own line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub title: String,
    pub status: Option<Status>,
    pub badge: Option<String>,
}

impl Section {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            status: None,
            badge: None,
        }
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }
}

/// Heading line plus optional short rule (heading-width, never terminal-width).
fn write_heading_rule(
    out: &mut dyn Write,
    ctx: &RenderContext,
    heading_width: usize,
    plain: bool,
) -> io::Result<()> {
    if ctx.style.section_rule == SectionRule::Short {
        if plain {
            PlainRenderer::write_rule(out, ctx, heading_width)?;
        } else {
            HumanRenderer::write_rule(out, ctx, heading_width)?;
        }
    }
    Ok(())
}

impl RenderHuman for Section {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_text = ctx.style.format_section_header(&self.title);
        let title_len = title_text.width();

        HumanRenderer::write_styled(
            out,
            ctx.style.section_style(),
            &title_text,
            ctx.color_enabled,
        )?;

        match (ctx.style.status_layout, self.status, self.badge.as_ref()) {
            (StatusLayout::Inline, Some(status), _) => {
                write!(out, "{}", " ".repeat(ctx.style.status_gap))?;
                HumanRenderer::write_status(out, status, ctx)?;
                writeln!(out)?;
            }
            (StatusLayout::Inline, None, Some(badge)) => {
                write!(out, "{}", " ".repeat(ctx.style.status_gap))?;
                HumanRenderer::write_styled(
                    out,
                    ctx.style.muted_style(),
                    badge,
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
            }
            (StatusLayout::Stacked, Some(status), _) => {
                writeln!(out)?;
                write_heading_rule(out, ctx, title_len, false)?;
                HumanRenderer::write_status(out, status, ctx)?;
                writeln!(out)?;
            }
            (StatusLayout::Stacked, None, Some(badge)) => {
                // Badge metadata stays inline; the heading keeps its air.
                write!(out, "  ")?;
                HumanRenderer::write_styled(
                    out,
                    ctx.style.muted_style(),
                    badge,
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
            }
            (_, None, None) => {
                writeln!(out)?;
                write_heading_rule(out, ctx, title_len, false)?;
            }
        }
        Ok(())
    }
}

impl RenderPlain for Section {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_text = ctx.style.format_section_header(&self.title);
        let title_len = title_text.width();

        write!(out, "{title_text}")?;

        match (ctx.style.status_layout, self.status, self.badge.as_ref()) {
            (StatusLayout::Inline, Some(status), _) => {
                write!(out, "{}", " ".repeat(ctx.style.status_gap))?;
                PlainRenderer::write_status(out, status, ctx)?;
                writeln!(out)?;
            }
            (StatusLayout::Inline, None, Some(badge)) => {
                write!(out, "{}", " ".repeat(ctx.style.status_gap))?;
                writeln!(out, "{badge}")?;
            }
            (StatusLayout::Stacked, Some(status), _) => {
                writeln!(out)?;
                write_heading_rule(out, ctx, title_len, true)?;
                PlainRenderer::write_status(out, status, ctx)?;
                writeln!(out)?;
            }
            (StatusLayout::Stacked, None, Some(badge)) => {
                writeln!(out, "  {badge}")?;
            }
            (_, None, None) => {
                writeln!(out)?;
                write_heading_rule(out, ctx, title_len, true)?;
            }
        }
        Ok(())
    }
}
