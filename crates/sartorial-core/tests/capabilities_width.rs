//! Capabilities, widths, determinism, plain invariants, progress honesty.

use sartorial_core::{
    Capabilities, ColorPolicy, Outcome, Presentable, Preset, ProgressError, ProgressState,
    ResolvedStyle, Status, SymbolMode, Theme,
};

fn style() -> ResolvedStyle {
    ResolvedStyle::resolve(
        Preset::House,
        &Theme::preset_default(Preset::House),
        Preset::House.default_density(),
        sartorial_core::preset::BorderStyle::Subtle,
        SymbolMode::Ascii,
        false,
    )
}

#[test]
fn widths_render_without_panic_or_overflow() {
    let outcome = Outcome::new(Status::Ready, "Widths").fact(
        "A very long fact label here",
        "and a very long value that must survive",
    );
    let doc = outcome.to_document();
    let style = style();
    for width in [40, 60, 100, 120] {
        let caps = Capabilities::piped(width);
        let plain =
            sartorial_core::render::PlainRenderer::render_to_string(&doc, &style, &caps).unwrap();
        assert!(plain.contains("Widths".to_uppercase().as_str()));
        for line in plain.lines() {
            // ASCII-safe plain output stays narrow-device friendly: no
            // absurdly long lines from unbounded padding.
            assert!(line.len() <= width.max(80) + 40, "line overflow at {width}");
        }
    }
}

#[test]
fn narrow_terminals_stack_facts() {
    let outcome = Outcome::new(Status::Ready, "T").fact("Label", "Value");
    let doc = outcome.to_document();
    let style = style();
    let narrow = sartorial_core::render::PlainRenderer::render_to_string(
        &doc,
        &style,
        &Capabilities::piped(40),
    )
    .unwrap();
    assert!(narrow.contains("Label:\n  Value"));
}

#[test]
fn explicit_capabilities_are_deterministic() {
    let outcome = Outcome::new(Status::Ready, "Determinism").fact("K", "V");
    let doc = outcome.to_document();
    let style = style();
    let a = sartorial_core::render::PlainRenderer::render_to_string(
        &doc,
        &style,
        &Capabilities::explicit(
            100,
            true,
            ColorPolicy::Never,
            false,
            false,
            false,
            false,
            false,
        ),
    )
    .unwrap();
    let b = sartorial_core::render::PlainRenderer::render_to_string(
        &doc,
        &style,
        &Capabilities::explicit(
            100,
            true,
            ColorPolicy::Never,
            false,
            false,
            false,
            false,
            false,
        ),
    )
    .unwrap();
    assert_eq!(a, b);
}

#[test]
fn core_never_sniffs_the_environment() {
    // Core takes NO_COLOR as an explicit snapshot input; setting the real
    // variable must not change rendering (detection lives in full Sartorial).
    let outcome = Outcome::new(Status::Ready, "Env").fact("K", "V");
    let doc = outcome.to_document();
    let style = style();
    let render = || {
        sartorial_core::render::PlainRenderer::render_to_string(
            &doc,
            &style,
            &Capabilities::piped(80),
        )
        .unwrap()
    };
    let before = render();
    unsafe { std::env::set_var("NO_COLOR", "1") };
    let during = render();
    unsafe { std::env::remove_var("NO_COLOR") };
    assert_eq!(before, during);
}

#[test]
fn plain_output_is_pipe_safe() {
    let outcome = Outcome::new(Status::Failed, "Pipe")
        .fact("K", "V")
        .with_warning(sartorial_core::Notice::error("bad"));
    let doc = outcome.to_document();
    let out = sartorial_core::render::PlainRenderer::render_to_string(
        &doc,
        &style(),
        &Capabilities::piped(100),
    )
    .unwrap();
    assert!(!out.contains('\x1b'), "ANSI leaked into plain output");
    assert!(out.is_ascii(), "plain output must be ASCII-safe");
    assert!(out.contains("[X] FAILED"));
}

#[test]
fn progress_honesty_unknown_work_has_no_percentage() {
    let activity = ProgressState::activity("Scanning");
    assert_eq!(activity.derived_percent(), None);
    let doc = activity.to_document();
    let out = sartorial_core::render::PlainRenderer::render_to_string(
        &doc,
        &style(),
        &Capabilities::piped(80),
    )
    .unwrap();
    assert!(
        !out.contains('%'),
        "fabricated percentage in activity snapshot"
    );
}

#[test]
fn progress_honesty_zero_total_stays_indeterminate() {
    let zero = ProgressState::count("Empty", 0, 0).unwrap();
    assert_eq!(zero.derived_percent(), None);
    assert!(matches!(
        ProgressState::count("Bad", 1, 0),
        Err(ProgressError::CurrentExceedsTotal { .. })
    ));
    assert!(matches!(
        ProgressState::count("Bad", 5, 3),
        Err(ProgressError::CurrentExceedsTotal { .. })
    ));
}

#[test]
fn progress_honesty_contradictions_rejected() {
    let mut state = ProgressState::count("Batch", 0, 100).unwrap();
    assert!(state.update_percent(50).is_err());
    assert!(state.update_percent(101).is_err());
    assert!(state.update_current(101).is_err());
}
