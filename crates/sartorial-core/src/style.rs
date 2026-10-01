use crate::capabilities::SymbolMode;
use crate::preset::{
    BorderStyle, Density, Preset, ProgressTreatment, SectionRule, StatusLayout, TitleCase,
};
use crate::semantic::{ChangeKind, NoticeLevel, Status};
use crate::theme::Theme;
use anstyle::{AnsiColor, Effects, Style};

/// Fully resolved presentation authority.
///
/// Built once from [`Preset`] (grammar) + [`Theme`] (identity) +
/// explicit layout inputs. Renderers consume these concrete decisions
/// and never branch on preset or theme identity themselves.
#[derive(Debug, Clone)]
pub struct ResolvedStyle {
    /// Grammar identity, informational only. Renderers must not branch on this.
    pub preset: Preset,
    /// Theme name, informational only. Renderers must not branch on this.
    pub theme_name: String,
    /// Resolved accent, informational. Renderers use the concrete styles below.
    pub accent: AnsiColor,
    pub density: Density,
    pub border: BorderStyle,
    pub symbols: SymbolMode,
    pub color_enabled: bool,
    pub action_spacing: usize,
    pub rule_char: char,
    pub progress_treatment: ProgressTreatment,
    pub title_case: TitleCase,
    pub title_marker: &'static str,
    pub title_block_rule: bool,
    pub status_layout: StatusLayout,
    pub status_gap: usize,
    pub section_marker: &'static str,
    pub section_rule: SectionRule,
    pub component_gap_lines: usize,
    pub fact_uppercase: bool,
    pub fact_colon: bool,
    pub notice_compact: bool,
    pub table_headers: bool,
    pub key_open: &'static str,
    pub key_close: &'static str,
    theme: Theme,
    title_style: Style,
    section_style: Style,
    label_style: Style,
    value_style: Style,
    key_bracket_style: Style,
    key_char_style: Style,
    muted_style: Style,
    rule_style: Style,
}

impl ResolvedStyle {
    /// Single authority turning grammar + identity into concrete decisions.
    /// `symbols` must already be resolved (no `Auto`); `color_enabled` must
    /// already fold colour policy, `NO_COLOR`, and accessibility.
    pub fn resolve(
        preset: Preset,
        theme: &Theme,
        density: Density,
        border: BorderStyle,
        symbols: SymbolMode,
        color_enabled: bool,
    ) -> Self {
        let unicode = symbols != SymbolMode::Ascii;
        let (title_marker, section_marker) = if unicode {
            (preset.structural_marker(), preset.structural_marker())
        } else {
            (
                preset.ascii_structural_marker(),
                preset.ascii_structural_marker(),
            )
        };
        let rule_char = if unicode { '─' } else { '-' };
        let (key_open, key_close) = match preset {
            Preset::BlackTie => ("(", ")"),
            _ => ("[", "]"),
        };

        let bold = || Style::new().effects(Effects::BOLD);
        let fg = |c: AnsiColor| Style::new().fg_color(Some(c.into()));
        let fg_bold = |c: AnsiColor| Style::new().fg_color(Some(c.into())).effects(Effects::BOLD);
        let fg_bold_underline = |c: AnsiColor| {
            Style::new()
                .fg_color(Some(c.into()))
                .effects(Effects::BOLD | Effects::UNDERLINE)
        };

        // Title: accent-forward; underline only when the theme asks (Studio).
        let title_style = if !color_enabled {
            bold()
        } else if theme.underline_titles {
            fg_bold_underline(theme.accent)
        } else {
            fg_bold(theme.accent)
        };
        // Section: accent only for grammars that want expressive sections
        // (Workwear, Studio) and only when the theme is not monochrome.
        let expressive_section =
            matches!(preset, Preset::Workwear | Preset::Studio) && !theme.monochrome;
        let section_style = if !color_enabled {
            bold()
        } else if expressive_section {
            fg_bold(theme.heading)
        } else {
            bold()
        };
        // Fact labels: Black Tie grammar keeps a formal white voice.
        let formal_labels = matches!(preset, Preset::BlackTie) && !theme.monochrome;
        let label_style = if !color_enabled {
            Style::new()
        } else if formal_labels {
            fg(AnsiColor::White)
        } else {
            fg(theme.muted)
        };
        let value_style = Style::new();
        let key_bracket_style = if !color_enabled {
            Style::new()
        } else if formal_labels {
            fg(AnsiColor::White)
        } else {
            fg(theme.muted)
        };
        let key_char_style = if !color_enabled {
            bold()
        } else {
            fg_bold(theme.accent)
        };
        let muted_style = if !color_enabled {
            Style::new()
        } else {
            fg(theme.muted)
        };
        // Rules: Workwear grammar tints rules with the accent; others stay muted.
        let tinted_rules = matches!(preset, Preset::Workwear) && !theme.monochrome;
        let rule_style = if !color_enabled {
            Style::new()
        } else if tinted_rules {
            fg(theme.accent)
        } else {
            fg(theme.muted)
        };

        Self {
            preset,
            theme_name: theme.name.clone(),
            accent: theme.accent,
            density,
            border,
            symbols,
            color_enabled,
            action_spacing: preset.action_spacing(),
            rule_char,
            progress_treatment: preset.progress_treatment(),
            title_case: preset.title_case(),
            title_marker,
            title_block_rule: preset.title_block_rule(),
            status_layout: preset.status_layout(),
            status_gap: preset.status_gap(),
            section_marker,
            section_rule: preset.section_rule(),
            component_gap_lines: preset.component_gap(),
            fact_uppercase: preset.fact_uppercase(),
            fact_colon: preset.fact_colon(),
            notice_compact: preset.notice_compact(),
            table_headers: preset.table_headers(),
            key_open,
            key_close,
            theme: theme.clone(),
            title_style,
            section_style,
            label_style,
            value_style,
            key_bracket_style,
            key_char_style,
            muted_style,
            rule_style,
        }
    }

    /// Force pipe-safe invariants: ASCII symbols, no colour, ASCII grammar.
    /// Used by plain/agent targets so builder-call order cannot matter.
    pub fn force_plain(&mut self) {
        self.color_enabled = false;
        self.symbols = SymbolMode::Ascii;
        self.title_marker = self.preset.ascii_structural_marker();
        self.section_marker = self.preset.ascii_structural_marker();
        self.rule_char = '-';
    }

    /// Test/shim helper: default theme for the preset, explicit capabilities.
    pub fn test_resolve(preset: Preset, symbols: SymbolMode, color_enabled: bool) -> Self {
        let theme = Theme::preset_default(preset);
        Self::resolve(
            preset,
            &theme,
            preset.default_density(),
            BorderStyle::Subtle,
            symbols,
            color_enabled,
        )
    }

    pub fn title_style(&self) -> Style {
        self.title_style
    }
    pub fn section_style(&self) -> Style {
        self.section_style
    }
    pub fn label_style(&self) -> Style {
        self.label_style
    }
    pub fn value_style(&self) -> Style {
        self.value_style
    }
    pub fn key_bracket_style(&self) -> Style {
        self.key_bracket_style
    }
    pub fn key_char_style(&self) -> Style {
        self.key_char_style
    }
    pub fn muted_style(&self) -> Style {
        self.muted_style
    }
    pub fn rule_style(&self) -> Style {
        self.rule_style
    }

    /// Semantic status paint. Branching on `Status` is required: the status
    /// already carries the meaning, this only maps it through the theme.
    pub fn status_style(&self, status: Status) -> Style {
        if !self.color_enabled {
            return match status {
                Status::Ready | Status::Attention | Status::Failed | Status::Running => {
                    Style::new().effects(Effects::BOLD)
                }
                Status::Pending | Status::Skipped => Style::new(),
            };
        }
        match status {
            Status::Ready => Style::new()
                .fg_color(Some(self.theme.success.into()))
                .effects(Effects::BOLD),
            Status::Attention => Style::new()
                .fg_color(Some(self.theme.warning.into()))
                .effects(Effects::BOLD),
            Status::Failed => Style::new()
                .fg_color(Some(self.theme.failure.into()))
                .effects(Effects::BOLD),
            Status::Running => Style::new()
                .fg_color(Some(self.theme.accent.into()))
                .effects(Effects::BOLD),
            Status::Pending | Status::Skipped => {
                Style::new().fg_color(Some(self.theme.muted.into()))
            }
        }
    }

    /// Semantic notice-level paint, mapped through the theme.
    pub fn notice_style(&self, level: NoticeLevel) -> Style {
        if !self.color_enabled {
            return match level {
                NoticeLevel::Warning | NoticeLevel::Error => Style::new().effects(Effects::BOLD),
                _ => Style::new(),
            };
        }
        match level {
            NoticeLevel::Info | NoticeLevel::Tip => {
                Style::new().fg_color(Some(self.theme.accent.into()))
            }
            NoticeLevel::Warning => Style::new()
                .fg_color(Some(self.theme.warning.into()))
                .effects(Effects::BOLD),
            NoticeLevel::Error => Style::new()
                .fg_color(Some(self.theme.failure.into()))
                .effects(Effects::BOLD),
        }
    }

    /// Plan change-kind paint, mapped through the theme's success/failure/warning.
    pub fn change_style(&self, kind: ChangeKind) -> Style {
        if !self.color_enabled {
            return Style::new().effects(Effects::BOLD);
        }
        match kind {
            ChangeKind::Add => Style::new()
                .fg_color(Some(self.theme.success.into()))
                .effects(Effects::BOLD),
            ChangeKind::Remove => Style::new()
                .fg_color(Some(self.theme.failure.into()))
                .effects(Effects::BOLD),
            ChangeKind::Modify => Style::new()
                .fg_color(Some(self.theme.warning.into()))
                .effects(Effects::BOLD),
        }
    }

    pub fn action_gap(&self, is_narrow: bool) -> usize {
        if is_narrow {
            2.min(self.action_spacing)
        } else {
            self.action_spacing
        }
    }

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

    pub fn key_value_padding(&self) -> usize {
        self.preset.fact_pad()
    }

    pub fn fact_pad(&self, is_narrow: bool) -> usize {
        if is_narrow {
            1
        } else {
            self.preset.fact_pad()
        }
    }

    pub fn component_gap(&self, is_narrow: bool) -> usize {
        if is_narrow {
            self.component_gap_lines.min(1)
        } else {
            self.component_gap_lines
        }
    }

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

    pub fn format_section_header(&self, title: &str) -> String {
        let cased = if self.fact_uppercase {
            title.to_uppercase()
        } else {
            title.to_string()
        };
        format!("{}{}", self.section_marker, cased)
    }

    pub fn title_rule_len(&self, width: usize) -> usize {
        width.min(60)
    }

    pub fn key_delimiters(&self) -> (&'static str, &'static str) {
        (self.key_open, self.key_close)
    }
}
