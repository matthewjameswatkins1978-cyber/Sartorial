//! Plan before, receipt after: consequential operations presented honestly.
//!
//! Run: `cargo run -p sartorial --example plan_receipt`

use sartorial::{Action, Plan, Preset, Receipt, RenderContext, RenderPlain, Status};

fn main() -> std::io::Result<()> {
    let ctx = RenderContext::plain_preset(Preset::House).with_width(80);
    let mut out = std::io::stdout();

    // BEFORE: intent, never execution.
    let plan = Plan::new("Rotate API keys")
        .with_description("Proposed changes")
        .add("api-key-new")
        .remove("api-key-old")
        .warning("Old key stops working immediately.")
        .reversible(false)
        .with_action(Action::open());
    plan.render_plain(&ctx, &mut out)?;

    // AFTER: truthful changes plus material non-changes.
    let receipt = Receipt::success("Keys rotated")
        .with_status(Status::Ready)
        .change("Active key", "api-key-new")
        .unchanged("Permissions", "unchanged")
        .guidance("Update CI secrets before the next run.")
        .with_action(Action::quit());
    receipt.render_plain(&ctx, &mut out)?;
    Ok(())
}
