use crate::config::SymbolMode;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::plain::PlainRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::progress::ProgressState;
use crate::semantic::status::Status;
use std::io::{self, Write};
use std::time::Duration;

/// Progress component providing restrained, state-bearing progress reporting.
/// "Checking repository… cargo test 47s"
pub struct ProgressBar {
    state: ProgressState,
    indicatif_bar: Option<indicatif::ProgressBar>,
}

impl ProgressBar {
    /// Create a progress reporter for a task.
    pub fn new(task: impl Into<String>) -> Self {
        Self {
            state: ProgressState::new(task),
            indicatif_bar: None,
        }
    }

    /// Attach a subtask/activity.
    pub fn with_subtask(mut self, subtask: impl Into<String>) -> Self {
        self.state = self.state.with_subtask(subtask);
        self
    }

    /// Set current progress and total.
    pub fn with_progress(mut self, current: u64, total: u64, unit: impl Into<String>) -> Self {
        self.state = self.state.with_progress(current, total, unit);
        self
    }

    /// Set elapsed seconds.
    pub fn with_elapsed(mut self, elapsed_secs: u64) -> Self {
        self.state = self.state.with_elapsed(elapsed_secs);
        self
    }

    /// Start interactive terminal progress spinner if TTY.
    pub fn start_interactive(&mut self, is_tty: bool) {
        if is_tty {
            let pb = indicatif::ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(100));
            let subtask_str = self.state.subtask.clone().unwrap_or_default();
            pb.set_message(format!("{}… {}", self.state.task, subtask_str));
            self.indicatif_bar = Some(pb);
        }
    }

    /// Finish the progress reporting with final status.
    pub fn finish_with_status(&mut self, status: Status) {
        self.state.status = status;
        if let Some(pb) = self.indicatif_bar.take() {
            pb.finish_and_clear();
        }
    }

    pub fn state(&self) -> &ProgressState {
        &self.state
    }
}

impl RenderHuman for ProgressBar {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let glyph = match ctx.symbols {
            SymbolMode::Ascii => self.state.status.ascii_glyph(),
            _ => self.state.status.unicode_glyph(),
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
            HumanRenderer::section_style(),
            &self.state.task,
            ctx.color_enabled,
        )?;

        let ellipsis = match ctx.symbols {
            SymbolMode::Ascii => "...",
            _ => "…",
        };
        write!(out, "{ellipsis}")?;

        if let Some(ref subtask) = self.state.subtask {
            write!(out, " ")?;
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                subtask,
                ctx.color_enabled,
            )?;
        }

        if let (Some(cur), Some(tot), Some(ref unit)) =
            (self.state.current, self.state.total, &self.state.unit)
        {
            write!(out, "  [{cur}/{tot} {unit}]")?;
        }

        if let Some(secs) = self.state.elapsed_secs {
            write!(out, "  ")?;
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                &format!("{secs}s"),
                ctx.color_enabled,
            )?;
        }

        writeln!(out)
    }
}

impl RenderPlain for ProgressBar {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        PlainRenderer::write_status(out, self.state.status, ctx)?;
        write!(out, " {}...", self.state.task)?;
        if let Some(ref subtask) = self.state.subtask {
            write!(out, " {subtask}")?;
        }
        if let Some(secs) = self.state.elapsed_secs {
            write!(out, " {secs}s")?;
        }
        writeln!(out)
    }
}
