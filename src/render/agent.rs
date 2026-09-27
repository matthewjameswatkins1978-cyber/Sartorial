use serde::Serialize;
use serde_json::Error;

/// Agent JSON formatting utilities ensuring bounded, structured context economy.
pub struct AgentRenderer;

impl AgentRenderer {
    /// Render any serializable semantic item as compact JSON.
    pub fn to_compact_json<T: Serialize>(item: &T) -> Result<String, Error> {
        serde_json::to_string(item)
    }

    /// Render any serializable semantic item as pretty-printed JSON.
    pub fn to_pretty_json<T: Serialize>(item: &T) -> Result<String, Error> {
        serde_json::to_string_pretty(item)
    }
}
