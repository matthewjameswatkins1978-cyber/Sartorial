//! Terrorbats dogfood: one semantic result dressed four ways.
//!
//! The Terrorbats theme lives in this example, not in the framework.
//! The same semantic data feeds Workwear + Terrorbats, Studio +
//! Terrorbats, Plain, and Markdown with no special-case renderer logic.
//!
//! Run: `cargo run -p sartorial --example terrorbats`

use anstyle::AnsiColor;
use sartorial::{
    Action, Config, Evidence, Notice, Preset, RenderContext, RenderHuman, RenderMarkdown,
    RenderPlain, RenderTarget, Status, SummaryScreen, TableModel, Theme,
};

fn terrorbats_theme() -> Theme {
    Theme::builder("Terrorbats")
        .accent(AnsiColor::Red)
        .heading(AnsiColor::Red)
        .success(AnsiColor::Green)
        .warning(AnsiColor::Yellow)
        .failure(AnsiColor::Red)
        .muted(AnsiColor::BrightBlack)
        .evidence(AnsiColor::BrightBlack)
        .build()
}

fn campaign() -> SummaryScreen {
    let mut survivors = TableModel::new(vec!["File", "Mutation", "Status"]).with_title("Survivors");
    survivors.add_row([
        "src/auth/token.rs:84",
        "remove expiry validation",
        "attention",
    ]);
    survivors.add_row([
        "src/billing/charge.rs:12",
        "skip zero-amount guard",
        "attention",
    ]);

    SummaryScreen::new("THE TERRORBATS TESTING FRAMEWORK", Status::Attention)
        .with_subtitle("mutation campaign report")
        .fact("Generated", "184")
        .fact("Killed", "173")
        .fact("Survived", "11")
        .with_table(survivors)
        .notice(Notice::warning(
            "11 surviving mutations require inspection.",
        ))
        .action(Action::details())
        .action(Action::quit())
}

fn survivor_evidence() -> Evidence {
    Evidence::new("remove expiry validation")
        .at("src/auth/token.rs:84")
        .with_handle("mut:tb-0084")
}

fn main() -> std::io::Result<()> {
    let _ = survivor_evidence();
    let screen = campaign();

    for preset in [Preset::Workwear, Preset::Studio] {
        let config = Config::new()
            .with_preset(preset)
            .with_theme(terrorbats_theme());
        let ctx = RenderContext::detect()
            .with_config(config)
            .with_target(RenderTarget::Human)
            .with_width(100);
        println!("===== {:?} + Terrorbats =====", preset);
        screen.render_human(&ctx, &mut std::io::stdout())?;
        println!();
    }

    let plain_ctx = RenderContext::plain_preset(Preset::Workwear).with_width(100);
    println!("===== Plain =====");
    screen.render_plain(&plain_ctx, &mut std::io::stdout())?;
    println!();

    let md_ctx = RenderContext::markdown(Preset::House);
    println!("===== Markdown =====");
    println!("{}", screen.render_markdown(&md_ctx)?);
    Ok(())
}
