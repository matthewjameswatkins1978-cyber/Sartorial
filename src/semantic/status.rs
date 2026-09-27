use serde::{Deserialize, Serialize};

/// High-level operational status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Everything is operational, verified, or succeeded.
    Ready,
    /// Requires attention, incomplete, or warning condition.
    Attention,
    /// Operation failed, error occurred, or check did not pass.
    Failed,
    /// Currently running or executing.
    Running,
    /// Awaiting execution or queued.
    Pending,
    /// Skipped or not applicable.
    Skipped,
}

impl Status {
    /// Canonical text label for the status.
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

    /// Uppercase display label.
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

    /// Unicode glyph symbol.
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

    /// ASCII fallback symbol.
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

    /// Primary semantic color style for human terminal rendering.
    pub fn style(&self) -> anstyle::Style {
        match self {
            Self::Ready => anstyle::Style::new()
                .fg_color(Some(anstyle::AnsiColor::Green.into()))
                .bold(),
            Self::Attention => anstyle::Style::new()
                .fg_color(Some(anstyle::AnsiColor::Yellow.into()))
                .bold(),
            Self::Failed => anstyle::Style::new()
                .fg_color(Some(anstyle::AnsiColor::Red.into()))
                .bold(),
            Self::Running => anstyle::Style::new()
                .fg_color(Some(anstyle::AnsiColor::Cyan.into()))
                .bold(),
            Self::Pending => {
                anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::BrightBlack.into()))
            }
            Self::Skipped => {
                anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::BrightBlack.into()))
            }
        }
    }
}
