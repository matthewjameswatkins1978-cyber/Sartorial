use crate::components::section::Section;
use crate::config::Config;
use crate::interaction::TerminalGuard;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::choice::ChoiceItem;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use std::io::{self, stdout, Write};

/// Explicit result of a choice selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoiceOutcome<'a> {
    /// Explicitly selected by human user.
    Selected(&'a ChoiceItem),
    /// Cancelled by human user (Esc or 'q').
    Cancelled,
    /// Denied automatically because environment was non-interactive and no fallback was authorized.
    NonInteractiveDenied,
    /// Resolved via explicitly authorized non-interactive fallback.
    NonInteractiveFallback(&'a ChoiceItem),
}

impl<'a> ChoiceOutcome<'a> {
    /// Get the resolved ChoiceItem if selected or resolved via fallback.
    pub fn item(&self) -> Option<&'a ChoiceItem> {
        match self {
            Self::Selected(item) | Self::NonInteractiveFallback(item) => Some(item),
            _ => None,
        }
    }
}

/// Interactive choice selection component adhering to interaction authority and fail-closed safety.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub prompt: String,
    pub items: Vec<ChoiceItem>,
    pub selected_index: usize,
    pub non_interactive_fallback: Option<usize>,
}

impl Choice {
    pub fn new(prompt: impl Into<String>, items: Vec<ChoiceItem>) -> Self {
        Self {
            prompt: prompt.into(),
            items,
            selected_index: 0,
            non_interactive_fallback: None,
        }
    }

    /// Set initial selected index for interactive navigation.
    pub fn with_default_index(mut self, index: usize) -> Self {
        if index < self.items.len() {
            self.selected_index = index;
        }
        self
    }

    /// Explicitly authorize a fallback index when executed non-interactively.
    /// Without this, non-interactive execution will strictly fail-closed (`NonInteractiveDenied`).
    pub fn with_non_interactive_fallback(mut self, index: usize) -> Self {
        if index < self.items.len() {
            self.non_interactive_fallback = Some(index);
        }
        self
    }

    /// Execute selection using default configuration.
    pub fn select(&mut self) -> io::Result<ChoiceOutcome<'_>> {
        self.select_with_config(&Config::default())
    }

    /// Execute selection using the single authority of the provided configuration.
    pub fn select_with_config(&mut self, config: &Config) -> io::Result<ChoiceOutcome<'_>> {
        if self.items.is_empty() {
            return Ok(ChoiceOutcome::Cancelled);
        }

        // Single authority for interactivity: fail-closed if non-interactive and no fallback
        if !config.is_interactive() {
            return match self.non_interactive_fallback {
                Some(idx) if idx < self.items.len() => {
                    Ok(ChoiceOutcome::NonInteractiveFallback(&self.items[idx]))
                }
                _ => Ok(ChoiceOutcome::NonInteractiveDenied),
            };
        }

        let ctx = RenderContext::detect().with_config(config.clone());
        let _guard = TerminalGuard::enter()?;
        let mut out = stdout();

        // Initial render
        self.render_human(&ctx, &mut out)?;
        out.flush()?;

        let total_lines = (1 + self.items.len()) as u16;

        loop {
            // Block until event is available: zero output or CPU usage while idle
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) = crossterm::event::read()?
            {
                if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                    return Ok(ChoiceOutcome::Cancelled);
                }

                let mut changed = false;
                match code {
                    KeyCode::Up => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                            changed = true;
                        }
                    }
                    KeyCode::Down => {
                        if self.selected_index + 1 < self.items.len() {
                            self.selected_index += 1;
                            changed = true;
                        }
                    }
                    KeyCode::Enter => {
                        return Ok(ChoiceOutcome::Selected(&self.items[self.selected_index]));
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        return Ok(ChoiceOutcome::Cancelled);
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let num = c.to_digit(10).unwrap_or(0) as usize;
                        if num >= 1 && num <= self.items.len() {
                            self.selected_index = num - 1;
                            return Ok(ChoiceOutcome::Selected(&self.items[self.selected_index]));
                        }
                    }
                    _ => {}
                }

                // Bounded redraw only when state actually changed
                if changed {
                    execute!(out, crossterm::cursor::MoveToPreviousLine(total_lines))?;
                    execute!(
                        out,
                        crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown)
                    )?;
                    self.render_human(&ctx, &mut out)?;
                    out.flush()?;
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
