use crate::components::progress::ProgressBar;
use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use std::io::{self, Write};

/// Restrained multi-task progress presentation for concurrent real operations.
pub struct MultiProgressView {
    pub tasks: Vec<ProgressBar>,
}

impl MultiProgressView {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, progress: ProgressBar) -> Self {
        self.tasks.push(progress);
        self
    }
}

impl Default for MultiProgressView {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderHuman for MultiProgressView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        for task in &self.tasks {
            task.render_human(ctx, out)?;
        }
        Ok(())
    }
}

impl RenderPlain for MultiProgressView {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        for task in &self.tasks {
            task.render_plain(ctx, out)?;
        }
        Ok(())
    }
}
