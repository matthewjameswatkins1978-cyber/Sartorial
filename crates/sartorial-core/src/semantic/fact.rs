use serde::{Deserialize, Serialize};

/// A strongly typed field or factual attribute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fact {
    #[serde(alias = "key")]
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub muted: bool,
}

impl Fact {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            unit: None,
            muted: false,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}
