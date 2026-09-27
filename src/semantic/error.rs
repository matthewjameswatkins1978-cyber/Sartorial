use super::action::Action;
use super::evidence::Evidence;
use serde::{Deserialize, Serialize};

/// Strongly typed Sartorial error structure answering:
/// 1. WHAT HAPPENED?
/// 2. WHY? (preserved as unknown if unestablished)
/// 3. WHAT CAN I DO NEXT?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorModel {
    /// WHAT HAPPENED? Brief, clear title/statement of failure.
    pub what: String,
    /// WHY? Specific established root cause, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
    /// Evidence supporting the error (paths, lines, snippets, logs).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<Evidence>,
    /// Actionable next steps available to the user or agent.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next_actions: Vec<Action>,
}

impl ErrorModel {
    /// Create a new error with known or unknown cause.
    pub fn new(what: impl Into<String>) -> Self {
        Self {
            what: what.into(),
            why: None,
            evidence: Vec::new(),
            next_actions: Vec::new(),
        }
    }

    /// Add the verified cause.
    pub fn with_why(mut self, why: impl Into<String>) -> Self {
        self.why = Some(why.into());
        self
    }

    /// Add supporting evidence.
    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    /// Add an actionable next step.
    pub fn with_action(mut self, action: Action) -> Self {
        self.next_actions.push(action);
        self
    }

    /// Check if the root cause is established.
    pub fn is_cause_known(&self) -> bool {
        self.why.is_some()
    }
}

use crate::components::error::ErrorView;
use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use std::io::{self, Write};

impl RenderHuman for ErrorModel {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        ErrorView::new(self.clone()).render_human(ctx, out)
    }
}

impl RenderPlain for ErrorModel {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        ErrorView::new(self.clone()).render_plain(ctx, out)
    }
}
