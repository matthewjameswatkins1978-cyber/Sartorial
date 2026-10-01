use anstyle::AnsiColor;

/// Semantic colour roles. A Theme maps meaning to paint; renderers
/// never branch on product identity, only on these resolved colours.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    /// Theme display name (e.g. "House", "Terrorbats").
    pub name: String,
    /// Primary brand accent: titles, key chars, rules (Workwear).
    pub accent: AnsiColor,
    /// Section headings accent where the grammar wants one.
    pub heading: AnsiColor,
    pub success: AnsiColor,
    pub warning: AnsiColor,
    pub failure: AnsiColor,
    pub muted: AnsiColor,
    pub evidence: AnsiColor,
    pub code: AnsiColor,
    pub path: AnsiColor,
    pub number: AnsiColor,
    /// Monochrome-first themes (Black Tie) suppress accent usage
    /// outside strictly semantic roles.
    pub monochrome: bool,
    /// Studio-style underlined titles.
    pub underline_titles: bool,
}

impl Theme {
    pub fn builder(name: impl Into<String>) -> ThemeBuilder {
        ThemeBuilder::new(name)
    }

    /// Familiar default visuals for a preset, preserving v0.2 personalities
    /// when no explicit theme is supplied.
    pub fn preset_default(preset: crate::Preset) -> Self {
        match preset {
            crate::Preset::House => Self::builder("House")
                .accent(AnsiColor::Cyan)
                .heading(AnsiColor::BrightWhite)
                .build(),
            crate::Preset::BlackTie => Self::builder("Black Tie")
                .accent(AnsiColor::BrightWhite)
                .heading(AnsiColor::White)
                .monochrome(true)
                .build(),
            crate::Preset::Workwear => Self::builder("Workwear")
                .accent(AnsiColor::Yellow)
                .heading(AnsiColor::Yellow)
                .build(),
            crate::Preset::Studio => Self::builder("Studio")
                .accent(AnsiColor::Magenta)
                .heading(AnsiColor::Magenta)
                .underline_titles(true)
                .build(),
        }
    }
}

/// Builder for application-defined themes. Biscuit Logic products
/// (Terrorbats, Omen, Tethers) construct their own; core stays generic.
#[derive(Debug, Clone)]
pub struct ThemeBuilder {
    name: String,
    accent: AnsiColor,
    heading: Option<AnsiColor>,
    success: AnsiColor,
    warning: AnsiColor,
    failure: AnsiColor,
    muted: AnsiColor,
    evidence: AnsiColor,
    code: AnsiColor,
    path: AnsiColor,
    number: AnsiColor,
    monochrome: bool,
    underline_titles: bool,
}

impl ThemeBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            accent: AnsiColor::Cyan,
            heading: None,
            success: AnsiColor::Green,
            warning: AnsiColor::Yellow,
            failure: AnsiColor::Red,
            muted: AnsiColor::BrightBlack,
            evidence: AnsiColor::BrightBlack,
            code: AnsiColor::BrightWhite,
            path: AnsiColor::BrightBlack,
            number: AnsiColor::BrightWhite,
            monochrome: false,
            underline_titles: false,
        }
    }

    pub fn accent(mut self, c: AnsiColor) -> Self {
        self.accent = c;
        self
    }
    pub fn heading(mut self, c: AnsiColor) -> Self {
        self.heading = Some(c);
        self
    }
    pub fn success(mut self, c: AnsiColor) -> Self {
        self.success = c;
        self
    }
    pub fn warning(mut self, c: AnsiColor) -> Self {
        self.warning = c;
        self
    }
    pub fn failure(mut self, c: AnsiColor) -> Self {
        self.failure = c;
        self
    }
    pub fn muted(mut self, c: AnsiColor) -> Self {
        self.muted = c;
        self
    }
    pub fn evidence(mut self, c: AnsiColor) -> Self {
        self.evidence = c;
        self
    }
    pub fn code(mut self, c: AnsiColor) -> Self {
        self.code = c;
        self
    }
    pub fn path(mut self, c: AnsiColor) -> Self {
        self.path = c;
        self
    }
    pub fn number(mut self, c: AnsiColor) -> Self {
        self.number = c;
        self
    }
    pub fn monochrome(mut self, m: bool) -> Self {
        self.monochrome = m;
        self
    }
    pub fn underline_titles(mut self, u: bool) -> Self {
        self.underline_titles = u;
        self
    }

    pub fn build(self) -> Theme {
        let heading = self.heading.unwrap_or(self.accent);
        Theme {
            name: self.name,
            accent: self.accent,
            heading,
            success: self.success,
            warning: self.warning,
            failure: self.failure,
            muted: self.muted,
            evidence: self.evidence,
            code: self.code,
            path: self.path,
            number: self.number,
            monochrome: self.monochrome,
            underline_titles: self.underline_titles,
        }
    }
}
