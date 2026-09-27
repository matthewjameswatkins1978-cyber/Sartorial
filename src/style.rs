use crate::config::{BorderStyle, Density, SymbolMode};
use anstyle::{AnsiColor, Effects, Style};

/// The four first-party visual presets of the Biscuit Logic Presentation Standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Preset {
    /// Biscuit Logic default: quiet, cool, precise, modern, restrained.
    /// Accent: Restrained Cyan/Slate.
    #[default]
    House,
    /// Formal, nearly monochrome, elegant, sparse, controlled.
    /// Accent: Restrained BrightWhite/Monochrome with semantic-only highlights.
    BlackTie,
    /// Dense, fast, practical, information-rich, operator focused.
    /// Accent: Industrial Amber/Yellow, compact spacing, numbers prioritised.
    Workwear,
    /// Slightly more expressive, slightly roomier, creative, presentation-friendly.
    /// Accent: Refined Magenta/Violet, polished whitespace.
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

    /// Default accent color associated with this preset.
    pub fn default_accent(&self) -> AnsiColor {
        match self {
            Self::House => AnsiColor::Cyan,
            Self::BlackTie => AnsiColor::BrightWhite,
            Self::Workwear => AnsiColor::Yellow,
            Self::Studio => AnsiColor::Magenta,
        }
    }

    /// Default layout density for this preset.
    pub fn default_density(&self) -> Density {
        match self {
            Self::House => Density::Standard,
            Self::BlackTie => Density::Standard,
            Self::Workwear => Density::Compact,
            Self::Studio => Density::Roomy,
        }
    }

    /// Spacing between action items in keyboard action bars.
    pub fn action_spacing(&self) -> usize {
        match self {
            Self::House => 3,
            Self::BlackTie => 3,
            Self::Workwear => 2,
            Self::Studio => 4,
        }
    }

    /// Horizontal rule character.
    pub fn rule_char(&self, symbols: SymbolMode) -> char {
        match symbols {
            SymbolMode::Ascii => '-',
            _ => match self {
                Self::Workwear => '─',
                Self::BlackTie => '─',
                Self::Studio => '─',
                Self::House => '─',
            },
        }
    }

    // ---- Resolved layout grammar (single authority; see ResolvedStyle) ----
    // These are `pub(crate)`: applications consume the resolved decisions,
    // never the per-preset mapping.

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

    /// Blank lines between major presentation components.
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

    /// Extra padding after the longest fact label (before the value column).
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

/// Character/motion treatment for progress indicators under this style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressTreatment {
    /// Restrained pulse/spinner and clean bar (House).
    Restrained,
    /// Almost static, tiny pulse or dot, numeric emphasis (Black Tie).
    Minimal,
    /// Compact numeric tags, bracketed counts and rates (Workwear).
    Numeric,
    /// Smooth expressive glyphs and polished bar (Studio).
    Expressive,
}

/// Title text casing decided by the preset grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitleCase {
    /// Uppercase display titles (House, Workwear).
    Upper,
    /// Preserve the application-supplied casing (Black Tie, Studio).
    Preserve,
}

/// Status/badge placement relative to its section heading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusLayout {
    /// Heading and badge share one line with a bounded gap (House, Workwear).
    Inline,
    /// Heading, short rule, then badge on its own line (Black Tie, Studio).
    Stacked,
}

/// Rule treatment under section headings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionRule {
    /// No rule; whitespace and typography carry the grouping.
    None,
    /// Short rule matching the heading width (Black Tie, Studio).
    Short,
}

/// Resolved presentation authority. Components consume this rather than querying presets.
///
/// The layout grammar fields below (`title_case`, `status_layout`, ...) are
/// the single authority for preset personality: renderers read explicit
/// decisions here and never branch on `Preset` themselves.
#[derive(Debug, Clone)]
pub struct ResolvedStyle {
    pub preset: Preset,
    pub accent: AnsiColor,
    pub density: Density,
    pub border: BorderStyle,
    pub symbols: SymbolMode,
    pub action_spacing: usize,
    pub rule_char: char,
    pub progress_treatment: ProgressTreatment,
    pub color_enabled: bool,
    /// Title casing grammar.
    pub title_case: TitleCase,
    /// Structural marker before titles (`» ` for Workwear, else empty).
    pub title_marker: &'static str,
    /// Whether a restrained rule follows the title/subtitle block.
    pub title_block_rule: bool,
    /// Status/badge placement grammar.
    pub status_layout: StatusLayout,
    /// Fixed gap between an inline section label and its badge.
    pub status_gap: usize,
    /// Structural marker before section headings (`» ` for Workwear, else empty).
    pub section_marker: &'static str,
    /// Rule treatment under section headings.
    pub section_rule: SectionRule,
    /// Blank lines between major presentation components.
    pub component_gap: usize,
    /// Whether fact labels render uppercase (Workwear).
    pub fact_uppercase: bool,
    /// Whether fact labels carry a trailing colon (Workwear).
    pub fact_colon: bool,
    /// Compact one-line notice treatment (Workwear).
    pub notice_compact: bool,
    /// Whether tables render their header row with a separating rule.
    pub table_headers: bool,
}

impl ResolvedStyle {
    pub fn title_style(&self) -> Style {
        if !self.color_enabled {
            return Style::new().effects(Effects::BOLD);
        }
        match self.preset {
            Preset::Studio => Style::new()
                .fg_color(Some(self.accent.into()))
                .effects(Effects::BOLD | Effects::UNDERLINE),
            _ => Style::new()
                .fg_color(Some(self.accent.into()))
                .effects(Effects::BOLD),
        }
    }

    pub fn section_style(&self) -> Style {
        if !self.color_enabled {
            return Style::new().effects(Effects::BOLD);
        }
        match self.preset {
            Preset::Workwear => Style::new()
                .fg_color(Some(self.accent.into()))
                .effects(Effects::BOLD),
            Preset::Studio => Style::new()
                .fg_color(Some(self.accent.into()))
                .effects(Effects::BOLD),
            _ => Style::new().effects(Effects::BOLD),
        }
    }

    pub fn label_style(&self) -> Style {
        if !self.color_enabled {
            return Style::new();
        }
        match self.preset {
            Preset::BlackTie => Style::new().fg_color(Some(AnsiColor::White.into())),
            _ => Style::new().fg_color(Some(AnsiColor::BrightBlack.into())),
        }
    }

    pub fn value_style(&self) -> Style {
        Style::new()
    }

    pub fn key_bracket_style(&self) -> Style {
        if !self.color_enabled {
            return Style::new();
        }
        match self.preset {
            Preset::BlackTie => Style::new().fg_color(Some(AnsiColor::White.into())),
            _ => Style::new().fg_color(Some(AnsiColor::BrightBlack.into())),
        }
    }

    pub fn key_char_style(&self) -> Style {
        if !self.color_enabled {
            return Style::new().effects(Effects::BOLD);
        }
        Style::new()
            .fg_color(Some(self.accent.into()))
            .effects(Effects::BOLD)
    }

    pub fn muted_style(&self) -> Style {
        if !self.color_enabled {
            return Style::new();
        }
        Style::new().fg_color(Some(AnsiColor::BrightBlack.into()))
    }

    pub fn rule_style(&self) -> Style {
        if !self.color_enabled {
            return Style::new();
        }
        match self.preset {
            Preset::Workwear => Style::new().fg_color(Some(self.accent.into())),
            _ => Style::new().fg_color(Some(AnsiColor::BrightBlack.into())),
        }
    }

    /// Spacing between action items in keyboard action bars.
    pub fn action_gap(&self, is_narrow: bool) -> usize {
        if is_narrow {
            2.min(self.action_spacing)
        } else {
            self.action_spacing
        }
    }

    /// Spacing between table columns.
    pub fn table_col_gap(&self, is_narrow: bool) -> usize {
        if is_narrow {
            match self.density {
                Density::Compact => 1,
                Density::Standard => 2,
                Density::Roomy => 3,
            }
        } else {
            match self.density {
                Density::Compact => 2,
                Density::Standard => 4,
                Density::Roomy => 5,
            }
        }
    }

    /// Horizontal padding for key-value fact labels.
    pub fn key_value_padding(&self) -> usize {
        self.preset.fact_pad()
    }

    /// Fact-label padding with narrow-terminal degradation.
    pub fn fact_pad(&self, is_narrow: bool) -> usize {
        if is_narrow {
            1
        } else {
            self.preset.fact_pad()
        }
    }

    /// Blank lines between major components, surrendering room before content.
    pub fn component_gap(&self, is_narrow: bool) -> usize {
        if is_narrow {
            self.component_gap.min(1)
        } else {
            self.component_gap
        }
    }

    /// Title text with preset casing and structural marker applied.
    pub fn format_title(&self, name: &str) -> String {
        let cased = match self.title_case {
            TitleCase::Upper => name.to_uppercase(),
            TitleCase::Preserve => name.to_string(),
        };
        format!("{}{}", self.title_marker, cased)
    }

    /// Fact label with preset casing and separator applied.
    /// Presentation only: semantic labels are never renamed.
    pub fn format_fact_label(&self, name: &str) -> String {
        let mut label = if self.fact_uppercase {
            name.to_uppercase()
        } else {
            name.to_string()
        };
        if self.fact_colon {
            label.push(':');
        }
        label
    }

    /// Section header formatting helper.
    pub fn format_section_header(&self, title: &str) -> String {
        let cased = if self.fact_uppercase {
            // Operator voice (Workwear): structural marker plus uppercase.
            title.to_uppercase()
        } else {
            title.to_string()
        };
        format!("{}{}", self.section_marker, cased)
    }

    /// Title-block rule length: restrained, never exceeding terminal width.
    pub fn title_rule_len(&self, width: usize) -> usize {
        width.min(60)
    }

    /// Action key bracket pair: (open, close).
    pub fn key_delimiters(&self) -> (&'static str, &'static str) {
        match self.preset {
            Preset::BlackTie => ("(", ")"),
            _ => ("[", "]"),
        }
    }
}
