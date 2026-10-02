use crate::config::Config;
#[cfg(feature = "interactive")]
use crate::interaction::TerminalGuard;
use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
#[cfg(feature = "interactive")]
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
#[cfg(feature = "interactive")]
use crossterm::execute;
use sartorial_core::render::TerminalRenderer;
use sartorial_core::{Block, ChoiceItem, Document, Presentable};
#[cfg(feature = "interactive")]
use std::io::stdout;
use std::io::{self, Write};

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
///
/// Live keyboard selection requires the `interactive` feature; static
/// display and fail-closed fallback resolution always work.
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

    /// Calculate visual terminal line count, accounting for lines that visually wrap due to terminal width.
    pub fn calculate_rendered_lines(&self, ctx: &RenderContext) -> u16 {
        use unicode_width::UnicodeWidthStr;
        let width = ctx.width.max(1);

        let prompt_width = self.prompt.width();
        let mut total = prompt_width.max(1).div_ceil(width) as u16;

        for (idx, item) in self.items.iter().enumerate() {
            let mut line_width = 2 + format!("{}. ", idx + 1).len() + item.label.width();
            if let Some(ref desc) = item.description {
                line_width += 2 + desc.width();
            }
            total += line_width.max(1).div_ceil(width) as u16;
        }

        total
    }

    /// Execute selection using default configuration.
    /// Without the `interactive` feature this always resolves
    /// non-interactively (fallback or fail-closed).
    pub fn select(&mut self) -> io::Result<ChoiceOutcome<'_>> {
        #[cfg(feature = "interactive")]
        return self.select_with_config(&Config::default());
        #[cfg(not(feature = "interactive"))]
        return Ok(self.resolve_non_interactive());
    }

    /// Execute selection using the single authority of the provided configuration.
    pub fn select_with_config(&mut self, config: &Config) -> io::Result<ChoiceOutcome<'_>> {
        if self.items.is_empty() {
            return Ok(ChoiceOutcome::Cancelled);
        }
        if !config.is_interactive() {
            return Ok(self.resolve_non_interactive());
        }
        #[cfg(not(feature = "interactive"))]
        #[allow(unreachable_code)]
        return Ok(self.resolve_non_interactive());
        #[cfg(feature = "interactive")]
        return self.select_live(config);
    }

    /// Live keyboard selection (requires the `interactive` feature).
    #[cfg(feature = "interactive")]
    fn select_live(&mut self, config: &Config) -> io::Result<ChoiceOutcome<'_>> {
        let ctx = RenderContext::detect().with_config(config.clone());
        let _guard = TerminalGuard::enter()?;
        let mut out = stdout();

        // Initial render
        self.render_human(&ctx, &mut out)?;
        out.flush()?;

        // Calculate exact visual terminal lines accounting for wrapped rows
        let total_lines = self.calculate_rendered_lines(&ctx);

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

    /// Resolve without terminal interaction (always available): honour the
    /// authorized fallback or fail closed.
    pub fn resolve_non_interactive(&self) -> ChoiceOutcome<'_> {
        match self.non_interactive_fallback {
            Some(idx) if idx < self.items.len() => {
                ChoiceOutcome::NonInteractiveFallback(&self.items[idx])
            }
            _ => ChoiceOutcome::NonInteractiveDenied,
        }
    }
}

impl Presentable for Choice {
    fn to_document(&self) -> Document {
        let mut doc = Document::new().push(Block::BadgeSection {
            label: self.prompt.clone(),
            badge: String::new(),
        });
        doc.blocks.push(Block::Choices {
            items: self.items.clone(),
        });
        doc
    }
}

impl RenderHuman for Choice {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        // Section prompt plus per-item selection markers (v0.2 display voice).
        let sec = crate::components::section::Section::new(&self.prompt);
        sec.render_human(ctx, out)?;

        for (idx, item) in self.items.iter().enumerate() {
            let is_selected = idx == self.selected_index;
            let marker = if is_selected { "› " } else { "  " };

            let marker_style = if is_selected {
                ctx.style.key_char_style()
            } else {
                ctx.style.muted_style()
            };

            TerminalRenderer::write_styled(out, marker_style, marker, ctx.color_enabled)?;
            TerminalRenderer::write_styled(
                out,
                ctx.style.muted_style(),
                &format!("{}. ", idx + 1),
                ctx.color_enabled,
            )?;

            let label_style = if is_selected {
                anstyle::Style::new().effects(anstyle::Effects::BOLD)
            } else {
                ctx.style.value_style()
            };
            TerminalRenderer::write_styled(out, label_style, &item.label, ctx.color_enabled)?;

            if let Some(ref desc) = item.description {
                write!(out, "  ")?;
                TerminalRenderer::write_styled(
                    out,
                    ctx.style.muted_style(),
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
