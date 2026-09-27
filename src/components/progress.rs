use crate::config::SymbolMode;
use crate::motion::ProgressMode;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::plain::PlainRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::progress::{ProgressError, ProgressState};
use crate::semantic::status::Status;
use crate::style::ProgressTreatment;
use std::io::{self, Write};
use std::time::Duration;

/// Progress component providing restrained, truthful progress reporting under the BL Motion Standard.
pub struct ProgressBar {
    state: ProgressState,
    indicatif_bar: Option<indicatif::ProgressBar>,
}

impl ProgressBar {
    pub fn new(task: impl Into<String>) -> Self {
        Self {
            state: ProgressState::new(task),
            indicatif_bar: None,
        }
    }

    pub fn activity(task: impl Into<String>) -> Self {
        Self {
            state: ProgressState::activity(task),
            indicatif_bar: None,
        }
    }

    pub fn count(task: impl Into<String>, current: u64, total: u64) -> Result<Self, ProgressError> {
        Ok(Self {
            state: ProgressState::count(task, current, total)?,
            indicatif_bar: None,
        })
    }

    pub fn percent(task: impl Into<String>, percent: u8) -> Self {
        Self {
            state: ProgressState::percent(task, percent),
            indicatif_bar: None,
        }
    }

    pub fn countdown(task: impl Into<String>, secs: u64) -> Self {
        Self {
            state: ProgressState::countdown(task, secs),
            indicatif_bar: None,
        }
    }

    pub fn rate(
        task: impl Into<String>,
        current: u64,
        total: u64,
        unit: impl Into<String>,
        rate: impl Into<String>,
    ) -> Result<Self, ProgressError> {
        Ok(Self {
            state: ProgressState::rate(task, current, total, unit, rate)?,
            indicatif_bar: None,
        })
    }

    pub fn with_subtask(mut self, subtask: impl Into<String>) -> Self {
        self.state = self.state.with_subtask(subtask);
        self
    }

    pub fn with_progress(
        mut self,
        current: u64,
        total: u64,
        unit: impl Into<String>,
    ) -> Result<Self, ProgressError> {
        self.state = self.state.with_progress(current, total, unit)?;
        Ok(self)
    }

    pub fn with_elapsed(mut self, elapsed_secs: u64) -> Self {
        self.state = self.state.with_elapsed(elapsed_secs);
        self
    }

    pub fn with_rate(mut self, rate: impl Into<String>) -> Self {
        self.state = self.state.with_rate(rate);
        self
    }

    pub fn with_countdown(mut self, secs: u64) -> Self {
        self.state = self.state.with_countdown(secs);
        self
    }

    pub fn state(&self) -> &ProgressState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ProgressState {
        &mut self.state
    }

    /// Whether live terminal animation is currently running.
    pub fn is_animating(&self) -> bool {
        self.indicatif_bar.is_some()
    }

    /// Apply an update enforcing single authority and semantic invariants.
    pub fn apply_update(
        &mut self,
        current: Option<u64>,
        percent: Option<u8>,
        rate: Option<String>,
        elapsed_secs: Option<u64>,
        subtask: Option<String>,
    ) -> Result<(), ProgressError> {
        self.state
            .apply_update(current, percent, rate, elapsed_secs, subtask)
    }

    pub fn update_current(&mut self, cur: u64) -> Result<(), ProgressError> {
        self.state.update_current(cur)
    }

    pub fn update_total(&mut self, tot: u64) -> Result<(), ProgressError> {
        self.state.update_total(tot)
    }

    pub fn update_percent(&mut self, pct: u8) -> Result<(), ProgressError> {
        self.state.update_percent(pct)
    }

    pub fn update_rate(&mut self, rate: impl Into<String>) {
        self.state.update_rate(rate);
    }

    pub fn update_subtask(&mut self, subtask: impl Into<String>) {
        self.state.update_subtask(subtask);
    }

    pub fn update_elapsed(&mut self, secs: u64) {
        self.state.update_elapsed(secs);
    }

    pub fn finish_with_status(&mut self, status: Status) {
        self.state.status = status;
        if let Some(pb) = self.indicatif_bar.take() {
            pb.finish_and_clear();
        }
    }

    /// Single authority for beginning live progress presentation with explicit TTY capability.
    pub fn start_live_with_tty(&mut self, ctx: &RenderContext, is_tty: bool) -> io::Result<()> {
        self.state
            .validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        if ctx.should_animate(is_tty) {
            let pb = indicatif::ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(100)); // 10 Hz rate limit

            let template = match ctx.style.progress_treatment {
                ProgressTreatment::Numeric => "{msg}",
                ProgressTreatment::Minimal => "{spinner} {msg}",
                ProgressTreatment::Expressive => "{spinner} {msg}",
                ProgressTreatment::Restrained => "{spinner} {msg}",
            };
            let tick_chars = if ctx.symbols == SymbolMode::Ascii {
                "/-\\| "
            } else {
                match ctx.style.progress_treatment {
                    ProgressTreatment::Minimal => "• ",
                    ProgressTreatment::Numeric => "- ",
                    _ => "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ",
                }
            };
            if let Ok(style) = indicatif::ProgressStyle::default_spinner()
                .tick_chars(tick_chars)
                .template(template)
            {
                pb.set_style(style);
            }

            let msg = if let Some(ref sub) = self.state.subtask {
                format!("{} - {sub}", self.state.task)
            } else {
                self.state.task.clone()
            };
            pb.set_message(msg);
            self.indicatif_bar = Some(pb);
        } else if ctx.target.is_plain() {
            let mut err = io::stderr();
            write!(err, "Starting: {}", self.state.task)?;
            if let Some(ref sub) = self.state.subtask {
                write!(err, " - {sub}")?;
            }
            writeln!(err)?;
        } else if !ctx.target.is_agent() {
            let mut err = anstream::stderr();
            write!(err, "Starting: {}", self.state.task)?;
            if let Some(ref sub) = self.state.subtask {
                write!(err, " - {sub}")?;
            }
            writeln!(err)?;
        }
        Ok(())
    }

    /// Single authority for beginning live progress presentation.
    ///
    /// Respects:
    /// - `Config::motion`
    /// - `AccessibilityMode` (reduced motion)
    /// - `RenderTarget` (plain text and agent JSON)
    /// - TTY capability (non-interactive stdout/stderr)
    /// - `ResolvedStyle` / preset progress treatment
    pub fn start_live(&mut self, ctx: &RenderContext) -> io::Result<()> {
        use std::io::IsTerminal;
        let is_tty = std::io::stderr().is_terminal();
        self.start_live_with_tty(ctx, is_tty)
    }

    /// Backwards compatibility helper delegating to start_live_with_tty without ignoring supplied is_tty.
    pub fn start_interactive(&mut self, is_tty: bool) {
        let ctx = RenderContext::detect();
        let _ = self.start_live_with_tty(&ctx, is_tty);
    }

    /// Visibly update live progress.
    pub fn update_live(&mut self, _ctx: &RenderContext) -> io::Result<()> {
        if let Some(ref pb) = self.indicatif_bar {
            if let Some(cur) = self.state.current {
                pb.set_position(cur);
            }
            let mut msg = self.state.task.clone();
            if let Some(ref sub) = self.state.subtask {
                msg.push_str(&format!(" - {sub}"));
            }
            if let (Some(cur), Some(tot)) = (self.state.current, self.state.total) {
                let unit = self.state.unit.as_deref().unwrap_or("units");
                msg.push_str(&format!("  {cur} / {tot} {unit}"));
            } else if let Some(pct) = self.state.derived_percent() {
                msg.push_str(&format!("  {pct}%"));
            }
            if let Some(ref r) = self.state.rate {
                msg.push_str(&format!("  {r}"));
            }
            if let Some(s) = self.state.elapsed_secs {
                msg.push_str(&format!("  {s}s"));
            }
            pb.set_message(msg);
        }
        // Non-TTY / static: do NOT print repeated updates (prevents line spam in logs/pipes!)
        Ok(())
    }

    /// Cleanly finish and clear live presentation, leaving final status line where appropriate.
    pub fn finish_live(&mut self, status: Status, ctx: &RenderContext) -> io::Result<()> {
        self.finish_with_status(status);
        if let Some(pb) = self.indicatif_bar.take() {
            pb.finish_and_clear();
        }
        if ctx.target.is_plain() {
            let mut err = io::stderr();
            self.render_plain(ctx, &mut err)?;
        } else if !ctx.target.is_agent() {
            let mut err = anstream::stderr();
            self.render_human(ctx, &mut err)?;
        }
        Ok(())
    }

    /// Formats a clean progress bar of the specified width.
    pub fn format_bar(percent: u8, width: usize, is_ascii: bool) -> String {
        let p = percent.min(100) as usize;
        let filled = (p * width) / 100;
        let remaining = width.saturating_sub(filled);

        if is_ascii {
            let fill_char = '=';
            let head_char = if filled > 0 && remaining > 0 {
                ">"
            } else {
                "="
            };
            let empty_char = '-';
            let f = if filled > 0 { filled - 1 } else { 0 };
            format!(
                "{}{}{}",
                fill_char.to_string().repeat(f),
                if filled > 0 { head_char } else { "" },
                empty_char.to_string().repeat(remaining)
            )
        } else {
            let fill_str = "━".repeat(if filled > 0 { filled - 1 } else { 0 });
            let head = if filled > 0 && remaining > 0 {
                "╸"
            } else if filled > 0 {
                "━"
            } else {
                ""
            };
            let rem_str = "─".repeat(remaining);
            format!("{fill_str}{head}{rem_str}")
        }
    }
}

impl RenderHuman for ProgressBar {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        self.state
            .validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        let is_ascii = ctx.symbols == SymbolMode::Ascii;
        let treatment = ctx.style.progress_treatment;

        // Workwear: Compact numeric bracket format "[38/60] 63%  00:14  cargo test"
        if treatment == ProgressTreatment::Numeric {
            let count_str = match (self.state.current, self.state.total) {
                (Some(c), Some(t)) => format!("[{c}/{t}]"),
                _ => {
                    if let Some(pct) = self.state.derived_percent() {
                        format!("[{pct}%]")
                    } else {
                        "[-]".to_string()
                    }
                }
            };
            HumanRenderer::write_styled(
                out,
                ctx.style.key_char_style(),
                &count_str,
                ctx.color_enabled,
            )?;

            if self.state.current.is_some() && self.state.total.is_some() {
                if let Some(pct) = self.state.derived_percent() {
                    write!(out, " {pct}%")?;
                }
            }

            if let Some(secs) = self.state.elapsed_secs {
                let mins = secs / 60;
                let rem_s = secs % 60;
                write!(out, "  {:02}:{:02}", mins, rem_s)?;
            }

            if let Some(ref subtask) = self.state.subtask {
                write!(out, "  ")?;
                HumanRenderer::write_styled(
                    out,
                    ctx.style.muted_style(),
                    subtask,
                    ctx.color_enabled,
                )?;
            } else {
                write!(out, "  {}", self.state.task)?;
            }
            return writeln!(out);
        }

        // House, Black Tie, Studio format
        let glyph = if self.state.status == Status::Running {
            match treatment {
                ProgressTreatment::Minimal => {
                    if is_ascii {
                        "."
                    } else {
                        "•"
                    }
                }
                ProgressTreatment::Expressive => {
                    if is_ascii {
                        "*"
                    } else {
                        "⠋"
                    }
                }
                _ => {
                    if is_ascii {
                        "*"
                    } else {
                        "◐"
                    }
                }
            }
        } else {
            match ctx.symbols {
                SymbolMode::Ascii => self.state.status.ascii_glyph(),
                _ => self.state.status.unicode_glyph(),
            }
        };

        if ctx.color_enabled {
            let style = self.state.status.style();
            HumanRenderer::write_styled(out, style, glyph, true)?;
        } else {
            write!(out, "{glyph}")?;
        }
        write!(out, " ")?;

        HumanRenderer::write_styled(
            out,
            ctx.style.section_style(),
            &self.state.task,
            ctx.color_enabled,
        )?;

        if let Some(ref subtask) = self.state.subtask {
            let sep = if is_ascii { " * " } else { " · " };
            write!(out, "{sep}")?;
            HumanRenderer::write_styled(out, ctx.style.muted_style(), subtask, ctx.color_enabled)?;
        }

        // Progress bar for percent mode
        if self.state.mode == ProgressMode::Percent {
            if let Some(pct) = self.state.derived_percent() {
                let bar_width = if ctx.is_narrow() { 10 } else { 18 };
                let bar = Self::format_bar(pct, bar_width, is_ascii);
                write!(out, "  ")?;
                HumanRenderer::write_styled(
                    out,
                    ctx.style.key_char_style(),
                    &bar,
                    ctx.color_enabled,
                )?;
                write!(out, "  {pct}%")?;
            }
        } else if let (Some(cur), Some(tot)) = (self.state.current, self.state.total) {
            write!(out, "  {cur} / {tot}")?;
            if let Some(ref unit) = self.state.unit {
                write!(out, " {unit}")?;
            }
        }

        if let Some(ref rate) = self.state.rate {
            write!(out, "  {rate}")?;
        }

        if let Some(rem_secs) = self.state.countdown_secs {
            write!(out, "  in {:02}s", rem_secs)?;
        } else if let Some(secs) = self.state.elapsed_secs {
            write!(out, "  ")?;
            HumanRenderer::write_styled(
                out,
                ctx.style.muted_style(),
                &format!("{secs}s"),
                ctx.color_enabled,
            )?;
        }

        writeln!(out)
    }
}

impl RenderPlain for ProgressBar {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        self.state
            .validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        PlainRenderer::write_status(out, self.state.status, ctx)?;
        write!(out, " {}", self.state.task)?;
        if let Some(ref subtask) = self.state.subtask {
            write!(out, " - {subtask}")?;
        }
        if let (Some(cur), Some(tot)) = (self.state.current, self.state.total) {
            write!(out, " ({cur}/{tot})")?;
        }
        if let Some(pct) = self.state.derived_percent() {
            write!(out, " {pct}%")?;
        }
        if let Some(secs) = self.state.elapsed_secs {
            write!(out, " {secs}s")?;
        }
        if let Some(secs) = self.state.countdown_secs {
            write!(out, " remaining: {secs}s")?;
        }
        writeln!(out)
    }
}

impl From<ProgressState> for ProgressBar {
    fn from(state: ProgressState) -> Self {
        Self {
            state,
            indicatif_bar: None,
        }
    }
}
