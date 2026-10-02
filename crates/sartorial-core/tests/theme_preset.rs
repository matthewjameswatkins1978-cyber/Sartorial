//! Preset vs Theme independence: grammar and identity compose freely,
//! and neither may alter semantic facts.

use sartorial_core::{
    BorderStyle, Capabilities, Density, Outcome, Presentable, Preset, ResolvedStyle, Status,
    SymbolMode, Theme,
};

fn terrorbats_theme() -> Theme {
    Theme::builder("Terrorbats")
        .accent(anstyle::AnsiColor::Red)
        .success(anstyle::AnsiColor::Green)
        .warning(anstyle::AnsiColor::Yellow)
        .failure(anstyle::AnsiColor::Red)
        .evidence(anstyle::AnsiColor::BrightBlack)
        .build()
}

fn fixture() -> Outcome {
    Outcome::new(Status::Attention, "Mutation campaign")
        .fact("Generated", "184")
        .fact("Survived", "11")
}

fn resolve(preset: Preset, theme: &Theme) -> ResolvedStyle {
    ResolvedStyle::resolve(
        preset,
        theme,
        preset.default_density(),
        BorderStyle::Subtle,
        SymbolMode::Ascii,
        false,
    )
}

fn plain_facts(preset: Preset, theme: &Theme) -> String {
    let doc = fixture().to_document();
    let style = resolve(preset, theme);
    sartorial_core::render::PlainRenderer::render_to_string(&doc, &style, &Capabilities::piped(100))
        .unwrap()
}

#[test]
fn all_presets_preserve_semantic_facts() {
    let theme = terrorbats_theme();
    for preset in [
        Preset::House,
        Preset::BlackTie,
        Preset::Workwear,
        Preset::Studio,
    ] {
        let out = plain_facts(preset, &theme);
        assert!(out.contains("184"), "{preset:?} lost a fact");
        assert!(out.contains("11"), "{preset:?} lost a fact");
        assert!(out.contains("ATTENTION"), "{preset:?} lost status");
    }
}

#[test]
fn changing_theme_does_not_alter_facts() {
    let before = plain_facts(Preset::Workwear, &Theme::preset_default(Preset::Workwear));
    let after = plain_facts(Preset::Workwear, &terrorbats_theme());
    // Same grammar: fact lines identical (theme only affects terminal paint).
    assert_eq!(before, after);
}

#[test]
fn changing_preset_does_not_alter_facts() {
    let theme = terrorbats_theme();
    let workwear = plain_facts(Preset::Workwear, &theme);
    let studio = plain_facts(Preset::Studio, &theme);
    for fact in ["184", "11", "ATTENTION"] {
        assert!(workwear.contains(fact));
        assert!(studio.contains(fact));
    }
    // But personalities stay recognisably distinct.
    assert_ne!(workwear, studio);
}

#[test]
fn explicit_theme_survives_preset_changes() {
    let theme = terrorbats_theme();
    for preset in [
        Preset::House,
        Preset::BlackTie,
        Preset::Workwear,
        Preset::Studio,
    ] {
        let style = resolve(preset, &theme);
        assert_eq!(style.accent, anstyle::AnsiColor::Red);
        assert_eq!(style.theme_name, "Terrorbats");
    }
}

#[test]
fn resolution_consumes_concrete_decisions() {
    // Colour enabled + Unicode: status paint flows through the theme.
    let style = ResolvedStyle::resolve(
        Preset::Studio,
        &terrorbats_theme(),
        Density::Roomy,
        BorderStyle::Subtle,
        SymbolMode::Unicode,
        true,
    );
    let painted = style.status_style(Status::Ready);
    let plain = format!("{painted}");
    assert!(plain.contains("\x1b["), "expected ANSI paint when enabled");

    // Same theme, no colour: no colour codes anywhere (bold effects may remain).
    let nocolor = ResolvedStyle::resolve(
        Preset::Studio,
        &terrorbats_theme(),
        Density::Roomy,
        BorderStyle::Subtle,
        SymbolMode::Ascii,
        false,
    );
    let bare = format!("{}", nocolor.status_style(Status::Ready));
    assert!(
        !bare.contains("[32m") && !bare.contains("[31m"),
        "colour leaked: {bare:?}"
    );
}
