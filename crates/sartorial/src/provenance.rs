use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::table::TableModel;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// Origin source of a configuration parameter or operational value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceSource {
    /// Provided via command-line argument (e.g. `--endpoint`).
    Flag(String),
    /// Loaded from an environment variable (e.g. `SARTORIAL_COLOR`).
    Environment(String),
    /// Loaded from a repository or workspace project configuration file.
    ProjectConfig(String),
    /// Loaded from a user global configuration file.
    UserConfig(String),
    /// Standard built-in default.
    Default,
    /// Other designated origin source.
    Other(String),
}

impl ProvenanceSource {
    pub fn display_label(&self) -> String {
        match self {
            Self::Flag(f) => f.clone(),
            Self::Environment(e) => e.clone(),
            Self::ProjectConfig(p) => p.clone(),
            Self::UserConfig(u) => u.clone(),
            Self::Default => "default".to_string(),
            Self::Other(o) => o.clone(),
        }
    }
}

/// A fact or configuration setting accompanied by its origin provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceFact {
    pub key: String,
    pub value: String,
    pub source: ProvenanceSource,
}

impl ProvenanceFact {
    pub fn new(key: impl Into<String>, value: impl Into<String>, source: ProvenanceSource) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            source,
        }
    }

    pub fn from_flag(
        key: impl Into<String>,
        value: impl Into<String>,
        flag: impl Into<String>,
    ) -> Self {
        Self::new(key, value, ProvenanceSource::Flag(flag.into()))
    }

    pub fn from_env(
        key: impl Into<String>,
        value: impl Into<String>,
        var: impl Into<String>,
    ) -> Self {
        Self::new(key, value, ProvenanceSource::Environment(var.into()))
    }

    pub fn from_default(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(key, value, ProvenanceSource::Default)
    }

    pub fn from_project(
        key: impl Into<String>,
        value: impl Into<String>,
        file: impl Into<String>,
    ) -> Self {
        Self::new(key, value, ProvenanceSource::ProjectConfig(file.into()))
    }
}

/// Component displaying a list of values with their origin provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceList {
    pub title: Option<String>,
    pub facts: Vec<ProvenanceFact>,
}

impl ProvenanceList {
    pub fn new() -> Self {
        Self {
            title: None,
            facts: Vec::new(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, fact: ProvenanceFact) -> Self {
        self.facts.push(fact);
        self
    }

    pub fn to_table_model(&self) -> TableModel {
        let mut model = TableModel::new(vec!["Key", "Value", "Source"]);
        if let Some(ref t) = self.title {
            model = model.with_title(t);
        }
        for f in &self.facts {
            model.add_row([&f.key, &f.value, &f.source.display_label()]);
        }
        model
    }
}

impl Default for ProvenanceList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderHuman for ProvenanceList {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let table = crate::components::table::TableView::new(self.to_table_model());
        table.render_human(ctx, out)
    }
}

impl RenderPlain for ProvenanceList {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let table = crate::components::table::TableView::new(self.to_table_model());
        table.render_plain(ctx, out)
    }
}
