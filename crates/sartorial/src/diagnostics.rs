//! Optional structured-diagnostics seam (feature `diagnostics`).
//!
//! Sartorial is not a compiler-renderer project and will not grow a
//! home-grown rustc-style diagnostic framework. This module offers a tiny
//! application-owned diagnostic shape — file, span, label, code, help,
//! related evidence — that converts into a presentation [`Document`].
//!
//! Products that need rich squiggles can adapt this shape into
//! [`miette`](https://docs.rs/miette)-style reports on their own side;
//! the conversion contract is deliberately one way (diagnostic → document)
//! so Sartorial never owns application schemas.

#![cfg(feature = "diagnostics")]

use sartorial_core::{Document, Evidence, Presentable};

/// A lightweight source diagnostic: where, what, and what to do next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// File path being diagnosed.
    pub file: String,
    /// 1-based line, if known.
    pub line: Option<usize>,
    /// 1-based column, if known.
    pub column: Option<usize>,
    /// Short label for the problem span.
    pub label: String,
    /// Machine diagnostic code (e.g. `TB001`, `E0308`).
    pub code: Option<String>,
    /// Help text suggesting a fix.
    pub help: Option<String>,
    /// Related evidence (spans, logs, references).
    pub evidence: Vec<Evidence>,
}

impl Diagnostic {
    pub fn new(file: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            line: None,
            column: None,
            label: label.into(),
            code: None,
            help: None,
            evidence: Vec::new(),
        }
    }

    pub fn at_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub fn at_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn with_evidence(mut self, evidence: Evidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    fn location_string(&self) -> String {
        match (self.line, self.column) {
            (Some(l), Some(c)) => format!("{}:{l}:{c}", self.file),
            (Some(l), None) => format!("{}:{l}", self.file),
            _ => self.file.clone(),
        }
    }
}

impl Presentable for Diagnostic {
    fn to_document(&self) -> Document {
        let mut error = sartorial_core::ErrorModel::new(&self.label);
        let mut why_parts = vec![format!("at {}", self.location_string())];
        if let Some(code) = &self.code {
            why_parts.push(format!("code {code}"));
        }
        error = error.with_why(why_parts.join(" · "));
        for ev in &self.evidence {
            error = error.with_evidence(ev.clone());
        }
        if let Some(help) = &self.help {
            error = error.with_action(
                sartorial_core::Action::new('h', "help", "Help").with_description(help.clone()),
            );
        }
        error.to_document()
    }
}
