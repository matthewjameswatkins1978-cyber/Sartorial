use crate::config::SymbolMode;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use std::io::{self, Write};

/// An ordered or bulleted list component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    pub items: Vec<String>,
    pub ordered: bool,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            ordered: false,
        }
    }

    pub fn ordered() -> Self {
        Self {
            items: Vec::new(),
            ordered: true,
        }
    }

    pub fn item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderHuman for List {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        for (idx, item) in self.items.iter().enumerate() {
            if self.ordered {
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::muted_style(),
                    &format!("{:2}. ", idx + 1),
                    ctx.color_enabled,
                )?;
            } else {
                let bullet = match ctx.symbols {
                    SymbolMode::Ascii => "- ",
                    _ => "• ",
                };
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::muted_style(),
                    bullet,
                    ctx.color_enabled,
                )?;
            }
            HumanRenderer::write_styled(
                out,
                HumanRenderer::value_style(),
                item,
                ctx.color_enabled,
            )?;
            writeln!(out)?;
        }
        Ok(())
    }
}

impl RenderPlain for List {
    fn render_plain(&self, _ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        for (idx, item) in self.items.iter().enumerate() {
            if self.ordered {
                writeln!(out, "{:2}. {}", idx + 1, item)?;
            } else {
                writeln!(out, "- {}", item)?;
            }
        }
        Ok(())
    }
}
