//! # Sartorial
//!
//! An opinionated Rust presentation toolkit for command-line applications,
//! embodying the **Biscuit Logic CLI Presentation Standard**.
//!
//! Sartorial provides:
//! - A strongly typed semantic model (`Outcome`, `Status`, `Fact`, `Evidence`, `Notice`, `Action`, `Table`, `Plan`, `Receipt`)
//! - Four first-party visual presets: **House** (default), **Black Tie**, **Workwear**, and **Studio**
//! - BL Motion Standard with honest progress modes (Activity, Count, Percent, Countdown, Rate)
//! - Triple render targets from one semantic truth: **Human** (styled ANSI), **Plain** (pipe-safe), and **Agent** (JSON)
//! - Operation UX: Consequential dry-run plans (`Plan`) and state-change receipts (`Receipt`)
//! - Standard screen shapes (`SummaryScreen`, `ListScreen`, `DetailScreen`)
//! - Standard components (`Title`, `Section`, `StatusBadge`, `KeyValue`, `Table`, `Notice`, `ErrorView`, `ActionBar`, `Choice`, `Confirm`, `Progress`, `MultiProgressView`, `HelpView`)
//! - Biscuit Logic keyboard grammar and safe terminal interaction
//! - Channel hygiene (Results -> STDOUT, Progress/Warnings -> STDERR, JSON -> STDOUT only)
//! - Documented semantic exit contract (`ExitCode`)

pub mod accessibility;
pub mod channel;
pub mod components;
pub mod config;
pub mod exit;
pub mod help;
pub mod hyperlink;
pub mod interaction;
pub mod motion;
pub mod pager;
pub mod protocol;
pub mod provenance;
pub mod render;
pub mod screens;
pub mod semantic;
pub mod style;
pub mod text;
pub mod typo;
pub mod verbosity;

#[cfg(feature = "clap")]
pub mod clap_ext;
#[cfg(feature = "completions")]
pub mod completions;

// Re-export core semantic types at top-level for fast, ergonomic usage
pub use accessibility::AccessibilityMode;
pub use channel::SartorialOutput;
pub use components::{
    ActionBar, Choice, ChoiceOutcome, Confirm, ConfirmOutcome, DetailView, ErrorView, KeyValueList,
    MultiProgressView, NoticeView, ProgressBar, Section, StatusBadge, TableView, Title,
};
pub use config::{BorderStyle, ColorChoice, Config, Density, InteractiveMode, SymbolMode};
pub use exit::ExitCode;
pub use help::{HelpEntry, HelpView};
pub use hyperlink::Hyperlink;
pub use motion::{MotionMode, ProgressMode};
pub use pager::PagerMode;
pub use protocol::{ProgressEvent, ProtocolEnvelope};
pub use provenance::{ProvenanceFact, ProvenanceList, ProvenanceSource};
pub use render::{
    AgentRenderer, HumanRenderer, PlainRenderer, RenderAgent, RenderContext, RenderHuman,
    RenderPlain, RenderTarget,
};
pub use screens::{DetailScreen, ListScreen, SummaryScreen};
pub use semantic::{
    Action, ChangeKind, ChoiceItem, ErrorModel, Evidence, Fact, Notice, NoticeLevel, Outcome, Plan,
    PlanChange, ProgressError, ProgressState, Receipt, Status, TableModel, TableRow,
};
pub use style::{Preset, ProgressTreatment, ResolvedStyle};
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

/// Quick helper to render an item as agent JSON.
pub fn to_agent_json<T>(item: &T) -> Result<String, serde_json::Error>
where
    T: render::RenderAgent,
{
    item.to_agent_json(true)
}
