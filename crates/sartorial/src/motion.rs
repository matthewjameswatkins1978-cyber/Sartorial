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

/// Semantic mode of progress reporting (canonical type lives in core).
/// Re-exported here so `use sartorial::motion::ProgressMode` keeps working.
pub use sartorial_core::ProgressMode;
