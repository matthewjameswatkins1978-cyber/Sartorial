//! Theme/Preset independence through the Config API: custom application
//! themes compose with every grammar, and an explicit theme is never
//! dropped by a preset change.

use anstyle::AnsiColor;
use sartorial::*;

fn terrorbats_theme() -> Theme {
    Theme::builder("Terrorbats")
        .accent(AnsiColor::Red)
        .heading(AnsiColor::Red)
        .success(AnsiColor::Green)
        .warning(AnsiColor::Yellow)
        .failure(AnsiColor::Red)
        .evidence(AnsiColor::BrightBlack)
        .build()
}

fn fixture() -> SummaryScreen {
    SummaryScreen::new("Mutation campaign", Status::Attention)
        .fact("Generated", "184")
        .fact("Survived", "11")
}

fn plain_text(preset: Preset, theme: Option<Theme>) -> String {
    let mut config = Config::new().with_preset(preset);
    if let Some(theme) = theme {
        config = config.with_theme(theme);
    }
    let ctx = RenderContext::detect()
        .with_config(config)
        .with_target(RenderTarget::Plain)
        .with_width(100);
    fixture().to_plain_string(&ctx).unwrap()
}

#[test]
fn custom_theme_composes_with_every_preset() {
    for preset in [
        Preset::House,
        Preset::BlackTie,
        Preset::Workwear,
        Preset::Studio,
    ] {
        let out = plain_text(preset, Some(terrorbats_theme()));
        assert!(
            out.contains("184"),
            "{preset:?} lost facts with custom theme"
        );
        assert!(out.contains("ATTENTION"), "{preset:?} lost status");
    }
}

#[test]
fn explicit_theme_survives_preset_changes() {
    let config = Config::new()
        .with_preset(Preset::House)
        .with_theme(terrorbats_theme())
        .with_preset(Preset::Workwear)
        .with_preset(Preset::Studio);
    assert_eq!(config.active_theme().name, "Terrorbats");
    assert_eq!(config.active_theme().accent, AnsiColor::Red);
    // Accent mirror stays in sync with the theme.
    assert_eq!(config.accent, AnsiColor::Red);
}

#[test]
fn default_visuals_follow_the_preset_without_explicit_theme() {
    let house = Config::new().with_preset(Preset::House);
    assert_eq!(house.active_theme().name, "House");
    let workwear = Config::new().with_preset(Preset::Workwear);
    assert_eq!(workwear.active_theme().accent, AnsiColor::Yellow);
}

#[test]
fn theme_change_does_not_alter_semantic_facts() {
    let a = plain_text(Preset::Workwear, None);
    let b = plain_text(Preset::Workwear, Some(terrorbats_theme()));
    assert_eq!(a, b);
}

#[test]
fn terminal_outputs_carry_theme_paint() {
    let config = Config::new()
        .with_preset(Preset::Workwear)
        .with_theme(terrorbats_theme())
        .with_color(ColorChoice::Always);
    let ctx = RenderContext::detect().with_config(config).with_width(100);
    let out = fixture().to_human_string(&ctx).unwrap();
    assert!(
        out.contains("\x1b["),
        "expected ANSI paint with forced colour"
    );
    assert!(out.contains("184"));
}

#[test]
fn no_color_and_never_strip_paint() {
    let config = Config::new()
        .with_preset(Preset::Studio)
        .with_theme(terrorbats_theme())
        .with_color(ColorChoice::Never);
    let ctx = RenderContext::detect().with_config(config).with_width(100);
    let out = fixture().to_human_string(&ctx).unwrap();
    assert!(!out.contains('\x1b'));
    assert!(out.contains("184"));
}
