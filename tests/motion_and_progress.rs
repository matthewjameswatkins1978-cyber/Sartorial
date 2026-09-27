use sartorial::*;
use serde_json::Value;

#[test]
fn test_unknown_total_never_fabricates_percentage() {
    let p = ProgressBar::activity("Checking repository")
        .with_subtask("cargo test")
        .with_elapsed(14);

    assert_eq!(p.state().mode, ProgressMode::Activity);
    assert_eq!(p.state().percent, None);
    assert_eq!(p.state().total, None);
    assert_eq!(p.state().elapsed_secs, Some(14));

    let json_str = p.state().to_agent_json(false).unwrap();
    let val: Value = serde_json::from_str(&json_str).unwrap();
    assert_eq!(val["mode"], "activity");
    assert!(val.get("percent").is_none());
    assert_eq!(val["elapsed_secs"], 14);
}

#[test]
fn test_count_produces_correct_progress_and_derives_percent() {
    let p = ProgressBar::count("Scanning files", 38, 60);

    assert_eq!(p.state().mode, ProgressMode::Count);
    assert_eq!(p.state().current, Some(38));
    assert_eq!(p.state().total, Some(60));
    assert_eq!(p.state().percent, Some(63)); // (38 * 100) / 60 = 63%

    let ctx = RenderContext::plain();
    let plain = p.to_plain_string(&ctx).unwrap();
    assert!(plain.contains("Scanning files"));
    assert!(plain.contains("(38/60)"));
    assert!(plain.contains("63%"));
}

#[test]
fn test_countdown_reflects_supplied_time() {
    let p = ProgressBar::countdown("Waiting for service", 17);

    assert_eq!(p.state().mode, ProgressMode::Countdown);
    assert_eq!(p.state().countdown_secs, Some(17));

    let ctx = RenderContext::plain();
    let plain = p.to_plain_string(&ctx).unwrap();
    assert!(plain.contains("Waiting for service"));
    assert!(plain.contains("remaining: 17s"));
}

#[test]
fn test_no_spinner_frames_in_json() {
    let p = ProgressBar::activity("Checking repository").with_elapsed(22);
    let json_str = p.state().to_agent_json(true).unwrap();

    assert!(!json_str.contains("◐"));
    assert!(!json_str.contains("⠋"));
    assert!(!json_str.contains("\x1b"));

    let val: Value = serde_json::from_str(&json_str).unwrap();
    assert_eq!(val["schema_version"], "sartorial.v0.1");
    assert_eq!(val["task"], "Checking repository");
}

#[test]
fn test_motion_mode_policy() {
    let auto = MotionMode::Auto;
    // When not a TTY or in plain/agent, should not animate
    assert!(!auto.should_animate(false, false, false, false));
    assert!(!auto.should_animate(true, true, false, false));
    assert!(!auto.should_animate(true, false, true, false));
    assert!(!auto.should_animate(true, false, false, true)); // reduced motion
    assert!(auto.should_animate(true, false, false, false));

    let never = MotionMode::Never;
    assert!(!never.should_animate(true, false, false, false));
}
