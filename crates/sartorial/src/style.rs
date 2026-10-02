//! Backwards-compatible style surface.
//!
//! The canonical definitions live in [`sartorial_core`]:
//! grammar in [`preset`](sartorial_core::preset), identity in
//! [`theme`](sartorial_core::theme), resolution in
//! [`ResolvedStyle`](sartorial_core::ResolvedStyle).
//! This module re-exports them under their historic paths so existing
//! `use sartorial::style::{Preset, ResolvedStyle, ...}` code keeps working.

pub use sartorial_core::preset::{
    BorderStyle, Density, Preset, ProgressTreatment, SectionRule, StatusLayout, TitleCase,
};
pub use sartorial_core::style::ResolvedStyle;
pub use sartorial_core::theme::{Theme, ThemeBuilder};
