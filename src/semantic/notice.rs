use serde::{Deserialize, Serialize};

/// Severity of a notice or callout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeLevel {
    /// Informational notice.
    Info,
    /// Proactive tip or helpful suggestion.
    Tip,
    /// Warning requiring caution or attention.
    Warning,
    /// Actionable error or failure notice.
    Error,
}

impl NoticeLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Tip => "tip",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    pub fn unicode_glyph(&self) -> &'static str {
        match self {
            Self::Info => "ℹ",
            Self::Tip => "💡",
            Self::Warning => "!",
            Self::Error => "×",
        }
    }

    pub fn ascii_glyph(&self) -> &'static str {
        match self {
            Self::Info => "[info]",
            Self::Tip => "[tip]",
            Self::Warning => "[!]",
            Self::Error => "[X]",
        }
    }

    pub fn style(&self) -> anstyle::Style {
        match self {
            Self::Info => anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Cyan.into())),
            Self::Tip => anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into())),
            Self::Warning => anstyle::Style::new()
                .fg_color(Some(anstyle::AnsiColor::Yellow.into()))
                .bold(),
            Self::Error => anstyle::Style::new()
                .fg_color(Some(anstyle::AnsiColor::Red.into()))
                .bold(),
        }
    }
}

/// A bounded notification, callout, or advisory message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notice {
    /// Severity level.
    pub level: NoticeLevel,
    /// Main headline or summary message.
    pub message: String,
    /// Optional secondary detail or guidance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl Notice {
    pub fn new(level: NoticeLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            detail: None,
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(NoticeLevel::Info, message)
    }

    pub fn tip(message: impl Into<String>) -> Self {
        Self::new(NoticeLevel::Tip, message)
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(NoticeLevel::Warning, message)
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(NoticeLevel::Error, message)
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}
