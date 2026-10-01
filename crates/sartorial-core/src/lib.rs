//! sartorial-core: tiny deterministic presentation engine.
//!
//! Applications own truth. Sartorial owns presentation.
//!
//! Core turns semantic meaning into a [`Document`](crate::Document),
//! resolves [`Preset`] behaviour plus [`Theme`] identity plus
//! [`Capabilities`] into one [`ResolvedStyle`], then renders to
//! terminal, plain, or Markdown. It never touches live terminals,
//! progress machinery, prompts, CLIs, or application schemas.

pub mod capabilities;
pub mod document;
pub mod preset;
pub mod render;
pub mod semantic;
pub mod style;
pub mod text;
pub mod theme;

pub use capabilities::{Capabilities, ColorPolicy, Interactive, SymbolMode};
pub use document::{Block, Document, Presentable};
pub use preset::{
    BorderStyle, Density, Preset, ProgressTreatment, SectionRule, StatusLayout, TitleCase,
};
pub use render::{MarkdownRenderer, PlainRenderer, RenderTarget, TerminalRenderer};
pub use semantic::{
    Action, ChangeKind, ChoiceItem, ColumnAlignment, ErrorModel, Evidence, Fact, KeyTrigger,
    Notice, NoticeLevel, Outcome, Plan, PlanChange, ProgressError, ProgressMode, ProgressState,
    Receipt, Status, TableModel, TableRow,
};
pub use style::ResolvedStyle;
pub use theme::{Theme, ThemeBuilder};
