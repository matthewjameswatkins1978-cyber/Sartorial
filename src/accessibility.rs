use std::env;

/// Accessibility policy across all Sartorial presentations.
/// Accessibility is an override layer across all presets, never a theme of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccessibilityMode {
    /// Detect environment hints (`REDUCED_MOTION`, `ACCESSIBILITY`, `NO_COLOR`).
    #[default]
    Auto,
    /// Standard full visual presentation.
    Standard,
    /// Disable animations and spinners; preserve static counts, percentages, and elapsed time.
    ReducedMotion,
    /// Screen-reader and high-compatibility mode (ASCII glyphs, plain layout, no motion).
    Plain,
}

impl AccessibilityMode {
    /// Check whether reduced motion is requested.
    pub fn is_reduced_motion(&self) -> bool {
        match self {
            Self::ReducedMotion | Self::Plain => true,
            Self::Standard => false,
            Self::Auto => {
                env::var_os("REDUCED_MOTION").is_some_and(|v| !v.is_empty())
                    || env::var_os("PREFERS_REDUCED_MOTION").is_some_and(|v| !v.is_empty())
            }
        }
    }

    /// Check whether plain high-compatibility mode is requested.
    pub fn is_plain(&self) -> bool {
        match self {
            Self::Plain => true,
            Self::Standard | Self::ReducedMotion => false,
            Self::Auto => env::var_os("ACCESSIBILITY").is_some_and(|v| v == "1" || v == "plain"),
        }
    }
}
