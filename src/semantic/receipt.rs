use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};
use crate::semantic::action::Action;
use crate::semantic::fact::Fact;
use crate::semantic::notice::Notice;
use crate::semantic::status::Status;
use serde::{Deserialize, Serialize};

/// Semantic model for operation receipts AFTER a state-changing operation.
/// Expresses truthful changes, material non-changes, next actions, and guidance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub title: String,
    pub status: Status,
    pub changes: Vec<Fact>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub unchanged: Vec<Fact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<Notice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_handle: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub actions: Vec<Action>,
}

impl Receipt {
    pub fn success(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            status: Status::Ready,
            changes: Vec::new(),
            unchanged: Vec::new(),
            guidance: None,
            warnings: Vec::new(),
            evidence_handle: None,
            actions: Vec::new(),
        }
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn change(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.changes.push(Fact::new(label, value));
        self
    }

    pub fn unchanged(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.unchanged.push(Fact::new(label, value));
        self
    }

    pub fn guidance(mut self, text: impl Into<String>) -> Self {
        self.guidance = Some(text.into());
        self
    }

    pub fn warning(mut self, notice: Notice) -> Self {
        self.warnings.push(notice);
        self
    }

    pub fn with_evidence_handle(mut self, handle: impl Into<String>) -> Self {
        self.evidence_handle = Some(handle.into());
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
}

#[derive(Serialize)]
struct AgentReceiptRepresentation<'a> {
    schema_version: &'static str,
    title: &'a str,
    status: Status,
    changes: &'a [Fact],
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    unchanged: &'a [Fact],
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance: &'a Option<String>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    warnings: &'a [Notice],
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence_handle: &'a Option<String>,
    next_actions: Vec<&'a str>,
}

impl RenderAgent for Receipt {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentReceiptRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            title: &self.title,
            status: self.status,
            changes: &self.changes,
            unchanged: &self.unchanged,
            guidance: &self.guidance,
            warnings: &self.warnings,
            evidence_handle: &self.evidence_handle,
            next_actions: self.actions.iter().map(|a| a.id.as_str()).collect(),
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}
