use serde::{Deserialize, Serialize};

/// Spacing and visual density. Behaviour, not colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Density {
    Compact,
    #[default]
    Standard,
    Roomy,
}

/// Border styling for separators and table headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BorderStyle {
    None,
    #[default]
    Subtle,
}

/// The four first-party presentation grammars.
/// A Preset describes layout behaviour only: density, casing, markers,
/// rules, gaps, table and progress treatment. It never picks colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Preset {
    #[default]
    House,
    BlackTie,
    Workwear,
    Studio,
}

impl Preset {
    pub fn name(&self) -> &'static str {
        match self {
            Self::House => "House",
            Self::BlackTie => "Black Tie",
            Self::Workwear => "Workwear",
            Self::Studio => "Studio",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::House => "Biscuit Logic default. Quiet, cool, precise, restrained.",
            Self::BlackTie => "Formal, nearly monochrome, elegant, controlled.",
            Self::Workwear => "Dense, fast, practical, operator-focused.",
            Self::Studio => "Slightly more expressive, roomier, presentation-friendly.",
        }
    }

    pub fn default_density(&self) -> Density {
        match self {
            Self::House => Density::Standard,
            Self::BlackTie => Density::Standard,
            Self::Workwear => Density::Compact,
            Self::Studio => Density::Roomy,
        }
    }

    pub fn action_spacing(&self) -> usize {
        match self {
            Self::House => 3,
            Self::BlackTie => 3,
            Self::Workwear => 2,
            Self::Studio => 4,
        }
    }

    pub(crate) fn title_case(self) -> TitleCase {
        match self {
            Self::House | Self::Workwear => TitleCase::Upper,
            Self::BlackTie | Self::Studio => TitleCase::Preserve,
        }
    }

    pub(crate) fn structural_marker(self) -> &'static str {
        match self {
            Self::Workwear => "» ",
            _ => "",
        }
    }

    pub(crate) fn ascii_structural_marker(self) -> &'static str {
        match self {
            Self::Workwear => "> ",
            _ => "",
        }
    }

    pub(crate) fn title_block_rule(self) -> bool {
        matches!(self, Self::BlackTie)
    }

    pub(crate) fn status_layout(self) -> StatusLayout {
        match self {
            Self::House | Self::Workwear => StatusLayout::Inline,
            Self::BlackTie | Self::Studio => StatusLayout::Stacked,
        }
    }

    pub(crate) fn status_gap(self) -> usize {
        match self {
            Self::Workwear => 2,
            _ => 8,
        }
    }

    pub(crate) fn section_rule(self) -> SectionRule {
        match self {
            Self::BlackTie | Self::Studio => SectionRule::Short,
            _ => SectionRule::None,
        }
    }

    pub(crate) fn component_gap(self) -> usize {
        match self {
            Self::House | Self::BlackTie => 1,
            Self::Workwear => 0,
            Self::Studio => 2,
        }
    }

    pub(crate) fn fact_uppercase(self) -> bool {
        matches!(self, Self::Workwear)
    }

    pub(crate) fn fact_colon(self) -> bool {
        matches!(self, Self::Workwear)
    }

    pub(crate) fn fact_pad(self) -> usize {
        match self {
            Self::House => 2,
            Self::BlackTie => 3,
            Self::Workwear => 1,
            Self::Studio => 6,
        }
    }

    pub(crate) fn notice_compact(self) -> bool {
        matches!(self, Self::Workwear)
    }

    pub(crate) fn table_headers(self) -> bool {
        matches!(self, Self::BlackTie)
    }

    pub(crate) fn progress_treatment(self) -> ProgressTreatment {
        match self {
            Self::House => ProgressTreatment::Restrained,
            Self::BlackTie => ProgressTreatment::Minimal,
            Self::Workwear => ProgressTreatment::Numeric,
            Self::Studio => ProgressTreatment::Expressive,
        }
    }
}

/// Character/motion treatment for progress indicators under this grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressTreatment {
    Restrained,
    Minimal,
    Numeric,
    Expressive,
}

/// Title text casing decided by the preset grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleCase {
    Upper,
    Preserve,
}

/// Status/badge placement relative to its section heading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusLayout {
    Inline,
    Stacked,
}

/// Rule treatment under section headings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SectionRule {
    None,
    Short,
}
