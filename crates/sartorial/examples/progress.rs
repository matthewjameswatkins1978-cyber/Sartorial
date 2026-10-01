//! Honest progress: activity for unknown work, counts for known work.
//!
//! Run: `cargo run -p sartorial --example progress`

use sartorial::{MotionMode, Preset, ProgressBar, RenderContext, RenderHuman, Status};

fn main() -> std::io::Result<()> {
    let ctx = RenderContext::human_motion(Preset::Workwear, MotionMode::Never);

    // Unknown totals get activity, never fabricated percentages.
    let activity = ProgressBar::activity("Scanning repository").with_elapsed(3);
    activity.render_human(&ctx, &mut std::io::stderr())?;

    // Known totals show real counts and derived percentages.
    let mut known = ProgressBar::count("Mutating files", 173, 184).unwrap();
    known.update_current(174).unwrap();
    known.render_human(&ctx, &mut std::io::stderr())?;
    known.finish_with_status(Status::Ready);
    known.render_human(&ctx, &mut std::io::stderr())?;
    Ok(())
}
