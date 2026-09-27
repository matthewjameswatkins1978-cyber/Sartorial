use serde::{Deserialize, Serialize};

/// Standard key trigger for an action.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyTrigger {
    Char(char),
    Enter,
    Esc,
    Space,
    Up,
    Down,
    Left,
    Right,
    Custom(String),
}

impl KeyTrigger {
    pub fn display_tag(&self) -> String {
        match self {
            Self::Char(c) => c.to_uppercase().to_string(),
            Self::Enter => "Enter".to_string(),
            Self::Esc => "Esc".to_string(),
            Self::Space => "Space".to_string(),
            Self::Up => "↑".to_string(),
            Self::Down => "↓".to_string(),
            Self::Left => "←".to_string(),
            Self::Right => "→".to_string(),
            Self::Custom(s) => s.clone(),
        }
    }

    pub fn matches_char(&self, c: char) -> bool {
        match self {
            Self::Char(target) => target.eq_ignore_ascii_case(&c),
            _ => false,
        }
    }
}

/// An interactive or semantic action available to a human or agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    /// Programmatic identifier (e.g. "details", "install", "retry").
    pub id: String,
    /// Display label for human presentation (e.g. "Details", "Install").
    pub label: String,
    /// Triggering keyboard key.
    pub trigger: KeyTrigger,
    /// Optional expanded description for contextual help.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this is the default action (e.g. on Enter).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_default: bool,
}

impl Action {
    /// Create a custom character-triggered action.
    pub fn new(key: char, id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            trigger: KeyTrigger::Char(key),
            description: None,
            is_default: false,
        }
    }

    /// Set an expanded description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Mark as default action.
    pub fn default_action(mut self) -> Self {
        self.is_default = true;
        self
    }

    // Standard Biscuit Logic Grammar actions:

    /// [Enter] Open / Accept
    pub fn open() -> Self {
        Self {
            id: "open".into(),
            label: "Open".into(),
            trigger: KeyTrigger::Enter,
            description: Some("Open or accept current selection".into()),
            is_default: true,
        }
    }

    /// [Esc] Back / Cancel
    pub fn back() -> Self {
        Self {
            id: "back".into(),
            label: "Back".into(),
            trigger: KeyTrigger::Esc,
            description: Some("Return to previous screen or cancel".into()),
            is_default: false,
        }
    }

    /// [Esc] Cancel
    pub fn cancel() -> Self {
        Self {
            id: "cancel".into(),
            label: "Cancel".into(),
            trigger: KeyTrigger::Esc,
            description: Some("Cancel operation".into()),
            is_default: false,
        }
    }

    /// [/] Find / Search
    pub fn find() -> Self {
        Self {
            id: "find".into(),
            label: "Find".into(),
            trigger: KeyTrigger::Char('/'),
            description: Some("Search or filter items".into()),
            is_default: false,
        }
    }

    /// [?] Help
    pub fn help() -> Self {
        Self {
            id: "help".into(),
            label: "Help".into(),
            trigger: KeyTrigger::Char('?'),
            description: Some("Show contextual help".into()),
            is_default: false,
        }
    }

    /// [D] Details
    pub fn details() -> Self {
        Self {
            id: "details".into(),
            label: "Details".into(),
            trigger: KeyTrigger::Char('d'),
            description: Some("Show detailed evidence and inspection data".into()),
            is_default: false,
        }
    }

    /// [R] Retry / Refresh
    pub fn retry() -> Self {
        Self {
            id: "retry".into(),
            label: "Retry".into(),
            trigger: KeyTrigger::Char('r'),
            description: Some("Retry failed operation or refresh data".into()),
            is_default: false,
        }
    }

    /// [Q] Quit
    pub fn quit() -> Self {
        Self {
            id: "quit".into(),
            label: "Quit".into(),
            trigger: KeyTrigger::Char('q'),
            description: Some("Exit application".into()),
            is_default: false,
        }
    }
}
