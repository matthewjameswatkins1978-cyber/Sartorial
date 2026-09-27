use serde::{Deserialize, Serialize};

/// Target destination and format for Sartorial output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderTarget {
    /// Human terminal view: ANSI styled, terminal-width responsive, unicode icons.
    #[default]
    Human,
    /// Plain text: No ANSI escape codes, pipe-safe, grep-friendly.
    Plain,
    /// AI agent / machine JSON: Strictly structured, bounded, no decorative prose.
    Agent,
}

impl RenderTarget {
    pub fn is_human(&self) -> bool {
        matches!(self, Self::Human)
    }

    pub fn is_plain(&self) -> bool {
        matches!(self, Self::Plain)
    }

    pub fn is_agent(&self) -> bool {
        matches!(self, Self::Agent)
    }
}
