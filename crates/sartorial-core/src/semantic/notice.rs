use serde::{Deserialize, Serialize};

/// Severity of a notice or callout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeLevel {
    Info,
    Tip,
    Warning,
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
            Self::Error => "x",
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

    /// Semantic colour role for this level; the [`crate::Theme`] maps it to paint.
    pub fn theme_role(&self) -> &'static str {
        match self {
            Self::Info => "accent",
            Self::Tip => "accent",
            Self::Warning => "warning",
            Self::Error => "failure",
        }
    }
}

/// A bounded notification, callout, or advisory message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notice {
    pub level: NoticeLevel,
    pub message: String,
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
