use crate::config::SymbolMode;
use crate::motion::ProgressMode;
use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::progress::{ProgressError, ProgressState};
use crate::semantic::status::Status;
use crate::style::ProgressTreatment;
use sartorial_core::render::{PlainRenderer, TerminalRenderer};
use std::io::{self, Write};
#[cfg(feature = "progress")]
use std::time::Duration;

/// Placeholder live-bar slot when the `progress` feature is disabled.
/// Live animation is unavailable; static honest rendering always works.
#[cfg(not(feature = "progress"))]
#[derive(Debug)]
struct NoLiveBar;

/// Progress component providing restrained, truthful progress reporting under the BL Motion Standard.
///
/// Live presentation uses the **stderr** TTY (spinners draw on stderr), while
/// the render context is detected from the **stdout** TTY. The split is
/// deliberate: result output may be piped to a file while progress still
/// animates on an attended terminal, and vice versa. See
/// [`ProgressBar::start_live`] for the exact rules.
pub struct ProgressBar {
    state: ProgressState,
    #[cfg(feature = "progress")]
    indicatif_bar: Option<indicatif::ProgressBar>,
    #[cfg(not(feature = "progress"))]
    indicatif_bar: Option<NoLiveBar>,
    /// Last static line flushed to stderr (Minimal/Numeric treatments).
    last_static_line: Option<String>,
}

impl ProgressBar {
    pub fn new(task: impl Into<String>) -> Self {
        Self {
            state: ProgressState::new(task),
            indicatif_bar: None,
            last_static_line: None,
        }
    }

    pub fn activity(task: impl Into<String>) -> Self {
        Self {
            state: ProgressState::activity(task),
            indicatif_bar: None,
            last_static_line: None,
        }
    }

    pub fn count(task: impl Into<String>, current: u64, total: u64) -> Result<Self, ProgressError> {
        Ok(Self {
            state: ProgressState::count(task, current, total)?,
            indicatif_bar: None,
            last_static_line: None,
        })
    }

    pub fn percent(task: impl Into<String>, percent: u8) -> Self {
        Self {
            state: ProgressState::percent(task, percent),
            indicatif_bar: None,
            last_static_line: None,
        }
    }

    pub fn countdown(task: impl Into<String>, secs: u64) -> Self {
        Self {
            state: ProgressState::countdown(task, secs),
            indicatif_bar: None,
            last_static_line: None,
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
            last_static_line: None,
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
    /// Always false without the `progress` feature.
    pub fn is_animating(&self) -> bool {
        #[cfg(feature = "progress")]
        {
            self.indicatif_bar.is_some()
        }
        #[cfg(not(feature = "progress"))]
        {
            false
        }
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
        #[cfg(feature = "progress")]
        if let Some(pb) = self.indicatif_bar.take() {
            pb.finish_and_clear();
        }
    }

    /// Single authority for beginning live progress presentation with explicit TTY capability.
    ///
    /// `is_tty` must describe the **stderr** stream (the progress channel):
    /// pass `std::io::stderr().is_terminal()`. Animation additionally
    /// requires a human target, a permitting motion policy, and a preset
    /// treatment that animates at all — Minimal (Black Tie) and Numeric
    /// (Workwear) never spin; they emit one static line at start, one per
    /// semantic change, and one at completion.
    pub fn start_live_with_tty(&mut self, ctx: &RenderContext, is_tty: bool) -> io::Result<()> {
        self.state
            .validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        let treatment = ctx.style.progress_treatment;
        let static_protocol = matches!(
            treatment,
            ProgressTreatment::Minimal | ProgressTreatment::Numeric
        );
        if ctx.should_animate(is_tty) && !static_protocol {
            #[cfg(feature = "progress")]
            {
                Self::start_spinner(self, ctx);
            }
            #[cfg(not(feature = "progress"))]
            {
                // No live machinery: fall back to the static line protocol.
                self.flush_static_line(ctx)?;
            }
        } else if static_protocol {
            self.flush_static_line(ctx)?;
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

    /// Live spinner construction (requires the `progress` feature).
    #[cfg(feature = "progress")]
    fn start_spinner(&mut self, ctx: &RenderContext) {
        let treatment = ctx.style.progress_treatment;
        let pb = indicatif::ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100)); // 10 Hz rate limit

        let template = match treatment {
            ProgressTreatment::Numeric => "{msg}",
            ProgressTreatment::Minimal => "{spinner} {msg}",
            ProgressTreatment::Expressive => "{spinner} {msg}",
            ProgressTreatment::Restrained => "{spinner} {msg}",
        };
        let tick_chars = if ctx.symbols == SymbolMode::Ascii {
            "/-\\| "
        } else {
            match treatment {
                ProgressTreatment::Minimal => "• ",
                ProgressTreatment::Numeric => "- ",
                ProgressTreatment::Restrained => "◐◑◒◓ ",
                ProgressTreatment::Expressive => "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ",
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
    }

    /// Unstyled static line for the Minimal/Numeric protocol: what is running
    /// and where it is, with real counts only. Never percentages, never spin.
    fn static_line(&self, ctx: &RenderContext) -> String {
        let mut evidence = String::new();
        if let (Some(cur), Some(tot)) = (self.state.current, self.state.total) {
            evidence.push_str(&format!("  {cur}/{tot}"));
            if let Some(ref unit) = self.state.unit {
                evidence.push_str(&format!(" {unit}"));
            }
        }
        if let Some(ref rate) = self.state.rate {
            evidence.push_str(&format!("  {rate}"));
        }
        match ctx.style.progress_treatment {
            ProgressTreatment::Numeric => {
                let mut line = format!(
                    "{}{}  ",
                    ctx.style.section_marker,
                    self.state.task.to_uppercase()
                );
                if let Some(ref sub) = self.state.subtask {
                    line.push_str(sub);
                }
                line.push_str(&evidence);
                line
            }
            _ => {
                let mut line = self.state.task.clone();
                if let Some(ref sub) = self.state.subtask {
                    line.push_str(&format!(" · {sub}"));
                }
                line.push_str(&evidence);
                line
            }
        }
    }

    /// Print the static line, but only when it changed since the last flush:
    /// Minimal moves only when information changes.
    fn flush_static_line(&mut self, ctx: &RenderContext) -> io::Result<()> {
        let line = self.static_line(ctx);
        if self.last_static_line.as_deref() != Some(line.as_str()) {
            if ctx.target.is_plain() {
                let mut err = io::stderr();
                writeln!(err, "{line}")?;
            } else if !ctx.target.is_agent() {
                let mut err = anstream::stderr();
                writeln!(err, "{line}")?;
            }
            self.last_static_line = Some(line);
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
    ///
    /// Spinner treatments update the spinner message in place. Minimal and
    /// Numeric treatments reprint a static line, but only when the semantic
    /// content actually changed (new phase, new counts) — never a busy tick.
    /// Non-animating House/Studio print nothing here.
    pub fn update_live(&mut self, ctx: &RenderContext) -> io::Result<()> {
        #[cfg(feature = "progress")]
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
            return Ok(());
        }
        match ctx.style.progress_treatment {
            ProgressTreatment::Minimal | ProgressTreatment::Numeric => self.flush_static_line(ctx),
            // Non-TTY / static: do NOT print repeated updates (prevents line spam in logs/pipes!)
            _ => Ok(()),
        }
    }

    /// Cleanly finish and clear live presentation, leaving final status line where appropriate.
    pub fn finish_live(&mut self, status: Status, ctx: &RenderContext) -> io::Result<()> {
        self.finish_with_status(status);
        #[cfg(feature = "progress")]
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

        // Workwear: operational voice. `» SCAN  languages`, `» COPY  18/42`,
        // completion `✓ SCAN  finalize`. Real counts and rates foregrounded;
        // nothing manufactured (Activity simply shows no numbers).
        if treatment == ProgressTreatment::Numeric {
            if self.state.status == Status::Running {
                write!(out, "{}", ctx.style.section_marker)?;
            } else {
                let glyph = match ctx.symbols {
                    SymbolMode::Ascii => self.state.status.ascii_glyph(),
                    _ => self.state.status.unicode_glyph(),
                };
                if ctx.color_enabled {
                    TerminalRenderer::write_styled(
                        out,
                        ctx.style.status_style(self.state.status),
                        glyph,
                        true,
                    )?;
                } else {
                    write!(out, "{glyph}")?;
                }
                write!(out, " ")?;
            }
            TerminalRenderer::write_styled(
                out,
                ctx.style.key_char_style(),
                &self.state.task.to_uppercase(),
                ctx.color_enabled,
            )?;
            if let Some(ref subtask) = self.state.subtask {
                write!(out, "  {subtask}")?;
            }
            if let (Some(cur), Some(tot)) = (self.state.current, self.state.total) {
                write!(out, "  {cur}/{tot}")?;
                if let Some(ref unit) = self.state.unit {
                    write!(out, " {unit}")?;
                }
            } else if let Some(pct) = self.state.derived_percent() {
                write!(out, "  {pct}%")?;
            }
            if let Some(ref rate) = self.state.rate {
                write!(out, "  {rate}")?;
            }
            if let Some(secs) = self.state.elapsed_secs {
                write!(out, "  {secs}s")?;
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
            let style = ctx.style.status_style(self.state.status);
            TerminalRenderer::write_styled(out, style, glyph, true)?;
        } else {
            write!(out, "{glyph}")?;
        }
        write!(out, " ")?;

        TerminalRenderer::write_styled(
            out,
            ctx.style.section_style(),
            &self.state.task,
            ctx.color_enabled,
        )?;

        if let Some(ref subtask) = self.state.subtask {
            let sep = if is_ascii { " * " } else { " · " };
            write!(out, "{sep}")?;
            TerminalRenderer::write_styled(
                out,
                ctx.style.muted_style(),
                subtask,
                ctx.color_enabled,
            )?;
        }

        // Progress bar for percent mode
        if self.state.mode == ProgressMode::Percent {
            if let Some(pct) = self.state.derived_percent() {
                let bar_width = if ctx.is_narrow() { 10 } else { 18 };
                let bar = Self::format_bar(pct, bar_width, is_ascii);
                write!(out, "  ")?;
                TerminalRenderer::write_styled(
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
            TerminalRenderer::write_styled(
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
    fn render_plain(&self, _ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        self.state
            .validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        PlainRenderer::write_status(out, self.state.status)?;
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
            #[cfg(feature = "progress")]
            indicatif_bar: None,
            #[cfg(not(feature = "progress"))]
            indicatif_bar: None,
            last_static_line: None,
        }
    }
}

impl sartorial_core::Presentable for ProgressBar {
    fn to_document(&self) -> sartorial_core::Document {
        self.state.to_document()
    }
}
