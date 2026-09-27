use serde::{Deserialize, Serialize};

/// A strongly typed field or factual attribute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fact {
    /// The name/label of the fact (e.g. "Environment", "Version").
    #[serde(alias = "key")]
    pub name: String,
    /// The textual value of the fact (e.g. "x86_64", "2.51.0").
    pub value: String,
    /// Optional unit or secondary annotation (e.g. "ms", "GB", "latest").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Whether the value should be presented as muted/subdued.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub muted: bool,
}

impl Fact {
    /// Create a new standard fact.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            unit: None,
            muted: false,
        }
    }

    /// Add a unit or note to the fact.
    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    /// Mark the fact as muted.
    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}
