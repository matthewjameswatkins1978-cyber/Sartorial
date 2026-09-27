use crate::render::context::RenderContext;
use crate::render::{RenderAgent, RenderHuman, RenderPlain, RenderTarget};
use std::io;

/// Output channel routing helpers ensuring standard channel hygiene:
/// - Primary operation results -> STDOUT
/// - Progress diagnostics and spinners -> STDERR
/// - Warnings and non-fatal diagnostics -> STDERR
/// - Structured Agent JSON -> STDOUT only (never polluted by STDERR diagnostics)
pub struct SartorialOutput;

impl SartorialOutput {
    /// Emit a primary human/plain command result to stdout.
    pub fn print_result<T>(item: &T, ctx: &RenderContext) -> io::Result<()>
    where
        T: RenderHuman + RenderPlain,
    {
        match ctx.target {
            RenderTarget::Human => {
                let mut out = anstream::stdout();
                item.render_human(ctx, &mut out)
            }
            RenderTarget::Plain => {
                let mut out = io::stdout();
                item.render_plain(ctx, &mut out)
            }
            RenderTarget::Agent => {
                // Agent mode should use print_agent_json directly
                let mut out = io::stdout();
                item.render_plain(ctx, &mut out)
            }
        }
    }

    /// Emit an in-flight progress state or ephemeral notification to stderr.
    pub fn print_progress<T>(item: &T, ctx: &RenderContext) -> io::Result<()>
    where
        T: RenderHuman + RenderPlain,
    {
        // If agent JSON mode is active, do not output prose or animation to stderr
        if ctx.target.is_agent() {
            return Ok(());
        }
        match ctx.target {
            RenderTarget::Human => {
                let mut err = anstream::stderr();
                item.render_human(ctx, &mut err)
            }
            RenderTarget::Plain | RenderTarget::Agent => {
                let mut err = io::stderr();
                item.render_plain(ctx, &mut err)
            }
        }
    }

    /// Emit a warning or diagnostic message to stderr.
    pub fn print_diagnostic<T>(item: &T, ctx: &RenderContext) -> io::Result<()>
    where
        T: RenderHuman + RenderPlain,
    {
        match ctx.target {
            RenderTarget::Human => {
                let mut err = anstream::stderr();
                item.render_human(ctx, &mut err)
            }
            RenderTarget::Plain | RenderTarget::Agent => {
                let mut err = io::stderr();
                item.render_plain(ctx, &mut err)
            }
        }
    }

    /// Emit clean structured JSON result to stdout only.
    pub fn print_agent_json<T>(item: &T, pretty: bool) -> Result<(), serde_json::Error>
    where
        T: RenderAgent,
    {
        let json_str = item.to_agent_json(pretty)?;
        println!("{json_str}");
        Ok(())
    }
}
