pub mod agent;
#[cfg(feature = "wire")]
pub(crate) mod agent_impls;
pub mod context;
pub mod document;
pub mod target;

#[cfg(feature = "wire")]
pub use agent::{AgentRenderer, SARTORIAL_SCHEMA_VERSION};
pub use context::{RenderContext, WidthCategory};
pub use document::RenderMarkdown;
pub use target::RenderTarget;

pub use sartorial_core::render::{MarkdownRenderer, PlainRenderer, TerminalRenderer};

use std::io::{self, Write};

/// Trait for human-facing terminal rendering with optional ANSI styling.
pub trait RenderHuman {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()>;

    fn to_human_string(&self, ctx: &RenderContext) -> io::Result<String> {
        let mut buf = Vec::new();
        self.render_human(ctx, &mut buf)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }
}

/// Trait for pipe-safe, plain text rendering (no ANSI escape codes).
pub trait RenderPlain {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()>;

    fn to_plain_string(&self, ctx: &RenderContext) -> io::Result<String> {
        let mut buf = Vec::new();
        self.render_plain(ctx, &mut buf)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }
}

/// Trait for types participating in the optional presentation wire surface.
///
/// Unlike generic serialization, implementations produce a stable,
/// schema-versioned JSON structure strictly bounded for context economy.
/// Available with the `wire` feature; application business schemas stay
/// application-owned.
#[cfg(feature = "wire")]
pub trait RenderAgent {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error>;
}
