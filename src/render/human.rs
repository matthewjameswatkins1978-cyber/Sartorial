use crate::config::SymbolMode;
use crate::render::context::RenderContext;
use crate::semantic::status::Status;
use anstyle::{AnsiColor, Effects, Style};
use std::io::{self, Write};

/// Styling utilities and helpers for human terminal presentation.
pub struct HumanRenderer;

impl HumanRenderer {
    pub fn title_style(accent: AnsiColor) -> Style {
        Style::new()
            .fg_color(Some(accent.into()))
            .effects(Effects::BOLD)
    }

    pub fn section_style() -> Style {
        Style::new().effects(Effects::BOLD)
    }

    pub fn label_style() -> Style {
        Style::new().fg_color(Some(AnsiColor::BrightBlack.into()))
    }

    pub fn value_style() -> Style {
        Style::new()
    }

    pub fn key_bracket_style() -> Style {
        Style::new().fg_color(Some(AnsiColor::BrightBlack.into()))
    }

    pub fn key_char_style(accent: AnsiColor) -> Style {
        Style::new()
            .fg_color(Some(accent.into()))
            .effects(Effects::BOLD)
    }

    pub fn muted_style() -> Style {
        Style::new().fg_color(Some(AnsiColor::BrightBlack.into()))
    }

    pub fn rule_style() -> Style {
        Style::new().fg_color(Some(AnsiColor::BrightBlack.into()))
    }

    pub fn write_styled(
        out: &mut dyn Write,
        style: Style,
        text: &str,
        color_enabled: bool,
    ) -> io::Result<()> {
        if color_enabled {
            write!(out, "{style}{text}{style:#}")
        } else {
            write!(out, "{text}")
        }
    }

    /// Render a restrained horizontal rule (e.g. `──────────` or `----------`).
    pub fn write_rule(out: &mut dyn Write, ctx: &RenderContext, len: usize) -> io::Result<()> {
        let ch = match ctx.symbols {
            SymbolMode::Ascii => '-',
            _ => '─',
        };
        let line: String = std::iter::repeat_n(ch, len.min(ctx.width)).collect();
        Self::write_styled(out, Self::rule_style(), &line, ctx.color_enabled)?;
        writeln!(out)
    }

    /// Render a status badge (e.g. `✓ Ready` or `[OK] READY`).
    pub fn write_status(
        out: &mut dyn Write,
        status: Status,
        ctx: &RenderContext,
    ) -> io::Result<()> {
        let glyph = match ctx.symbols {
            SymbolMode::Ascii => status.ascii_glyph(),
            _ => status.unicode_glyph(),
        };
        let label = status.display_label();

        if ctx.color_enabled {
            let style = status.style();
            write!(out, "{style}{glyph} {label}{style:#}")
        } else {
            write!(out, "{glyph} {label}")
        }
    }
}
