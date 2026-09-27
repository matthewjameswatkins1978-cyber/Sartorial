use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use std::io::{self, Write};

/// Program title component: Visually strongest textual identifier, compact, uppercase.
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
}

impl RenderHuman for Title {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_upper = self.name.to_uppercase();
        HumanRenderer::write_styled(
            out,
            HumanRenderer::title_style(ctx.config.accent),
            &title_upper,
            ctx.color_enabled,
        )?;

        if let Some(ref ver) = self.version {
            write!(out, " ")?;
            HumanRenderer::write_styled(out, HumanRenderer::muted_style(), ver, ctx.color_enabled)?;
        }
        writeln!(out)
    }
}

impl RenderPlain for Title {
    fn render_plain(&self, _ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_upper = self.name.to_uppercase();
        if let Some(ref ver) = self.version {
            writeln!(out, "{title_upper} {ver}")
        } else {
            writeln!(out, "{title_upper}")
        }
    }
}
