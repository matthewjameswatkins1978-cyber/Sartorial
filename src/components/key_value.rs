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

        let labels: Vec<String> = self
            .facts
            .iter()
            .map(|f| ctx.style.format_fact_label(&f.name))
            .collect();
        let max_label_len = labels.iter().map(|l| l.width()).max().unwrap_or(0);
        let pad_spacing = ctx.style.fact_pad(ctx.is_narrow());
        let pad_label = (max_label_len + pad_spacing).min(30);

        for (fact, label) in self.facts.iter().zip(labels.iter()) {
            if ctx.is_narrow() {
                // Stacked format for narrow terminals (< 60 cols)
                HumanRenderer::write_styled(
                    out,
                    ctx.style.label_style(),
                    label,
                    ctx.color_enabled,
                )?;
                if !label.ends_with(':') {
                    write!(out, ":")?;
                }
                writeln!(out)?;
                write!(out, "  ")?;
                let val_style = if fact.muted {
                    ctx.style.muted_style()
                } else {
                    ctx.style.value_style()
                };
                HumanRenderer::write_styled(out, val_style, &fact.value, ctx.color_enabled)?;
                if let Some(ref unit) = fact.unit {
                    write!(out, " ")?;
                    HumanRenderer::write_styled(
                        out,
                        ctx.style.muted_style(),
                        unit,
                        ctx.color_enabled,
                    )?;
                }
                writeln!(out)?;
            } else {
                // Aligned columns for standard/wide terminals
                let label_width = label.width();
                HumanRenderer::write_styled(
                    out,
                    ctx.style.label_style(),
                    label,
                    ctx.color_enabled,
                )?;
                let pad = if pad_label > label_width {
                    pad_label - label_width
                } else {
                    pad_spacing
                };
                write!(out, "{}", " ".repeat(pad))?;

                let val_style = if fact.muted {
                    ctx.style.muted_style()
                } else {
                    ctx.style.value_style()
                };
                HumanRenderer::write_styled(out, val_style, &fact.value, ctx.color_enabled)?;
                if let Some(ref unit) = fact.unit {
                    write!(out, " ")?;
                    HumanRenderer::write_styled(
                        out,
                        ctx.style.muted_style(),
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
        let labels: Vec<String> = self
            .facts
            .iter()
            .map(|f| ctx.style.format_fact_label(&f.name))
            .collect();
        let max_label_len = labels.iter().map(|l| l.width()).max().unwrap_or(0);
        let pad_spacing = ctx.style.fact_pad(ctx.is_narrow());
        let pad_label = (max_label_len + pad_spacing).min(30);

        for (fact, label) in self.facts.iter().zip(labels.iter()) {
            if ctx.is_narrow() {
                if label.ends_with(':') {
                    writeln!(out, "{label}\n  {}", fact.value)?;
                } else {
                    writeln!(out, "{label}:\n  {}", fact.value)?;
                }
            } else {
                let label_width = label.width();
                let pad = if pad_label > label_width {
                    pad_label - label_width
                } else {
                    pad_spacing
                };
                let unit_suffix = fact
                    .unit
                    .as_ref()
                    .map(|u| format!(" {u}"))
                    .unwrap_or_default();
                writeln!(
                    out,
                    "{label}{}{}{}",
                    " ".repeat(pad),
                    fact.value,
                    unit_suffix
                )?;
            }
        }
        Ok(())
    }
}
