use crate::components::section::Section;
use crate::interaction::{read_key, TerminalGuard};
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::action::KeyTrigger;
use crate::semantic::choice::ChoiceItem;
use std::io::{self, stdout, IsTerminal, Write};
use std::time::Duration;

/// Interactive choice selection component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub prompt: String,
    pub items: Vec<ChoiceItem>,
    pub selected_index: usize,
}

impl Choice {
    pub fn new(prompt: impl Into<String>, items: Vec<ChoiceItem>) -> Self {
        Self {
            prompt: prompt.into(),
            items,
            selected_index: 0,
        }
    }

    pub fn with_default_index(mut self, index: usize) -> Self {
        if index < self.items.len() {
            self.selected_index = index;
        }
        self
    }

    /// Run the interactive selector, or return default item deterministically in non-interactive mode.
    pub fn select_interactive(&mut self) -> io::Result<Option<&ChoiceItem>> {
        if self.items.is_empty() {
            return Ok(None);
        }

        if !stdout().is_terminal() {
            return Ok(self.items.get(self.selected_index));
        }

        let ctx = RenderContext::detect();
        let _guard = TerminalGuard::enter()?;

        loop {
            // Render options
            let mut out = stdout();
            self.render_human(&ctx, &mut out)?;
            out.flush()?;

            if let Some(key) = read_key(Duration::from_millis(200))? {
                match key {
                    KeyTrigger::Up => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                    }
                    KeyTrigger::Down => {
                        if self.selected_index + 1 < self.items.len() {
                            self.selected_index += 1;
                        }
                    }
                    KeyTrigger::Enter => {
                        return Ok(self.items.get(self.selected_index));
                    }
                    KeyTrigger::Esc | KeyTrigger::Char('q') => {
                        return Ok(None);
                    }
                    KeyTrigger::Char(c) if c.is_ascii_digit() => {
                        let num = c.to_digit(10).unwrap_or(0) as usize;
                        if num >= 1 && num <= self.items.len() {
                            self.selected_index = num - 1;
                            return Ok(self.items.get(self.selected_index));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl RenderHuman for Choice {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let sec = Section::new(&self.prompt);
        sec.render_human(ctx, out)?;

        for (idx, item) in self.items.iter().enumerate() {
            let is_selected = idx == self.selected_index;
            let marker = if is_selected { "› " } else { "  " };

            let marker_style = if is_selected {
                anstyle::Style::new()
                    .fg_color(Some(ctx.config.accent.into()))
                    .effects(anstyle::Effects::BOLD)
            } else {
                HumanRenderer::muted_style()
            };

            HumanRenderer::write_styled(out, marker_style, marker, ctx.color_enabled)?;
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                &format!("{}. ", idx + 1),
                ctx.color_enabled,
            )?;

            let label_style = if is_selected {
                anstyle::Style::new().effects(anstyle::Effects::BOLD)
            } else {
                HumanRenderer::value_style()
            };
            HumanRenderer::write_styled(out, label_style, &item.label, ctx.color_enabled)?;

            if let Some(ref desc) = item.description {
                write!(out, "  ")?;
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::muted_style(),
                    desc,
                    ctx.color_enabled,
                )?;
            }
            writeln!(out)?;
        }
        Ok(())
    }
}

impl RenderPlain for Choice {
    fn render_plain(&self, _ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "{}", self.prompt)?;
        for (idx, item) in self.items.iter().enumerate() {
            let marker = if idx == self.selected_index { ">" } else { " " };
            write!(out, "{marker} {}. {}", idx + 1, item.label)?;
            if let Some(ref desc) = item.description {
                write!(out, "  ({desc})")?;
            }
            writeln!(out)?;
        }
        Ok(())
    }
}
