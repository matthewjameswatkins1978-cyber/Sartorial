use serde::{Deserialize, Serialize};

/// High-level operational status. Meaning lives here; colour lives in [`crate::Theme`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ready,
    Attention,
    Failed,
    Running,
    Pending,
    Skipped,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Attention => "attention",
            Self::Failed => "failed",
            Self::Running => "running",
            Self::Pending => "pending",
            Self::Skipped => "skipped",
        }
    }

    pub fn display_label(&self) -> &'static str {
        match self {
            Self::Ready => "READY",
            Self::Attention => "ATTENTION",
            Self::Failed => "FAILED",
            Self::Running => "RUNNING",
            Self::Pending => "PENDING",
            Self::Skipped => "SKIPPED",
        }
    }

    pub fn unicode_glyph(&self) -> &'static str {
        match self {
            Self::Ready => "✓",
            Self::Attention => "!",
            Self::Failed => "×",
            Self::Running => "●",
            Self::Pending => "○",
            Self::Skipped => "–",
        }
    }

    pub fn ascii_glyph(&self) -> &'static str {
        match self {
            Self::Ready => "[OK]",
            Self::Attention => "[!]",
            Self::Failed => "[X]",
            Self::Running => "[*]",
            Self::Pending => "[.]",
            Self::Skipped => "[-]",
        }
    }

    /// Semantic colour role for this status; the [`crate::Theme`] maps it to paint.
    pub fn theme_role(&self) -> &'static str {
        match self {
            Self::Ready => "success",
            Self::Attention => "warning",
            Self::Failed => "failure",
            Self::Running => "accent",
            Self::Pending => "muted",
            Self::Skipped => "muted",
        }
    }
}
