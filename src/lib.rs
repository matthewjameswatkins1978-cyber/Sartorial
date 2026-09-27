//! # Sartorial
//!
//! An opinionated Rust presentation toolkit for command-line applications,
//! embodying the **Biscuit Logic CLI Presentation Standard**.
//!
//! Sartorial provides:
//! - A strongly typed semantic model (`Outcome`, `Status`, `Fact`, `Evidence`, `Notice`, `Action`, `Table`)
//! - Triple render targets from one semantic truth: **Human** (styled ANSI), **Plain** (pipe-safe), and **Agent** (JSON)
//! - Standard screen shapes (`SummaryScreen`, `ListScreen`, `DetailScreen`)
//! - Standard components (`Title`, `Section`, `StatusBadge`, `KeyValue`, `Table`, `Notice`, `ErrorView`, `ActionBar`, `Choice`, `Confirm`, `Progress`)
//! - Biscuit Logic keyboard grammar and safe terminal interaction
//! - Graceful terminal width degradation (narrow, normal, wide)
//! - Restrained, quiet visual character ("well-made control instrument")

pub mod components;
pub mod config;
pub mod interaction;
pub mod render;
pub mod screens;
pub mod semantic;

#[cfg(feature = "clap")]
pub mod clap_ext;

// Re-export core semantic types at top-level for fast, ergonomic usage
pub use components::{
    ActionBar, Choice, Confirm, DetailView, ErrorView, KeyValueList, NoticeView, ProgressBar,
    Section, StatusBadge, TableView, Title,
};
pub use config::{BorderStyle, ColorChoice, Config, Density, SymbolMode};
pub use render::{
    AgentRenderer, HumanRenderer, PlainRenderer, RenderAgent, RenderContext, RenderHuman,
    RenderPlain, RenderTarget,
};
pub use screens::{DetailScreen, ListScreen, SummaryScreen};
pub use semantic::{
    Action, ChoiceItem, ErrorModel, Evidence, Fact, Notice, NoticeLevel, Outcome, ProgressState,
    Status, TableModel, TableRow,
};

/// Quick helper to render any displayable Sartorial item to stdout using default configuration.
pub fn print_human<T>(item: &T)
where
    T: render::RenderHuman,
{
    let ctx = RenderContext::detect();
    let mut out = anstream::stdout();
    let _ = item.render_human(&ctx, &mut out);
}

/// Quick helper to render an item as plain text.
pub fn to_plain<T>(item: &T) -> String
where
    T: render::RenderPlain,
{
    let ctx = RenderContext::plain();
    let mut buf = Vec::new();
    let _ = item.render_plain(&ctx, &mut buf);
    String::from_utf8_lossy(&buf).into_owned()
}

/// Quick helper to render an item as agent JSON.
pub fn to_agent_json<T>(item: &T) -> Result<String, serde_json::Error>
where
    T: render::RenderAgent,
{
    item.render_agent_json(true)
}
