use sartorial::*;

#[test]
fn test_color_policy_always_never() {
    let always = ColorChoice::Always;
    let never = ColorChoice::Never;

    assert!(always.should_render_color(true));
    assert!(always.should_render_color(false));

    assert!(!never.should_render_color(true));
    assert!(!never.should_render_color(false));
}

#[test]
fn test_color_policy_auto() {
    let auto = ColorChoice::Auto;
    // When not a TTY, auto must not render color
    assert!(!auto.should_render_color(false));
}

#[test]
fn test_ansi_absence_with_never_color() {
    let outcome = Outcome::new(Status::Ready, "Test").fact("Key", "Value");

    let ctx = RenderContext::detect().with_config(Config::new().with_color(ColorChoice::Never));

    let rendered = outcome.to_human_string(&ctx);
    assert!(!rendered.contains("\x1b["));
    assert!(rendered.contains("TEST"));
    assert!(rendered.contains("Key"));
    assert!(rendered.contains("Value"));
}

#[test]
fn test_ansi_presence_with_always_color() {
    let outcome = Outcome::new(Status::Ready, "Test").fact("Key", "Value");

    let ctx = RenderContext::detect().with_config(Config::new().with_color(ColorChoice::Always));

    let rendered = outcome.to_human_string(&ctx);
    // Should contain ANSI escape codes
    assert!(rendered.contains("\x1b["));
}
