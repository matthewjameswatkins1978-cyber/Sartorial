//! Preset personalities: the same truth dressed four ways.
//!
//! Run: `cargo run -p sartorial --example styles`

use sartorial::{Outcome, Preset, RenderContext, RenderPlain, Status};

fn main() -> std::io::Result<()> {
    for preset in [
        Preset::House,
        Preset::BlackTie,
        Preset::Workwear,
        Preset::Studio,
    ] {
        let ctx = RenderContext::plain_preset(preset).with_width(80);
        let outcome = Outcome::new(Status::Attention, "Deploy")
            .with_summary("One service needs attention")
            .fact("Service", "api")
            .fact("Region", "eu-west");
        println!("================ {} ================", preset.name());
        outcome.render_plain(&ctx, &mut std::io::stdout())?;
        println!();
    }
    Ok(())
}
