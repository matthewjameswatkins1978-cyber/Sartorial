use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::plain::PlainRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::status::Status;
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// Section header with clear typography and optional right-aligned status or badge.
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

impl RenderHuman for Section {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_len = self.title.width();
        let target_width = ctx.width.min(60);

        HumanRenderer::write_styled(
            out,
            HumanRenderer::section_style(),
            &self.title,
            ctx.color_enabled,
        )?;

        if let Some(status) = self.status {
            let glyph_label = format!("{} {}", status.unicode_glyph(), status.display_label());
            let badge_len = glyph_label.width();
            let spaces = if target_width > title_len + badge_len {
                target_width - title_len - badge_len
            } else {
                2
            };
            write!(out, "{}", " ".repeat(spaces))?;
            HumanRenderer::write_status(out, status, ctx)?;
        } else if let Some(ref badge) = self.badge {
            let badge_len = badge.width();
            let spaces = if target_width > title_len + badge_len {
                target_width - title_len - badge_len
            } else {
                2
            };
            write!(out, "{}", " ".repeat(spaces))?;
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                badge,
                ctx.color_enabled,
            )?;
        }
        writeln!(out)
    }
}

impl RenderPlain for Section {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_len = self.title.width();
        let target_width = ctx.width.min(60);

        write!(out, "{}", self.title)?;

        if let Some(status) = self.status {
            let badge_text = format!("{} {}", status.ascii_glyph(), status.display_label());
            let badge_len = badge_text.width();
            let spaces = if target_width > title_len + badge_len {
                target_width - title_len - badge_len
            } else {
                2
            };
            write!(out, "{}", " ".repeat(spaces))?;
            PlainRenderer::write_status(out, status, ctx)?;
        } else if let Some(ref badge) = self.badge {
            let badge_len = badge.width();
            let spaces = if target_width > title_len + badge_len {
                target_width - title_len - badge_len
            } else {
                2
            };
            write!(out, "{}{badge}", " ".repeat(spaces))?;
        }
        writeln!(out)
    }
}
