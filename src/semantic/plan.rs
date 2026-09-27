use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};
use crate::semantic::action::Action;
use serde::{Deserialize, Serialize};

/// Type of change proposed in a dry-run Plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Add,
    Remove,
    Modify,
}

impl ChangeKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Remove => "-",
            Self::Modify => "~",
        }
    }

    pub fn color(&self) -> anstyle::AnsiColor {
        match self {
            Self::Add => anstyle::AnsiColor::Green,
            Self::Remove => anstyle::AnsiColor::Red,
            Self::Modify => anstyle::AnsiColor::Yellow,
        }
    }
}

/// A single proposed change item within a Plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanChange {
    pub kind: ChangeKind,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl PlanChange {
    pub fn add(target: impl Into<String>) -> Self {
        Self {
            kind: ChangeKind::Add,
            target: target.into(),
            detail: None,
        }
    }

    pub fn remove(target: impl Into<String>) -> Self {
        Self {
            kind: ChangeKind::Remove,
            target: target.into(),
            detail: None,
        }
    }

    pub fn modify(target: impl Into<String>) -> Self {
        Self {
            kind: ChangeKind::Modify,
            target: target.into(),
            detail: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

/// Semantic model for dry-run / consequential operations BEFORE they happen.
/// Sartorial presents application-owned intent; it never executes the plan itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub changes: Vec<PlanChange>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub consequences: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversible: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub actions: Vec<Action>,
}

impl Plan {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            changes: Vec::new(),
            consequences: Vec::new(),
            warnings: Vec::new(),
            reversible: None,
            actions: Vec::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn add_change(mut self, change: PlanChange) -> Self {
        self.changes.push(change);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(self, target: impl Into<String>) -> Self {
        self.add_change(PlanChange::add(target))
    }

    pub fn remove(self, target: impl Into<String>) -> Self {
        self.add_change(PlanChange::remove(target))
    }

    pub fn modify(self, target: impl Into<String>) -> Self {
        self.add_change(PlanChange::modify(target))
    }

    pub fn consequence(mut self, text: impl Into<String>) -> Self {
        self.consequences.push(text.into());
        self
    }

    pub fn warning(mut self, text: impl Into<String>) -> Self {
        self.warnings.push(text.into());
        self
    }

    pub fn reversible(mut self, reversible: bool) -> Self {
        self.reversible = Some(reversible);
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
}

#[derive(Serialize)]
struct AgentPlanRepresentation<'a> {
    schema_version: &'static str,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: &'a Option<String>,
    changes: &'a [PlanChange],
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    consequences: &'a [String],
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    warnings: &'a [String],
    #[serde(skip_serializing_if = "Option::is_none")]
    reversible: Option<bool>,
    next_actions: Vec<&'a str>,
}

impl RenderAgent for Plan {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentPlanRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            title: &self.title,
            description: &self.description,
            changes: &self.changes,
            consequences: &self.consequences,
            warnings: &self.warnings,
            reversible: self.reversible,
            next_actions: self.actions.iter().map(|a| a.id.as_str()).collect(),
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}
