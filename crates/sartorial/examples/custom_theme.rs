//! Custom themes: application identity without new presets.
//!
//! Run: `cargo run -p sartorial --example custom_theme`

use anstyle::AnsiColor;
use sartorial::{Config, Outcome, Preset, RenderContext, RenderHuman, RenderTarget, Status, Theme};

fn omen_theme() -> Theme {
    Theme::builder("Omen")
        .accent(AnsiColor::BrightBlue)
        .heading(AnsiColor::BrightBlue)
        .success(AnsiColor::Green)
        .warning(AnsiColor::Yellow)
        .failure(AnsiColor::Red)
        .build()
}

fn main() -> std::io::Result<()> {
    // One custom theme composes with every grammar; no preset per product.
    for preset in [Preset::House, Preset::Workwear, Preset::Studio] {
        let config = Config::new().with_preset(preset).with_theme(omen_theme());
        let ctx = RenderContext::detect()
            .with_config(config)
            .with_target(RenderTarget::Human)
            .with_width(80);
        let outcome = Outcome::new(Status::Attention, "Omen scan")
            .fact("Signals", "3")
            .fact("Verdict", "watch");
        println!("----- {} + Omen -----", preset.name());
        outcome.render_human(&ctx, &mut std::io::stdout())?;
        println!();
    }
    Ok(())
}
