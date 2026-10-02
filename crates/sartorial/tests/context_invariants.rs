//! Regression tests for the v0.3 review hardening pass.

use sartorial::*;

fn rich_human_context() -> RenderContext {
    RenderContext::from_config_with_tty(
        Config::default()
            .with_preset(Preset::Workwear)
            .with_color(ColorChoice::Always)
            .with_symbols(SymbolMode::Unicode)
            .with_width(100),
        true,
        RenderTarget::Human,
    )
}

#[test]
fn target_round_trip_re_resolves_human_state() {
    let human = rich_human_context();
    assert!(human.color_enabled);
    assert_eq!(human.symbols, SymbolMode::Unicode);
    assert_eq!(human.style.title_marker, "» ");

    let restored = human
        .with_target(RenderTarget::Plain)
        .with_target(RenderTarget::Human);

    assert_eq!(restored.target, RenderTarget::Human);
    assert!(restored.color_enabled);
    assert!(restored.style.color_enabled);
    assert!(restored.caps.color_enabled);
    assert!(restored.caps.unicode);
    assert_eq!(restored.symbols, SymbolMode::Unicode);
    assert_eq!(restored.style.title_marker, "» ");
}

#[test]
fn agent_round_trip_re_resolves_human_state() {
    let restored = rich_human_context()
        .with_target(RenderTarget::Agent)
        .with_target(RenderTarget::Human);

    assert_eq!(restored.target, RenderTarget::Human);
    assert!(restored.color_enabled);
    assert!(restored.caps.color_enabled);
    assert_eq!(restored.symbols, SymbolMode::Unicode);
}

#[test]
fn markdown_is_static_even_on_a_tty() {
    let ctx = RenderContext::from_config_with_tty(
        Config::default().with_width(100),
        true,
        RenderTarget::Markdown,
    );

    assert!(!ctx.color_enabled);
    assert!(!ctx.caps.color_enabled);
    assert!(!ctx.caps.motion);
    assert!(!ctx.should_animate(true));
}

#[test]
fn explicit_constructor_uses_a_synthetic_capability_snapshot() {
    let ctx = RenderContext::from_config_with_tty(
        Config::default()
            .with_color(ColorChoice::Always)
            .with_width(91),
        true,
        RenderTarget::Human,
    );

    assert_eq!(ctx.width, 91);
    assert!(ctx.caps.is_tty);
    assert!(ctx.caps.color_enabled);
    assert!(!ctx.caps.hyperlinks);

    let reconfigured = ctx.with_config(
        Config::default()
            .with_color(ColorChoice::Always)
            .with_width(91),
    );
    assert_eq!(reconfigured.width, 91);
    assert!(reconfigured.caps.is_tty);
    assert!(!reconfigured.caps.hyperlinks);
}

#[test]
fn hyperlink_formatting_can_consume_resolved_capability() {
    let linked = Hyperlink::format_resolved(
        "docs",
        "https://example.test",
        true,
        false,
        true,
    );
    assert!(linked.contains("\x1b]8;;https://example.test"));

    let fallback = Hyperlink::format_resolved(
        "docs",
        "https://example.test",
        true,
        false,
        false,
    );
    assert_eq!(fallback, "docs (https://example.test)");
}

#[test]
fn generic_agent_result_path_fails_closed() {
    let ctx = RenderContext::from_config_with_tty(
        Config::default(),
        true,
        RenderTarget::Agent,
    );
    let outcome = Outcome::new(Status::Ready, "Machine result");

    let err = SartorialOutput::print_result(&outcome, &ctx).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);

    // Diagnostics are deliberately suppressed in Agent mode.
    SartorialOutput::print_diagnostic(&Notice::warning("do not leak"), &ctx).unwrap();
}
