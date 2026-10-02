use serde::{Deserialize, Serialize};

/// Output destination and format for Sartorial rendering.
/// Terminal and Plain are static presentation; Markdown is first-class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderTarget {
    #[default]
    Human,
    Plain,
    Markdown,
    /// Structured agent/machine output (JSON lives in the full crate's
    /// optional `wire` surface; core never invents application schemas).
    Agent,
}

impl RenderTarget {
    pub fn is_human(&self) -> bool {
        matches!(self, Self::Human)
    }

    pub fn is_plain(&self) -> bool {
        matches!(self, Self::Plain)
    }

    pub fn is_markdown(&self) -> bool {
        matches!(self, Self::Markdown)
    }

    pub fn is_agent(&self) -> bool {
        matches!(self, Self::Agent)
    }
}
