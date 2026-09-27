pub mod agent;
pub mod context;
pub mod human;
pub mod plain;
pub mod target;

pub use agent::{AgentEnvelope, AgentRenderer, SARTORIAL_SCHEMA_VERSION};
pub use context::{RenderContext, WidthCategory};
pub use human::HumanRenderer;
pub use plain::PlainRenderer;
pub use target::RenderTarget;

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

/// Trait for types participating in the canonical Sartorial machine/agent protocol.
///
/// Unlike generic serialization, implementations of this trait produce a stable,
/// schema-versioned JSON structure strictly bounded for context economy.
pub trait RenderAgent {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error>;
}
