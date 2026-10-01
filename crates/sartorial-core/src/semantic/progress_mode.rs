use serde::{Deserialize, Serialize};

/// Semantic mode of progress reporting. The honesty rule lives here:
/// unknown work gets activity, never a fabricated percentage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressMode {
    #[default]
    Activity,
    Count,
    Percent,
    Countdown,
    Rate,
}
