use super::status::Status;
use serde::{Deserialize, Serialize};

/// Semantic snapshot of an in-flight or completed task progress.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressState {
    /// High-level task name (e.g. "Checking repository").
    pub task: String,
    /// Current specific activity (e.g. "cargo test").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtask: Option<String>,
    /// Completed units count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<u64>,
    /// Total units count if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    /// Unit label (e.g. "files", "bytes").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Elapsed seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_secs: Option<u64>,
    /// Operational status.
    pub status: Status,
}

impl ProgressState {
    pub fn new(task: impl Into<String>) -> Self {
        Self {
            task: task.into(),
            subtask: None,
            current: None,
            total: None,
            unit: None,
            elapsed_secs: None,
            status: Status::Running,
        }
    }

    pub fn with_subtask(mut self, subtask: impl Into<String>) -> Self {
        self.subtask = Some(subtask.into());
        self
    }

    pub fn with_progress(mut self, current: u64, total: u64, unit: impl Into<String>) -> Self {
        self.current = Some(current);
        self.total = Some(total);
        self.unit = Some(unit.into());
        self
    }

    pub fn with_elapsed(mut self, seconds: u64) -> Self {
        self.elapsed_secs = Some(seconds);
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }
}

use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};

#[derive(Serialize)]
struct AgentProgressRepresentation<'a> {
    schema_version: &'static str,
    task: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtask: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elapsed_secs: Option<u64>,
    status: Status,
}

impl RenderAgent for ProgressState {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentProgressRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            task: &self.task,
            subtask: &self.subtask,
            current: self.current,
            total: self.total,
            unit: &self.unit,
            elapsed_secs: self.elapsed_secs,
            status: self.status,
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}
