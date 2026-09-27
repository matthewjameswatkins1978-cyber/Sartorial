use super::status::Status;
use crate::motion::ProgressMode;
use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};
use serde::{Deserialize, Serialize};

/// Semantic snapshot of an in-flight or completed task progress according to the BL Motion Standard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "UncheckedProgressState")]
#[non_exhaustive]
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

#[derive(Deserialize)]
struct UncheckedProgressState {
    task: String,
    subtask: Option<String>,
    mode: ProgressMode,
    current: Option<u64>,
    total: Option<u64>,
    percent: Option<u8>,
    unit: Option<String>,
    elapsed_secs: Option<u64>,
    rate: Option<String>,
    countdown_secs: Option<u64>,
    status: Status,
}

impl TryFrom<UncheckedProgressState> for ProgressState {
    type Error = ProgressError;

    fn try_from(raw: UncheckedProgressState) -> Result<Self, Self::Error> {
        if let (Some(current), Some(total)) = (raw.current, raw.total) {
            validate_count(current, total)?;
        }
        Ok(Self {
            task: raw.task,
            subtask: raw.subtask,
            mode: raw.mode,
            current: raw.current,
            total: raw.total,
            percent: raw.percent,
            unit: raw.unit,
            elapsed_secs: raw.elapsed_secs,
            rate: raw.rate,
            countdown_secs: raw.countdown_secs,
            status: raw.status,
        })
    }
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
    pub fn count(task: impl Into<String>, current: u64, total: u64) -> Result<Self, ProgressError> {
        validate_count(current, total)?;
        let mut s = Self::new(task);
        s.mode = ProgressMode::Count;
        s.current = Some(current);
        s.total = Some(total);
        s.percent = (current.saturating_mul(100))
            .checked_div(total)
            .map(|p| p.min(100) as u8);
        Ok(s)
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
    ) -> Result<Self, ProgressError> {
        let mut s = Self::count(task, current, total)?;
        s.mode = ProgressMode::Rate;
        s.unit = Some(unit.into());
        s.rate = Some(rate.into());
        Ok(s)
    }

    pub fn with_subtask(mut self, subtask: impl Into<String>) -> Self {
        self.subtask = Some(subtask.into());
        self
    }

    pub fn with_progress(
        mut self,
        current: u64,
        total: u64,
        unit: impl Into<String>,
    ) -> Result<Self, ProgressError> {
        validate_count(current, total)?;
        self.current = Some(current);
        self.total = Some(total);
        self.unit = Some(unit.into());
        self.percent = (current.saturating_mul(100))
            .checked_div(total)
            .map(|p| p.min(100) as u8);
        Ok(self)
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

    /// Derive the truthful progress percentage.
    ///
    /// Semantic authority rules:
    /// - If `current` and `total` are known:
    ///   - If `total > 0`, percent is derived: `(current * 100) / total`, bounded to 100.
    ///   - If `total == 0`, percent is `None` (never derive fake 100%).
    /// - Otherwise, if only explicit `percent` is known, returns bounded `percent`.
    pub fn derived_percent(&self) -> Option<u8> {
        match (self.current, self.total) {
            (Some(cur), Some(tot)) if tot > 0 => {
                Some(((cur.saturating_mul(100)) / tot).min(100) as u8)
            }
            (Some(_), Some(0)) => None,
            _ => self.percent.map(|p| p.min(100)),
        }
    }

    /// Reject invalid native count pairs before they cross a rendering boundary.
    pub fn validate(&self) -> Result<(), ProgressError> {
        if let (Some(current), Some(total)) = (self.current, self.total) {
            validate_count(current, total)?;
        }
        Ok(())
    }

    /// Apply an update enforcing single authority and semantic invariants:
    /// - `current > total` is rejected with `CurrentExceedsTotal`
    /// - `percent > 100` is rejected with `PercentOutOfRange`
    /// - If `current` and `total` exist (total > 0), percent is derived.
    ///   Any explicit percent MUST agree with derived percent or is rejected with `PercentContradiction`.
    /// - If `total == 0`, derived percent is None. Explicit percent is rejected with `PercentContradiction`.
    /// - If only `percent` is updated, mode transitions to `Percent` if no count exists.
    pub fn apply_update(
        &mut self,
        current: Option<u64>,
        percent: Option<u8>,
        rate: Option<String>,
        elapsed_secs: Option<u64>,
        subtask: Option<String>,
    ) -> Result<(), ProgressError> {
        let new_current = current.or(self.current);
        let new_total = self.total;

        if let (Some(cur), Some(tot)) = (new_current, new_total) {
            if cur > tot {
                return Err(ProgressError::CurrentExceedsTotal {
                    current: cur,
                    total: tot,
                });
            }
        }

        let derived = match (new_current, new_total) {
            (Some(cur), Some(tot)) if tot > 0 => {
                Some(((cur.saturating_mul(100)) / tot).min(100) as u8)
            }
            _ => None,
        };

        if let Some(exp) = percent {
            if exp > 100 {
                return Err(ProgressError::PercentOutOfRange { percent: exp });
            }
            if let Some(d) = derived {
                if exp != d {
                    return Err(ProgressError::PercentContradiction {
                        derived: Some(d),
                        explicit: exp,
                    });
                }
            } else if new_total == Some(0) {
                return Err(ProgressError::PercentContradiction {
                    derived: None,
                    explicit: exp,
                });
            }
            self.percent = Some(exp);
        } else if derived.is_some() {
            self.percent = derived;
        } else if new_total == Some(0) {
            self.percent = None;
        }

        if let Some(cur) = current {
            self.current = Some(cur);
            if self.mode == ProgressMode::Activity {
                self.mode = ProgressMode::Count;
            }
        }

        if percent.is_some() && self.current.is_none() && self.total.is_none() {
            self.mode = ProgressMode::Percent;
        }

        if let Some(r) = rate {
            self.rate = Some(r);
            if self.mode == ProgressMode::Count || self.mode == ProgressMode::Activity {
                self.mode = ProgressMode::Rate;
            }
        }

        if let Some(secs) = elapsed_secs {
            self.elapsed_secs = Some(secs);
        }

        if let Some(sub) = subtask {
            self.subtask = Some(sub);
        }

        Ok(())
    }

    /// Update current progress count and recompute percentage if total is known.
    pub fn update_current(&mut self, cur: u64) -> Result<(), ProgressError> {
        self.apply_update(Some(cur), None, None, None, None)
    }

    /// Update total count and recompute percentage if current is known.
    pub fn update_total(&mut self, tot: u64) -> Result<(), ProgressError> {
        if let Some(current) = self.current {
            validate_count(current, tot)?;
        }
        self.total = Some(tot);
        if let Some(cur) = self.current {
            self.percent = if tot > 0 {
                (cur.saturating_mul(100))
                    .checked_div(tot)
                    .map(|p| p.min(100) as u8)
            } else {
                None
            };
        }
        Ok(())
    }

    /// Update explicit percentage and set mode to Percent if appropriate.
    pub fn update_percent(&mut self, pct: u8) -> Result<(), ProgressError> {
        self.apply_update(None, Some(pct), None, None, None)
    }

    /// Update rate and set mode to Rate if units/progress exist.
    pub fn update_rate(&mut self, rate: impl Into<String>) {
        self.rate = Some(rate.into());
        if self.mode == ProgressMode::Count || self.mode == ProgressMode::Activity {
            self.mode = ProgressMode::Rate;
        }
    }

    /// Update subtask.
    pub fn update_subtask(&mut self, subtask: impl Into<String>) {
        self.subtask = Some(subtask.into());
    }

    /// Update elapsed seconds.
    pub fn update_elapsed(&mut self, secs: u64) {
        self.elapsed_secs = Some(secs);
    }
}

fn validate_count(current: u64, total: u64) -> Result<(), ProgressError> {
    if current > total {
        Err(ProgressError::CurrentExceedsTotal { current, total })
    } else {
        Ok(())
    }
}

/// Progress protocol and semantic validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgressError {
    CurrentExceedsTotal { current: u64, total: u64 },
    PercentContradiction { derived: Option<u8>, explicit: u8 },
    PercentOutOfRange { percent: u8 },
}

impl std::fmt::Display for ProgressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurrentExceedsTotal { current, total } => {
                write!(f, "progress current ({current}) exceeds total ({total})")
            }
            Self::PercentContradiction {
                derived: Some(d),
                explicit,
            } => {
                write!(
                    f,
                    "explicit percent ({explicit}%) contradicts derived percent ({d}%)"
                )
            }
            Self::PercentContradiction {
                derived: None,
                explicit,
            } => {
                write!(
                    f,
                    "explicit percent ({explicit}%) supplied when total is zero/unavailable"
                )
            }
            Self::PercentOutOfRange { percent } => {
                write!(f, "percent ({percent}) exceeds maximum allowed 100%")
            }
        }
    }
}

impl std::error::Error for ProgressError {}

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
        self.validate().map_err(serde::ser::Error::custom)?;
        let rep = AgentProgressRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            task: &self.task,
            subtask: &self.subtask,
            mode: self.mode,
            current: self.current,
            total: self.total,
            percent: self.derived_percent(),
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
