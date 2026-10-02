//! Minimal Sartorial: deterministic static rendering in ten lines.
//!
//! Run: `cargo run -p sartorial --example minimal`

use sartorial::{Outcome, RenderContext, Status, SummaryScreen};

fn main() -> std::io::Result<()> {
    // Batteries-included ergonomic path.
    let screen = SummaryScreen::new("Backup", Status::Ready)
        .fact("Files", "1,204")
        .fact("Duration", "42s");
    sartorial::print_human(&screen)?;

    // Same truth through an explicit context.
    let ctx = RenderContext::plain().with_width(80);
    let outcome = Outcome::new(Status::Ready, "Backup").fact("Files", "1,204");
    let mut out = std::io::stdout();
    sartorial::RenderPlain::render_plain(&outcome, &ctx, &mut out)?;
    Ok(())
}
