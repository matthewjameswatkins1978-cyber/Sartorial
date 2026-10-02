use crate::render::context::RenderContext;
#[cfg(feature = "wire")]
use crate::render::RenderAgent;
use crate::render::{RenderHuman, RenderPlain, RenderTarget};
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
            RenderTarget::Markdown => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "markdown target requires print_markdown; refusing implicit format widening",
            )),
            RenderTarget::Agent => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "agent target requires print_agent_json; refusing to emit prose to stdout",
            )),
        }
    }

    /// Emit a Markdown document to stdout.
    pub fn print_markdown<T>(item: &T, ctx: &RenderContext) -> io::Result<()>
    where
        T: crate::render::RenderMarkdown,
    {
        use std::io::Write;
        let md = item.render_markdown(ctx)?;
        let mut out = io::stdout();
        write!(out, "{md}")?;
        if !md.ends_with('\n') {
            writeln!(out)?;
        }
        Ok(())
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
            RenderTarget::Plain | RenderTarget::Agent | RenderTarget::Markdown => {
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
        // Machine output must remain presentation-noise free on both streams.
        if ctx.target.is_agent() {
            return Ok(());
        }
        match ctx.target {
            RenderTarget::Human => {
                let mut err = anstream::stderr();
                item.render_human(ctx, &mut err)
            }
            RenderTarget::Plain | RenderTarget::Agent | RenderTarget::Markdown => {
                let mut err = io::stderr();
                item.render_plain(ctx, &mut err)
            }
        }
    }

    /// Emit clean structured JSON result to stdout only (requires `wire`).
    #[cfg(feature = "wire")]
    pub fn print_agent_json<T>(item: &T, pretty: bool) -> Result<(), serde_json::Error>
    where
        T: RenderAgent,
    {
        let json_str = item.to_agent_json(pretty)?;
        println!("{json_str}");
        Ok(())
    }
}
