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

/// Resolved presentation authority. Components consume this rather than querying presets.
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
        match self.density {
            Density::Compact => 1,
            Density::Standard => 2,
            Density::Roomy => 4,
        }
    }

    /// Section header formatting helper.
    pub fn format_section_header(&self, title: &str) -> String {
        match self.preset {
            Preset::Workwear => format!("» {title}"),
            _ => title.to_string(),
        }
    }

    /// Action key bracket pair: (open, close).
    pub fn key_delimiters(&self) -> (&'static str, &'static str) {
        match self.preset {
            Preset::BlackTie => ("(", ")"),
            _ => ("[", "]"),
        }
    }
}
