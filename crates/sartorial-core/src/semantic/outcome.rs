use super::action::Action;
use super::evidence::Evidence;
use super::fact::Fact;
use super::notice::Notice;
use super::status::Status;
use serde::{Deserialize, Serialize};

/// High-level semantic result: the single authority of application truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    pub status: Status,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facts: Vec<Fact>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<Evidence>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Notice>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl Outcome {
    pub fn new(status: Status, title: impl Into<String>) -> Self {
        Self {
            status,
            title: title.into(),
            summary: None,
            facts: Vec::new(),
            evidence: Vec::new(),
            warnings: Vec::new(),
            actions: Vec::new(),
            details: None,
        }
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn with_fact(mut self, fact: Fact) -> Self {
        self.facts.push(fact);
        self
    }

    pub fn fact(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.with_fact(Fact::new(name, value))
    }

    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    pub fn with_warning(mut self, warning: Notice) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}
