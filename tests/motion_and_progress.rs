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
    let p = ProgressBar::count("Scanning files", 38, 60).unwrap();

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

#[test]
fn test_zero_total_semantics_0_0_and_1_0_never_derive_100_percent() {
    // 0/0 case: completed 0 of 0 items
    let p0 = ProgressBar::count("Zero total tasks", 0, 0).unwrap();
    assert_eq!(p0.state().mode, ProgressMode::Count);
    assert_eq!(p0.state().current, Some(0));
    assert_eq!(p0.state().total, Some(0));
    assert_eq!(p0.state().percent, None); // NEVER Some(100)
    assert_eq!(p0.state().derived_percent(), None);

    let ctx_plain = RenderContext::plain();
    let plain0 = p0.to_plain_string(&ctx_plain).unwrap();
    assert!(plain0.contains("(0/0)"));
    assert!(!plain0.contains("100%"));

    let cfg_ww = Config::new().with_preset(Preset::Workwear);
    let ctx_ww = RenderContext::detect().with_config(cfg_ww);
    let human0 = p0.to_human_string(&ctx_ww).unwrap();
    assert!(human0.contains("[0/0]"));
    assert!(!human0.contains("100%"));

    let json0 = p0.state().to_agent_json(false).unwrap();
    let val0: Value = serde_json::from_str(&json0).unwrap();
    assert_eq!(val0["current"], 0);
    assert_eq!(val0["total"], 0);
    assert!(val0.get("percent").is_none());

    assert!(matches!(
        ProgressBar::count("Overflow zero tasks", 1, 0),
        Err(ProgressError::CurrentExceedsTotal {
            current: 1,
            total: 0
        })
    ));
}

#[test]
fn test_contradictory_progress_input_enforces_single_authority() {
    let mut p = ProgressBar::count("Processing batch", 0, 100).unwrap();

    // 1. Current exceeding total is rejected
    let res_exceed = p.apply_update(Some(105), None, None, None, None);
    assert!(matches!(
        res_exceed,
        Err(ProgressError::CurrentExceedsTotal {
            current: 105,
            total: 100
        })
    ));

    // 2. Percent exceeding 100 is rejected
    let res_pct_range = p.apply_update(Some(50), Some(120), None, None, None);
    assert!(matches!(
        res_pct_range,
        Err(ProgressError::PercentOutOfRange { percent: 120 })
    ));

    // 3. Contradictory percent (50 / 100 with 12%) is rejected
    let res_contra = p.apply_update(Some(50), Some(12), None, None, None);
    assert!(matches!(
        res_contra,
        Err(ProgressError::PercentContradiction {
            derived: Some(50),
            explicit: 12
        })
    ));

    // 4. Agreeing percent (50 / 100 with 50%) is accepted
    let res_agree = p.apply_update(Some(50), Some(50), None, None, None);
    assert!(res_agree.is_ok());
    assert_eq!(p.state().current, Some(50));
    assert_eq!(p.state().percent, Some(50));
    assert_eq!(p.state().derived_percent(), Some(50));

    // 5. Subsequent contradictory percent update alone is rejected
    let res_pct_alone = p.apply_update(None, Some(12), None, None, None);
    assert!(matches!(
        res_pct_alone,
        Err(ProgressError::PercentContradiction {
            derived: Some(50),
            explicit: 12
        })
    ));

    // 6. Prohibit states such as "50 / 100  12%" in presentation:
    // Even if a raw state were maliciously crafted, derived_percent is single authority
    let mut hostile_state = ProgressState::count("Hostile task", 50, 100).unwrap();
    hostile_state.percent = Some(12); // forcibly setting raw field
    assert_eq!(hostile_state.derived_percent(), Some(50)); // single authority overrides!

    let cfg_ww = Config::new().with_preset(Preset::Workwear);
    let ctx_ww = RenderContext::detect().with_config(cfg_ww);
    let mut hostile_pb = ProgressBar::count("Hostile task", 50, 100).unwrap();
    hostile_pb.state_mut().percent = Some(12);
    let human_out = hostile_pb.to_human_string(&ctx_ww).unwrap();
    assert!(human_out.contains("[50/100] 50%"));
    assert!(!human_out.contains("12%"));
}

#[test]
fn test_native_construction_rejects_invalid_counts_and_zero_total_stays_indeterminate() {
    assert!(matches!(
        ProgressState::count("too far", 101, 100),
        Err(ProgressError::CurrentExceedsTotal {
            current: 101,
            total: 100
        })
    ));
    assert!(matches!(
        ProgressBar::rate("too far", 2, 1, "items", "1/s"),
        Err(ProgressError::CurrentExceedsTotal {
            current: 2,
            total: 1
        })
    ));
    assert!(matches!(
        ProgressBar::activity("empty").with_progress(1, 0, "items"),
        Err(ProgressError::CurrentExceedsTotal {
            current: 1,
            total: 0
        })
    ));

    let zero = ProgressState::count("empty", 0, 0).unwrap();
    assert_eq!(zero.percent, None);
    assert_eq!(zero.derived_percent(), None);
    assert!(matches!(
        zero.clone().with_progress(1, 0, "items"),
        Err(ProgressError::CurrentExceedsTotal { .. })
    ));
    let invalid_json = r#"{"task":"bad","mode":"count","current":2,"total":1,"status":"running"}"#;
    assert!(serde_json::from_str::<ProgressState>(invalid_json).is_err());
}

#[test]
fn test_invalid_mutated_native_state_cannot_render_or_serialize_as_valid() {
    let mut bar = ProgressBar::count("scan", 0, 10).unwrap();
    bar.state_mut().current = Some(11);
    assert_eq!(
        bar.to_plain_string(&RenderContext::plain())
            .unwrap_err()
            .kind(),
        std::io::ErrorKind::InvalidInput
    );
    assert_eq!(
        bar.to_human_string(&RenderContext::plain())
            .unwrap_err()
            .kind(),
        std::io::ErrorKind::InvalidInput
    );
    assert!(bar.state().to_agent_json(false).is_err());

    let mut total_update = ProgressState::count("scan", 4, 5).unwrap();
    assert!(matches!(
        total_update.update_total(3),
        Err(ProgressError::CurrentExceedsTotal {
            current: 4,
            total: 3
        })
    ));
    assert_eq!(total_update.total, Some(5));
}
