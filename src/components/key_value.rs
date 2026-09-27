use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::fact::Fact;
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// A structured list of key-value facts aligned according to BL typography.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyValueList {
    pub facts: Vec<Fact>,
}

impl KeyValueList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_facts(facts: Vec<Fact>) -> Self {
        Self { facts }
    }

    pub fn add(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.facts.push(Fact::new(name, value));
        self
    }

    pub fn with_fact(mut self, fact: Fact) -> Self {
        self.facts.push(fact);
        self
    }
}

impl RenderHuman for KeyValueList {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        if self.facts.is_empty() {
            return Ok(());
        }

        let max_label_len = self.facts.iter().map(|f| f.name.width()).max().unwrap_or(0);
        let pad_label = (max_label_len + 2).min(30);

        for fact in &self.facts {
            if ctx.is_narrow() {
                // Stacked format for narrow terminals (< 60 cols)
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::label_style(),
                    &fact.name,
                    ctx.color_enabled,
                )?;
                writeln!(out, ":")?;
                write!(out, "  ")?;
                let val_style = if fact.muted {
                    HumanRenderer::muted_style()
                } else {
                    HumanRenderer::value_style()
                };
                HumanRenderer::write_styled(out, val_style, &fact.value, ctx.color_enabled)?;
                if let Some(ref unit) = fact.unit {
                    write!(out, " ")?;
                    HumanRenderer::write_styled(
                        out,
                        HumanRenderer::muted_style(),
                        unit,
                        ctx.color_enabled,
                    )?;
                }
                writeln!(out)?;
            } else {
                // Aligned columns for standard/wide terminals
                let label_width = fact.name.width();
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::label_style(),
                    &fact.name,
                    ctx.color_enabled,
                )?;
                let pad = if pad_label > label_width {
                    pad_label - label_width
                } else {
                    2
                };
                write!(out, "{}", " ".repeat(pad))?;

                let val_style = if fact.muted {
                    HumanRenderer::muted_style()
                } else {
                    HumanRenderer::value_style()
                };
                HumanRenderer::write_styled(out, val_style, &fact.value, ctx.color_enabled)?;
                if let Some(ref unit) = fact.unit {
                    write!(out, " ")?;
                    HumanRenderer::write_styled(
                        out,
                        HumanRenderer::muted_style(),
                        unit,
                        ctx.color_enabled,
                    )?;
                }
                writeln!(out)?;
            }
        }
        Ok(())
    }
}

impl RenderPlain for KeyValueList {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let max_label_len = self.facts.iter().map(|f| f.name.width()).max().unwrap_or(0);
        let pad_label = (max_label_len + 2).min(30);

        for fact in &self.facts {
            if ctx.is_narrow() {
                writeln!(out, "{}:\n  {}", fact.name, fact.value)?;
            } else {
                let label_width = fact.name.width();
                let pad = if pad_label > label_width {
                    pad_label - label_width
                } else {
                    2
                };
                let unit_suffix = fact
                    .unit
                    .as_ref()
                    .map(|u| format!(" {u}"))
                    .unwrap_or_default();
                writeln!(
                    out,
                    "{}{}{}{}",
                    fact.name,
                    " ".repeat(pad),
                    fact.value,
                    unit_suffix
                )?;
            }
        }
        Ok(())
    }
}
