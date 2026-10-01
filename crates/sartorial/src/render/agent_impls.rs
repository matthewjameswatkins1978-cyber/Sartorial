//! `RenderAgent` implementations for core semantic types.
//!
//! Part of the optional `wire` surface: stable, schema-versioned JSON for
//! Sartorial's own presentation model. Application business schemas stay
//! application-owned and never have to become Sartorial schemas.

#![cfg(feature = "wire")]

use crate::render::{RenderAgent, SARTORIAL_SCHEMA_VERSION};
use sartorial_core::{
    Action, ErrorModel, Evidence, Fact, Outcome, Plan, ProgressState, Receipt, Status, TableModel,
    TableRow,
};

#[derive(serde::Serialize)]
struct AgentOutcomeRepresentation<'a> {
    schema_version: &'static str,
    status: Status,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: &'a Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    facts: &'a Vec<Fact>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    evidence: &'a Vec<Evidence>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    warnings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    next_actions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: &'a Option<String>,
}

impl RenderAgent for Outcome {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentOutcomeRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            status: self.status,
            title: &self.title,
            summary: &self.summary,
            facts: &self.facts,
            evidence: &self.evidence,
            warnings: self.warnings.iter().map(|w| w.message.clone()).collect(),
            next_actions: self.actions.iter().map(|a| a.id.clone()).collect(),
            details: &self.details,
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}

#[derive(serde::Serialize)]
struct AgentTableRepresentation<'a> {
    schema_version: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    badge: &'a Option<String>,
    headers: &'a [String],
    rows: &'a [TableRow],
}

impl RenderAgent for TableModel {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentTableRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            title: &self.title,
            badge: &self.badge,
            headers: &self.headers,
            rows: &self.rows,
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}

#[derive(serde::Serialize)]
struct AgentPlanRepresentation<'a> {
    schema_version: &'static str,
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: &'a Option<String>,
    changes: &'a [sartorial_core::PlanChange],
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

#[derive(serde::Serialize)]
struct AgentReceiptRepresentation<'a> {
    schema_version: &'static str,
    title: &'a str,
    status: Status,
    changes: &'a [Fact],
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    unchanged: &'a [Fact],
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance: &'a Option<String>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    warnings: &'a [sartorial_core::Notice],
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence_handle: &'a Option<String>,
    next_actions: Vec<&'a str>,
}

impl RenderAgent for Receipt {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentReceiptRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            title: &self.title,
            status: self.status,
            changes: &self.changes,
            unchanged: &self.unchanged,
            guidance: &self.guidance,
            warnings: &self.warnings,
            evidence_handle: &self.evidence_handle,
            next_actions: self.actions.iter().map(|a| a.id.as_str()).collect(),
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}

#[derive(serde::Serialize)]
struct AgentErrorRepresentation<'a> {
    schema_version: &'static str,
    status: Status,
    what: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    why: &'a Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    evidence: &'a Vec<Evidence>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    next_actions: Vec<String>,
}

impl RenderAgent for ErrorModel {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        let rep = AgentErrorRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            status: Status::Failed,
            what: &self.what,
            why: &self.why,
            evidence: &self.evidence,
            next_actions: self.next_actions.iter().map(|a| a.id.clone()).collect(),
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}

#[derive(serde::Serialize)]
struct AgentProgressRepresentation<'a> {
    schema_version: &'static str,
    task: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtask: &'a Option<String>,
    mode: sartorial_core::ProgressMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    current: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elapsed_secs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    countdown_secs: Option<u64>,
    status: Status,
}

impl RenderAgent for ProgressState {
    fn to_agent_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        self.validate().map_err(serde::ser::Error::custom)?;
        let rep = AgentProgressRepresentation {
            schema_version: SARTORIAL_SCHEMA_VERSION,
            task: &self.task,
            subtask: &self.subtask,
            mode: self.mode,
            current: self.current,
            total: self.total,
            percent: self.derived_percent(),
            unit: &self.unit,
            elapsed_secs: self.elapsed_secs,
            rate: &self.rate,
            countdown_secs: self.countdown_secs,
            status: self.status,
        };
        if pretty {
            serde_json::to_string_pretty(&rep)
        } else {
            serde_json::to_string(&rep)
        }
    }
}

/// Screen-level agent representations (kept beside the core ones so the
/// `wire` surface stays in one reviewable place).
#[derive(serde::Serialize)]
pub(crate) struct AgentSummaryRepresentation<'a> {
    pub schema_version: &'static str,
    pub status: Status,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: &'a Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub facts: &'a Vec<Fact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: &'a Option<TableModel>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub next_actions: Vec<String>,
}

#[derive(serde::Serialize)]
pub(crate) struct AgentListRepresentation<'a> {
    pub schema_version: &'static str,
    pub status: Status,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_badge: &'a Option<String>,
    pub table: &'a TableModel,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub next_actions: Vec<String>,
}

#[derive(serde::Serialize)]
pub(crate) struct AgentDetailRepresentation<'a> {
    pub schema_version: &'static str,
    pub status: Status,
    pub title: &'a str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub facts: &'a Vec<Fact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: &'a Option<Evidence>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub next_actions: Vec<String>,
}

pub(crate) fn next_action_ids(actions: &[Action]) -> Vec<String> {
    actions.iter().map(|a| a.id.clone()).collect()
}
