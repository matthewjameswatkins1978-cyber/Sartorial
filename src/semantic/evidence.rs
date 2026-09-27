use serde::{Deserialize, Serialize};

/// Concrete evidence supporting an outcome or error.
///
/// Designed around context economy: conveys the minimum sufficient evidence
/// upfront, with optional handles and opt-in details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    /// Summary or direct message of the evidence (e.g. "unused import `Path`").
    pub summary: String,
    /// Source location if applicable (e.g. "src/repo.rs:184").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Unique handle or reference identifier for retrieval without dumping raw logs
    /// (e.g. "log:clippy-build-001" or "hash:84f1a").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Optional deep details, kept bounded or retrieved on demand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Whether full evidence was truncated to conserve context.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub truncated: bool,
}

impl Evidence {
    /// Create minimal sufficient evidence with just a summary.
    pub fn new(summary: impl Into<String>) -> Self {
        Self {
            summary: summary.into(),
            location: None,
            handle: None,
            details: None,
            truncated: false,
        }
    }

    /// Attach a source code or file location.
    pub fn at(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Attach a reference handle for out-of-band or progressive disclosure.
    pub fn with_handle(mut self, handle: impl Into<String>) -> Self {
        self.handle = Some(handle.into());
        self
    }

    /// Attach detailed trace/log content.
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Mark the evidence as truncated.
    pub fn truncated(mut self) -> Self {
        self.truncated = true;
        self
    }
}
