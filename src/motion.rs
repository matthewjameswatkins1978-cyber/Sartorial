use serde::{Deserialize, Serialize};

/// Animation / motion policy for terminal progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MotionMode {
    /// Animate only when attached to an attended interactive TTY with no reduced motion requested.
    #[default]
    Auto,
    /// Always animate (useful for demos and testing).
    Always,
    /// Never animate; emit static or line-oriented progress states.
    Never,
}

impl MotionMode {
    /// Decide whether animation frames should tick live in the terminal.
    pub fn should_animate(
        &self,
        is_tty: bool,
        is_agent: bool,
        is_plain: bool,
        reduced_motion: bool,
    ) -> bool {
        if !is_tty || is_agent || is_plain || reduced_motion {
            return false;
        }
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Auto => true,
        }
    }
}

/// Semantic mode of progress reporting according to the BL Motion Standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressMode {
    /// Total work is UNKNOWN. Always favors elapsed count-up; never fabricates fake percentage.
    #[default]
    Activity,
    /// Completed / total items are known (e.g. 38 / 60 files).
    Count,
    /// Percentage derived from genuinely known progress (e.g. 63%).
    Percent,
    /// Real future timing event (e.g. "Retrying in 17s", "Waiting for service 04s").
    Countdown,
    /// Meaningful throughput rate (e.g. "84 MB / 140 MB  11 MB/s").
    Rate,
}
