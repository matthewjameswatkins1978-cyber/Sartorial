//! Language-neutral presentation wire protocol (`sartorial.v0.1`).
//! Optional surface behind the `wire` feature: compatibility with the
//! current protocol is preserved, but application machine schemas stay
//! application-owned. This is presentation wire format, not business schema.

#![cfg(feature = "wire")]

use crate::components::choice::Choice;
use crate::components::confirm::Confirm;
use crate::components::error::ErrorView;
use crate::components::table::TableView;
use crate::render::context::RenderContext;
use crate::render::{RenderAgent, RenderHuman, RenderPlain, SARTORIAL_SCHEMA_VERSION};
use crate::screens::summary::SummaryScreen;
use crate::semantic::action::Action;
use crate::semantic::choice::ChoiceItem;
use crate::semantic::error::ErrorModel;
use crate::semantic::evidence::Evidence;
use crate::semantic::fact::Fact;
use crate::semantic::notice::Notice;
use crate::semantic::plan::{Plan, PlanChange};
use crate::semantic::receipt::Receipt;
use crate::semantic::status::Status;
use crate::semantic::table::TableModel;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// Top-level protocol envelope for language-neutral Sartorial messages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProtocolEnvelope {
    Summary(SummaryPayload),
    Table(TablePayload),
    Error(ErrorPayload),
    Plan(PlanPayload),
    Receipt(ReceiptPayload),
    Confirm(ConfirmPayload),
    Choice(ChoicePayload),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryPayload {
    pub schema_version: String,
    pub title: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub facts: Vec<Fact>,
    #[serde(default)]
    pub notices: Vec<Notice>,
    #[serde(default)]
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TablePayload {
    pub schema_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<String>,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub schema_version: String,
    pub what: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
    #[serde(default)]
    pub evidence: Vec<Evidence>,
    #[serde(default)]
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanPayload {
    pub schema_version: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub changes: Vec<PlanChange>,
    #[serde(default)]
    pub consequences: Vec<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversible: Option<bool>,
    #[serde(default)]
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReceiptPayload {
    pub schema_version: String,
    pub title: String,
    pub status: Status,
    #[serde(default)]
    pub changes: Vec<Fact>,
    #[serde(default)]
    pub unchanged: Vec<Fact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    #[serde(default)]
    pub warnings: Vec<Notice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_handle: Option<String>,
    #[serde(default)]
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfirmPayload {
    pub schema_version: String,
    pub prompt: String,
    #[serde(default = "default_true")]
    pub default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_interactive_fallback: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoicePayload {
    pub schema_version: String,
    pub prompt: String,
    pub items: Vec<ChoiceItem>,
    #[serde(default)]
    pub default_index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_interactive_fallback: Option<usize>,
}

pub fn default_schema_version() -> String {
    SARTORIAL_SCHEMA_VERSION.to_string()
}

fn default_true() -> bool {
    true
}

/// Protocol error type for inbound message validation.
#[derive(Debug)]
pub enum ProtocolError {
    UnsupportedVersion { expected: String, actual: String },
    Progress(crate::semantic::ProgressError),
    Json(serde_json::Error),
    Io(io::Error),
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedVersion { expected, actual } => {
                write!(
                    f,
                    "unsupported protocol schema_version: got {actual}, expected {expected}"
                )
            }
            Self::Progress(e) => write!(f, "protocol progress violation: {e}"),
            Self::Json(e) => write!(f, "Invalid JSON: {e}"),
            Self::Io(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl std::error::Error for ProtocolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Progress(e) => Some(e),
            Self::Json(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<crate::semantic::ProgressError> for ProtocolError {
    fn from(e: crate::semantic::ProgressError) -> Self {
        Self::Progress(e)
    }
}

impl From<serde_json::Error> for ProtocolError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<io::Error> for ProtocolError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

/// Centrally validate schema version boundary.
pub fn validate_schema_version(version: &str) -> Result<(), ProtocolError> {
    if version == SARTORIAL_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(ProtocolError::UnsupportedVersion {
            expected: SARTORIAL_SCHEMA_VERSION.to_string(),
            actual: version.to_string(),
        })
    }
}

impl ProtocolEnvelope {
    pub fn schema_version(&self) -> &str {
        match self {
            Self::Summary(p) => &p.schema_version,
            Self::Table(p) => &p.schema_version,
            Self::Error(p) => &p.schema_version,
            Self::Plan(p) => &p.schema_version,
            Self::Receipt(p) => &p.schema_version,
            Self::Confirm(p) => &p.schema_version,
            Self::Choice(p) => &p.schema_version,
        }
    }

    pub fn validate_version(&self) -> Result<(), ProtocolError> {
        validate_schema_version(self.schema_version())
    }

    pub fn from_json_str(s: &str) -> Result<Self, ProtocolError> {
        let envelope: Self = serde_json::from_str(s)?;
        envelope.validate_version()?;
        Ok(envelope)
    }
}

/// Streaming JSONL progress event for language-neutral live updates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProgressEvent {
    #[serde(rename = "progress.start")]
    Start {
        #[serde(default = "default_schema_version")]
        schema_version: String,
        id: String,
        activity: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtask: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        total: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        unit: Option<String>,
    },
    #[serde(rename = "progress.update")]
    Update {
        #[serde(default = "default_schema_version")]
        schema_version: String,
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        current: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        percent: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        rate: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        elapsed_secs: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtask: Option<String>,
    },
    #[serde(rename = "progress.finish")]
    Finish {
        #[serde(default = "default_schema_version")]
        schema_version: String,
        id: String,
        #[serde(default = "default_ready_status")]
        status: Status,
    },
}

impl ProgressEvent {
    pub fn schema_version(&self) -> &str {
        match self {
            Self::Start { schema_version, .. } => schema_version,
            Self::Update { schema_version, .. } => schema_version,
            Self::Finish { schema_version, .. } => schema_version,
        }
    }

    pub fn validate_version(&self) -> Result<(), ProtocolError> {
        validate_schema_version(self.schema_version())
    }

    pub fn from_json_str(s: &str) -> Result<Self, ProtocolError> {
        let event: Self = serde_json::from_str(s)?;
        event.validate_version()?;
        Ok(event)
    }
}

/// Typed result payload for external `sartorial confirm` invocations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfirmResult {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
}

impl ConfirmResult {
    pub fn confirmed() -> Self {
        Self {
            status: "confirmed".to_string(),
            confirmed: None,
        }
    }

    pub fn denied() -> Self {
        Self {
            status: "denied".to_string(),
            confirmed: None,
        }
    }

    pub fn non_interactive_fallback(confirmed: bool) -> Self {
        Self {
            status: "non_interactive_fallback".to_string(),
            confirmed: Some(confirmed),
        }
    }

    pub fn non_interactive_denied() -> Self {
        Self {
            status: "non_interactive_denied".to_string(),
            confirmed: None,
        }
    }

    pub fn cancelled() -> Self {
        Self {
            status: "cancelled".to_string(),
            confirmed: None,
        }
    }

    pub fn from_outcome(outcome: &crate::components::confirm::ConfirmOutcome) -> Self {
        use crate::components::confirm::ConfirmOutcome;
        match outcome {
            ConfirmOutcome::Confirmed => Self::confirmed(),
            ConfirmOutcome::Denied => Self::denied(),
            ConfirmOutcome::NonInteractiveFallback(c) => Self::non_interactive_fallback(*c),
            ConfirmOutcome::NonInteractiveDenied => Self::non_interactive_denied(),
            ConfirmOutcome::Cancelled => Self::cancelled(),
        }
    }

    pub fn is_confirmed(&self) -> bool {
        self.status == "confirmed" || self.confirmed == Some(true)
    }
}

/// Typed result payload for external `sartorial choice` invocations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChoiceResult {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl ChoiceResult {
    pub fn selected(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            status: "selected".to_string(),
            id: Some(id.into()),
            label: Some(label.into()),
        }
    }

    pub fn non_interactive_fallback(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            status: "non_interactive_fallback".to_string(),
            id: Some(id.into()),
            label: Some(label.into()),
        }
    }

    pub fn non_interactive_denied() -> Self {
        Self {
            status: "non_interactive_denied".to_string(),
            id: None,
            label: None,
        }
    }

    pub fn cancelled() -> Self {
        Self {
            status: "cancelled".to_string(),
            id: None,
            label: None,
        }
    }

    pub fn from_outcome(outcome: &crate::components::choice::ChoiceOutcome) -> Self {
        use crate::components::choice::ChoiceOutcome;
        match outcome {
            ChoiceOutcome::Selected(item) => Self::selected(&item.id, &item.label),
            ChoiceOutcome::NonInteractiveFallback(item) => {
                Self::non_interactive_fallback(&item.id, &item.label)
            }
            ChoiceOutcome::NonInteractiveDenied => Self::non_interactive_denied(),
            ChoiceOutcome::Cancelled => Self::cancelled(),
        }
    }
}

fn default_ready_status() -> Status {
    Status::Ready
}

impl RenderHuman for ProtocolEnvelope {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        match self {
            Self::Summary(p) => {
                let mut screen = SummaryScreen::new(&p.title, p.status);
                if let Some(ref sub) = p.subtitle {
                    screen = screen.with_subtitle(sub);
                }
                for f in &p.facts {
                    screen = screen.fact(&f.name, &f.value);
                }
                for n in &p.notices {
                    screen = screen.notice(n.clone());
                }
                for a in &p.actions {
                    screen = screen.action(a.clone());
                }
                screen.render_human(ctx, out)
            }
            Self::Table(p) => {
                let headers: Vec<&str> = p.headers.iter().map(|s| s.as_str()).collect();
                let mut model = TableModel::new(headers);
                if let Some(ref t) = p.title {
                    model = model.with_title(t);
                }
                if let Some(ref b) = p.badge {
                    model = model.with_badge(b);
                }
                for row in &p.rows {
                    model.add_row(row.iter().map(|s| s.as_str()));
                }
                let view = TableView::new(model);
                view.render_human(ctx, out)
            }
            Self::Error(p) => {
                let mut err = ErrorModel::new(&p.what);
                if let Some(ref why) = p.why {
                    err = err.with_why(why);
                }
                for ev in &p.evidence {
                    err = err.with_evidence(ev.clone());
                }
                for a in &p.actions {
                    err = err.with_action(a.clone());
                }
                let view = ErrorView::new(err);
                view.render_human(ctx, out)
            }
            Self::Plan(p) => {
                let mut plan = Plan::new(&p.title);
                if let Some(ref desc) = p.description {
                    plan = plan.with_description(desc);
                }
                for change in &p.changes {
                    plan = plan.add_change(change.clone());
                }
                for c in &p.consequences {
                    plan = plan.consequence(c);
                }
                for w in &p.warnings {
                    plan = plan.warning(w);
                }
                if let Some(rev) = p.reversible {
                    plan = plan.reversible(rev);
                }
                for a in &p.actions {
                    plan = plan.with_action(a.clone());
                }
                plan.render_human(ctx, out)
            }
            Self::Receipt(p) => {
                let mut receipt = Receipt::success(&p.title).with_status(p.status);
                for c in &p.changes {
                    receipt = receipt.change(&c.name, &c.value);
                }
                for u in &p.unchanged {
                    receipt = receipt.unchanged(&u.name, &u.value);
                }
                if let Some(ref g) = p.guidance {
                    receipt = receipt.guidance(g);
                }
                for w in &p.warnings {
                    receipt = receipt.warning(w.clone());
                }
                if let Some(ref h) = p.evidence_handle {
                    receipt = receipt.with_evidence_handle(h);
                }
                for a in &p.actions {
                    receipt = receipt.with_action(a.clone());
                }
                receipt.render_human(ctx, out)
            }
            Self::Confirm(p) => {
                let mut confirm = Confirm::new(&p.prompt).with_default(p.default);
                if let Some(fb) = p.non_interactive_fallback {
                    confirm = confirm.with_non_interactive_fallback(fb);
                }
                confirm.render_human(ctx, out)
            }
            Self::Choice(p) => {
                let mut choice =
                    Choice::new(&p.prompt, p.items.clone()).with_default_index(p.default_index);
                if let Some(fb) = p.non_interactive_fallback {
                    choice = choice.with_non_interactive_fallback(fb);
                }
                choice.render_human(ctx, out)
            }
        }
    }
}

impl RenderPlain for ProtocolEnvelope {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        match self {
            Self::Summary(p) => {
                let mut screen = SummaryScreen::new(&p.title, p.status);
                if let Some(ref sub) = p.subtitle {
                    screen = screen.with_subtitle(sub);
                }
                for f in &p.facts {
                    screen = screen.fact(&f.name, &f.value);
                }
                for n in &p.notices {
                    screen = screen.notice(n.clone());
                }
                for a in &p.actions {
                    screen = screen.action(a.clone());
                }
                screen.render_plain(ctx, out)
            }
            Self::Table(p) => {
                let headers: Vec<&str> = p.headers.iter().map(|s| s.as_str()).collect();
                let mut model = TableModel::new(headers);
                if let Some(ref t) = p.title {
                    model = model.with_title(t);
                }
                if let Some(ref b) = p.badge {
                    model = model.with_badge(b);
                }
                for row in &p.rows {
                    model.add_row(row.iter().map(|s| s.as_str()));
                }
                let view = TableView::new(model);
                view.render_plain(ctx, out)
            }
            Self::Error(p) => {
                let mut err = ErrorModel::new(&p.what);
                if let Some(ref why) = p.why {
                    err = err.with_why(why);
                }
                for ev in &p.evidence {
                    err = err.with_evidence(ev.clone());
                }
                for a in &p.actions {
                    err = err.with_action(a.clone());
                }
                let view = ErrorView::new(err);
                view.render_plain(ctx, out)
            }
            Self::Plan(p) => {
                let mut plan = Plan::new(&p.title);
                if let Some(ref desc) = p.description {
                    plan = plan.with_description(desc);
                }
                for change in &p.changes {
                    plan = plan.add_change(change.clone());
                }
                for c in &p.consequences {
                    plan = plan.consequence(c);
                }
                for w in &p.warnings {
                    plan = plan.warning(w);
                }
                if let Some(rev) = p.reversible {
                    plan = plan.reversible(rev);
                }
                for a in &p.actions {
                    plan = plan.with_action(a.clone());
                }
                plan.render_plain(ctx, out)
            }
            Self::Receipt(p) => {
                let mut receipt = Receipt::success(&p.title).with_status(p.status);
                for c in &p.changes {
                    receipt = receipt.change(&c.name, &c.value);
                }
                for u in &p.unchanged {
                    receipt = receipt.unchanged(&u.name, &u.value);
                }
                if let Some(ref g) = p.guidance {
                    receipt = receipt.guidance(g);
                }
                for w in &p.warnings {
                    receipt = receipt.warning(w.clone());
                }
                if let Some(ref h) = p.evidence_handle {
                    receipt = receipt.with_evidence_handle(h);
                }
                for a in &p.actions {
                    receipt = receipt.with_action(a.clone());
                }
                receipt.render_plain(ctx, out)
            }
            Self::Confirm(p) => {
                let mut confirm = Confirm::new(&p.prompt).with_default(p.default);
                if let Some(fb) = p.non_interactive_fallback {
                    confirm = confirm.with_non_interactive_fallback(fb);
                }
                confirm.render_plain(ctx, out)
            }
            Self::Choice(p) => {
                let mut choice =
                    Choice::new(&p.prompt, p.items.clone()).with_default_index(p.default_index);
                if let Some(fb) = p.non_interactive_fallback {
                    choice = choice.with_non_interactive_fallback(fb);
                }
                choice.render_plain(ctx, out)
            }
        }
    }
}

impl RenderAgent for ProtocolEnvelope {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        if pretty {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
    }
}
