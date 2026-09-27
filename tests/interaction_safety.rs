use sartorial::*;

#[test]
fn test_interaction_authority_transitions() {
    // Mode::Off must be non-interactive regardless of stream state
    assert!(!InteractiveMode::Off.is_interactive(true, true));
    assert!(!InteractiveMode::Off.is_interactive(false, false));

    // Mode::On must force interactive
    assert!(InteractiveMode::On.is_interactive(false, false));
    assert!(InteractiveMode::On.is_interactive(true, true));

    // Mode::Auto must require BOTH stdin and stdout to be interactive
    assert!(!InteractiveMode::Auto.is_interactive(true, false));
    assert!(!InteractiveMode::Auto.is_interactive(false, true));
    assert!(!InteractiveMode::Auto.is_interactive(false, false));
    assert!(InteractiveMode::Auto.is_interactive(true, true));
}

#[test]
fn test_confirm_non_interactive_fail_closed_by_default() {
    let confirm = Confirm::new("Delete production database?").with_default(true); // even if default is true!

    let non_interactive_config = Config::new().with_interactive(InteractiveMode::Off);

    // MUST NOT return true or Confirmed merely because non-interactive
    let outcome = confirm
        .prompt_with_config(&non_interactive_config)
        .expect("prompt execution");

    assert_eq!(outcome, ConfirmOutcome::NonInteractiveDenied);
    assert!(!outcome.is_confirmed());
}

#[test]
fn test_confirm_non_interactive_explicit_fallback() {
    let confirm = Confirm::new("Safe read-only check?").with_non_interactive_fallback(true);

    let non_interactive_config = Config::new().with_interactive(InteractiveMode::Off);

    let outcome = confirm
        .prompt_with_config(&non_interactive_config)
        .expect("prompt execution");

    assert_eq!(outcome, ConfirmOutcome::NonInteractiveFallback(true));
    assert!(outcome.is_confirmed());
}

#[test]
fn test_choice_non_interactive_fail_closed_by_default() {
    let items = vec![
        ChoiceItem::new("nuke", "Purge all caches"),
        ChoiceItem::new("keep", "Keep caches"),
    ];

    let mut choice = Choice::new("Select action:", items);
    let non_interactive_config = Config::new().with_interactive(InteractiveMode::Off);

    // MUST NOT silently choose item 0 without explicit fallback
    let outcome = choice
        .select_with_config(&non_interactive_config)
        .expect("choice execution");

    assert_eq!(outcome, ChoiceOutcome::NonInteractiveDenied);
    assert!(outcome.item().is_none());
}

#[test]
fn test_choice_non_interactive_explicit_fallback() {
    let items = vec![
        ChoiceItem::new("staging", "Deploy to Staging"),
        ChoiceItem::new("prod", "Deploy to Production"),
    ];

    let mut choice = Choice::new("Select deployment:", items).with_non_interactive_fallback(0);

    let non_interactive_config = Config::new().with_interactive(InteractiveMode::Off);

    let outcome = choice
        .select_with_config(&non_interactive_config)
        .expect("choice execution");

    match outcome {
        ChoiceOutcome::NonInteractiveFallback(item) => {
            assert_eq!(item.id, "staging");
        }
        _ => panic!("Expected NonInteractiveFallback, got {:?}", outcome),
    }
}

#[test]
fn test_terminal_guard_restoration_lifecycle() {
    // Test that TerminalGuard enters and drops cleanly without panicking
    {
        let guard = sartorial::interaction::TerminalGuard::enter().expect("enter guard");
        // Check active state
        let _ = guard.is_active();
    } // guard drops here; cursor and raw mode restored unconditionally
}

#[test]
fn test_convenience_helpers_return_io_results() {
    let outcome = Outcome::new(Status::Ready, "Test");

    // to_plain returns Result<String, io::Error>
    let plain_res = to_plain(&outcome);
    assert!(plain_res.is_ok());
    let plain_str = plain_res.unwrap();
    assert!(plain_str.contains("TEST"));

    // to_agent_json returns Result<String, serde_json::Error>
    let json_res = to_agent_json(&outcome);
    assert!(json_res.is_ok());
}

#[test]
fn test_interactive_off_prevents_live_animation() {
    let cfg = Config::new()
        .with_interactive(InteractiveMode::Off)
        .with_motion(MotionMode::Always);
    let ctx = RenderContext::detect().with_config(cfg);

    // InteractiveMode::Off must prevent live animation even if is_tty is true and motion is Always
    assert!(!ctx.should_animate(true));
    assert!(!ctx.should_animate(false));
}
