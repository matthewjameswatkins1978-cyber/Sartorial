pub mod agent;
pub mod context;
pub mod human;
pub mod plain;
pub mod target;

pub use agent::AgentRenderer;
pub use context::{RenderContext, WidthCategory};
pub use human::HumanRenderer;
pub use plain::PlainRenderer;
pub use target::RenderTarget;

use std::io::{self, Write};

/// Trait for human-facing terminal rendering with optional ANSI styling.
pub trait RenderHuman {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()>;

    fn to_human_string(&self, ctx: &RenderContext) -> String {
        let mut buf = Vec::new();
        let _ = self.render_human(ctx, &mut buf);
        String::from_utf8_lossy(&buf).into_owned()
    }
}

/// Trait for pipe-safe, plain text rendering (no ANSI escape codes).
pub trait RenderPlain {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()>;

    fn to_plain_string(&self, ctx: &RenderContext) -> String {
        let mut buf = Vec::new();
        let _ = self.render_plain(ctx, &mut buf);
        String::from_utf8_lossy(&buf).into_owned()
    }
}

/// Trait for AI agent / machine JSON rendering.
pub trait RenderAgent: serde::Serialize {
    fn render_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        if pretty {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
    }
}

// Blanket implementation of RenderAgent for any serde::Serialize type
impl<T: serde::Serialize> RenderAgent for T {}
