use super::action::Action;
use super::evidence::Evidence;
use serde::{Deserialize, Serialize};

/// Strongly typed error: WHAT happened, WHY (if known), WHAT NEXT.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorModel {
    pub what: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<Evidence>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next_actions: Vec<Action>,
}

impl ErrorModel {
    pub fn new(what: impl Into<String>) -> Self {
        Self {
            what: what.into(),
            why: None,
            evidence: Vec::new(),
            next_actions: Vec::new(),
        }
    }

    pub fn with_why(mut self, why: impl Into<String>) -> Self {
        self.why = Some(why.into());
        self
    }

    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.next_actions.push(action);
        self
    }

    pub fn is_cause_known(&self) -> bool {
        self.why.is_some()
    }
}
