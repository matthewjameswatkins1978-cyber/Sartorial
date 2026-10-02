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
/// Presentation only: if an action is displayed as interactive, the
/// application must genuinely support it. No dead keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub label: String,
    pub trigger: KeyTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_default: bool,
}

impl Action {
    pub fn new(key: char, id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            trigger: KeyTrigger::Char(key),
            description: None,
            is_default: false,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn default_action(mut self) -> Self {
        self.is_default = true;
        self
    }

    pub fn open() -> Self {
        Self {
            id: "open".into(),
            label: "Open".into(),
            trigger: KeyTrigger::Enter,
            description: Some("Open or accept current selection".into()),
            is_default: true,
        }
    }

    pub fn back() -> Self {
        Self {
            id: "back".into(),
            label: "Back".into(),
            trigger: KeyTrigger::Esc,
            description: Some("Return to previous screen or cancel".into()),
            is_default: false,
        }
    }

    pub fn cancel() -> Self {
        Self {
            id: "cancel".into(),
            label: "Cancel".into(),
            trigger: KeyTrigger::Esc,
            description: Some("Cancel operation".into()),
            is_default: false,
        }
    }

    pub fn find() -> Self {
        Self {
            id: "find".into(),
            label: "Find".into(),
            trigger: KeyTrigger::Char('/'),
            description: Some("Search or filter items".into()),
            is_default: false,
        }
    }

    pub fn help() -> Self {
        Self {
            id: "help".into(),
            label: "Help".into(),
            trigger: KeyTrigger::Char('?'),
            description: Some("Show contextual help".into()),
            is_default: false,
        }
    }

    pub fn details() -> Self {
        Self {
            id: "details".into(),
            label: "Details".into(),
            trigger: KeyTrigger::Char('d'),
            description: Some("Show detailed evidence and inspection data".into()),
            is_default: false,
        }
    }

    pub fn retry() -> Self {
        Self {
            id: "retry".into(),
            label: "Retry".into(),
            trigger: KeyTrigger::Char('r'),
            description: Some("Retry failed operation or refresh data".into()),
            is_default: false,
        }
    }

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
