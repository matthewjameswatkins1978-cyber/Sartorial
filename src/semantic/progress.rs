use super::status::Status;
use crate::motion::ProgressMode;
use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};
use serde::{Deserialize, Serialize};

/// Semantic snapshot of an in-flight or completed task progress according to the BL Motion Standard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressState {
    /// High-level task name (e.g. "Checking repository").
    pub task: String,
    /// Current specific activity (e.g. "cargo test").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtask: Option<String>,
    /// Semantic progress mode (Activity, Count, Percent, Countdown, Rate).
    pub mode: ProgressMode,
    /// Completed units count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<u64>,
    /// Total units count if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    /// Derived or explicit progress percentage (0..=100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<u8>,
    /// Unit label (e.g. "files", "bytes", "MB").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Elapsed seconds (honest count-up).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_secs: Option<u64>,
    /// Throughput rate (e.g. "11 MB/s").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// Real future event countdown in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countdown_secs: Option<u64>,
    /// Operational status.
    pub status: Status,
}

impl ProgressState {
    pub fn new(task: impl Into<String>) -> Self {
        Self {
            task: task.into(),
            subtask: None,
            mode: ProgressMode::Activity,
            current: None,
            total: None,
            percent: None,
            unit: None,
            elapsed_secs: None,
            rate: None,
            countdown_secs: None,
            status: Status::Running,
        }
    }

    /// Task with unknown total. Favors elapsed count-up; never fabricates fake percentage.
    pub fn activity(task: impl Into<String>) -> Self {
        Self::new(task)
    }

    /// Task with known completed and total items.
    pub fn count(task: impl Into<String>, current: u64, total: u64) -> Self {
        let mut s = Self::new(task);
        s.mode = ProgressMode::Count;
        s.current = Some(current);
        s.total = Some(total);
        s.percent = (current.saturating_mul(100))
            .checked_div(total)
            .map(|p| p.min(100) as u8);
        s
    }

    /// Task with percentage derived from known progress.
    pub fn percent(task: impl Into<String>, percent: u8) -> Self {
        let mut s = Self::new(task);
        s.mode = ProgressMode::Percent;
        s.percent = Some(percent.min(100));
        s
    }

    /// Real future timing event countdown.
    pub fn countdown(task: impl Into<String>, remaining_secs: u64) -> Self {
        let mut s = Self::new(task);
        s.mode = ProgressMode::Countdown;
        s.countdown_secs = Some(remaining_secs);
        s
    }

    /// Meaningful throughput rate.
    pub fn rate(
        task: impl Into<String>,
        current: u64,
        total: u64,
        unit: impl Into<String>,
        rate: impl Into<String>,
    ) -> Self {
        let mut s = Self::count(task, current, total);
        s.mode = ProgressMode::Rate;
        s.unit = Some(unit.into());
        s.rate = Some(rate.into());
        s
    }

    pub fn with_subtask(mut self, subtask: impl Into<String>) -> Self {
        self.subtask = Some(subtask.into());
        self
    }

    pub fn with_progress(mut self, current: u64, total: u64, unit: impl Into<String>) -> Self {
        self.current = Some(current);
        self.total = Some(total);
        self.unit = Some(unit.into());
        self.percent = (current.saturating_mul(100))
            .checked_div(total)
            .map(|p| p.min(100) as u8);
        self
    }

    pub fn with_elapsed(mut self, seconds: u64) -> Self {
        self.elapsed_secs = Some(seconds);
        self
    }

    pub fn with_rate(mut self, rate: impl Into<String>) -> Self {
        self.rate = Some(rate.into());
        self
    }

    pub fn with_countdown(mut self, seconds: u64) -> Self {
        self.countdown_secs = Some(seconds);
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }
}

#[derive(Serialize)]
struct AgentProgressRepresentation<'a> {
    schema_version: &'static str,
    task: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtask: &'a Option<String>,
    mode: ProgressMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    current: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elapsed_secs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    countdown_secs: Option<u64>,
    status: Status,
}

impl RenderAgent for ProgressState {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentProgressRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            task: &self.task,
            subtask: &self.subtask,
            mode: self.mode,
            current: self.current,
            total: self.total,
            percent: self.percent,
            unit: &self.unit,
            elapsed_secs: self.elapsed_secs,
            rate: &self.rate,
            countdown_secs: self.countdown_secs,
            status: self.status,
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}
