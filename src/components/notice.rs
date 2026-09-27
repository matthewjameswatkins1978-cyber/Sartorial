use crate::config::SymbolMode;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::notice::{Notice, NoticeLevel};
use std::io::{self, Write};

/// Renderable notice / callout component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoticeView {
    pub notice: Notice,
}

impl NoticeView {
    pub fn new(notice: Notice) -> Self {
        Self { notice }
    }
}

impl RenderHuman for NoticeView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let glyph = match ctx.symbols {
            SymbolMode::Ascii => self.notice.level.ascii_glyph(),
            _ => self.notice.level.unicode_glyph(),
        };

        if self.notice.level == NoticeLevel::Info {
            // Quiet info is just plain text or muted
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                &self.notice.message,
                ctx.color_enabled,
            )?;
        } else {
            HumanRenderer::write_styled(out, self.notice.level.style(), glyph, ctx.color_enabled)?;
            write!(out, " ")?;
            HumanRenderer::write_styled(
                out,
                HumanRenderer::value_style(),
                &self.notice.message,
                ctx.color_enabled,
            )?;
        }

        if let Some(ref detail) = self.notice.detail {
            write!(out, " ")?;
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                detail,
                ctx.color_enabled,
            )?;
        }
        writeln!(out)
    }
}

impl RenderPlain for NoticeView {
    fn render_plain(&self, _ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        if self.notice.level == NoticeLevel::Info {
            write!(out, "{}", self.notice.message)?;
        } else {
            write!(
                out,
                "{} {}",
                self.notice.level.ascii_glyph(),
                self.notice.message
            )?;
        }
        if let Some(ref detail) = self.notice.detail {
            write!(out, " {detail}")?;
        }
        writeln!(out)
    }
}
