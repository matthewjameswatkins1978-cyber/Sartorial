use crate::semantic::status::Status;
use serde::{Deserialize, Serialize};

/// Canonical schema version for the Sartorial agent protocol.
pub const SARTORIAL_SCHEMA_VERSION: &str = "sartorial.v0.1";

/// Standard envelope for agent-facing output ensuring stable, bounded contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentEnvelope<T> {
    pub schema_version: String,
    pub status: Status,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next_actions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    pub data: T,
}

impl<T> AgentEnvelope<T> {
    pub fn new(status: Status, data: T) -> Self {
        Self {
            schema_version: SARTORIAL_SCHEMA_VERSION.to_string(),
            status,
            next_actions: Vec::new(),
            warnings: Vec::new(),
            data,
        }
    }

    pub fn with_next_actions(mut self, actions: Vec<String>) -> Self {
        self.next_actions = actions;
        self
    }

    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }
}

/// Agent JSON formatting utilities ensuring bounded, structured context economy.
pub struct AgentRenderer;

impl AgentRenderer {
    pub fn to_compact_json<T: Serialize>(item: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string(item)
    }

    pub fn to_pretty_json<T: Serialize>(item: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(item)
    }
}
