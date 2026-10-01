//! # Sartorial
//!
//! Batteries-included terminal presentation toolkit for command-line
//! applications, embodying the **Biscuit Logic CLI Presentation Standard**.
//!
//! Applications own truth. Sartorial owns presentation.
//!
//! - Use [`sartorial_core`] when you want deterministic presentation:
//!   semantic types, one [`Document`](sartorial_core::Document) model,
//!   [`Preset`] behaviour plus [`Theme`] identity, explicit
//!   [`Capabilities`](sartorial_core::Capabilities), and terminal / plain /
//!   Markdown renderers with no live-terminal machinery.
//! - Use `sartorial` (this crate) when you want terminal behaviour and
//!   integrations: capability detection, channel hygiene, live progress,
//!   interactive prompts, Clap integration, completions, the optional
//!   `wire` protocol surface, and diagnostic adapters.
//!
//! Sartorial dresses your program's semantics; it never invents them.

pub mod accessibility;
pub mod channel;
pub mod components;
pub mod config;
#[cfg(feature = "diagnostics")]
pub mod diagnostics;
pub mod exit;
pub mod help;
pub mod hyperlink;
pub mod interaction;
pub mod motion;
pub mod pager;
#[cfg(feature = "wire")]
pub mod protocol;
pub mod provenance;
pub mod render;
pub mod screens;
/// Canonical semantic model (re-exported from core so both crates share
/// one truth; `sartorial::semantic::Fact` and `sartorial_core::Fact` are
/// the same type).
pub use sartorial_core::semantic;
pub mod style;
/// Text measurement and truncation helpers (canonical in core).
pub use sartorial_core::text;
pub mod typo;
pub mod verbosity;

#[cfg(feature = "clap")]
pub mod clap_ext;
#[cfg(feature = "completions")]
pub mod completions;

// Core engine re-exports: the small engine underneath the suit.
pub use sartorial_core::{
    Action, Block, Capabilities, ChangeKind, ChoiceItem, ColorPolicy, ColumnAlignment, Density,
    Document, ErrorModel, Evidence, Fact, Interactive, KeyTrigger, Notice, NoticeLevel, Outcome,
    Plan, PlanChange, Presentable, Preset, ProgressError, ProgressMode, ProgressState, Receipt,
    ResolvedStyle, Status, SymbolMode, TableModel, TableRow, Theme, ThemeBuilder,
};

// Re-export core semantic types at top-level for fast, ergonomic usage
pub use accessibility::AccessibilityMode;
pub use channel::SartorialOutput;
pub use components::{
    ActionBar, Choice, ChoiceOutcome, Confirm, ConfirmOutcome, DetailView, ErrorView, KeyValueList,
    MultiProgressView, NoticeView, ProgressBar, Section, StatusBadge, TableView, Title,
};
pub use config::{BorderStyle, ColorChoice, Config, InteractiveMode};
pub use exit::ExitCode;
pub use help::{HelpEntry, HelpView};
pub use hyperlink::Hyperlink;
pub use motion::MotionMode;
pub use pager::PagerMode;
#[cfg(feature = "wire")]
pub use protocol::{ProgressEvent, ProtocolEnvelope};
pub use provenance::{ProvenanceFact, ProvenanceList, ProvenanceSource};
pub use render::{
    AgentRenderer, MarkdownRenderer, PlainRenderer, RenderContext, RenderHuman, RenderMarkdown,
    RenderPlain, RenderTarget, TerminalRenderer, WidthCategory,
};
#[cfg(feature = "wire")]
pub use render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};
pub use screens::{DetailScreen, ListScreen, SummaryScreen};
pub use style::{ProgressTreatment, SectionRule, StatusLayout, TitleCase};
pub use typo::TypoSuggestion;
pub use verbosity::Verbosity;

/// Quick helper to render any displayable Sartorial item to stdout using default configuration.
pub fn print_human<T>(item: &T) -> std::io::Result<()>
where
    T: render::RenderHuman,
{
    let ctx = RenderContext::detect();
    let mut out = anstream::stdout();
    item.render_human(&ctx, &mut out)
}

/// Quick helper to render an item as plain text.
pub fn to_plain<T>(item: &T) -> std::io::Result<String>
where
    T: render::RenderPlain,
{
    let ctx = RenderContext::plain();
    let mut buf = Vec::new();
    item.render_plain(&ctx, &mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

/// Quick helper to render an item as Markdown.
pub fn to_markdown<T>(item: &T) -> std::io::Result<String>
where
    T: render::RenderMarkdown,
{
    let ctx = RenderContext::markdown(sartorial_core::Preset::House);
    item.render_markdown(&ctx)
}

/// Quick helper to render an item as agent JSON (requires the `wire` feature).
#[cfg(feature = "wire")]
pub fn to_agent_json<T>(item: &T) -> Result<String, serde_json::Error>
where
    T: render::RenderAgent,
{
    item.to_agent_json(true)
}
