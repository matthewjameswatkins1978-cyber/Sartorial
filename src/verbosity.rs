use serde::{Deserialize, Serialize};

/// Standard presentation verbosity policy for Biscuit Logic CLI applications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verbosity {
    /// Essential output and critical errors only. Suppresses secondary notes, hints, and elapsed metrics.
    Quiet,
    /// Default balanced and concise presentation.
    #[default]
    Normal,
    /// Informative details, secondary timings, provenance notes.
    Verbose,
    /// Exhaustive internal diagnostic traces and debugging payloads.
    Debug,
}

impl Verbosity {
    pub fn is_quiet(&self) -> bool {
        matches!(self, Self::Quiet)
    }

    pub fn is_verbose_or_higher(&self) -> bool {
        *self >= Self::Verbose
    }

    pub fn is_debug(&self) -> bool {
        matches!(self, Self::Debug)
    }
}
