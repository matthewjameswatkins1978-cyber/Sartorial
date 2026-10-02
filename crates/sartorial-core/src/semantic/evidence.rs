use serde::{Deserialize, Serialize};

/// Concrete evidence supporting an outcome or error.
/// Minimum sufficient evidence upfront; handles for the rest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub truncated: bool,
}

impl Evidence {
    pub fn new(summary: impl Into<String>) -> Self {
        Self {
            summary: summary.into(),
            location: None,
            handle: None,
            details: None,
            truncated: false,
        }
    }

    pub fn at(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn with_handle(mut self, handle: impl Into<String>) -> Self {
        self.handle = Some(handle.into());
        self
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn truncated(mut self) -> Self {
        self.truncated = true;
        self
    }
}
