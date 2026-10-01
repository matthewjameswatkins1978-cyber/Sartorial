use sartorial::*;

#[test]
fn test_status_glyphs_and_labels() {
    assert_eq!(Status::Ready.as_str(), "ready");
    assert_eq!(Status::Ready.display_label(), "READY");
    assert_eq!(Status::Ready.unicode_glyph(), "✓");
    assert_eq!(Status::Ready.ascii_glyph(), "[OK]");

    assert_eq!(Status::Attention.as_str(), "attention");
    assert_eq!(Status::Attention.display_label(), "ATTENTION");
    assert_eq!(Status::Attention.unicode_glyph(), "!");
    assert_eq!(Status::Attention.ascii_glyph(), "[!]");

    assert_eq!(Status::Failed.as_str(), "failed");
    assert_eq!(Status::Failed.display_label(), "FAILED");
    assert_eq!(Status::Failed.unicode_glyph(), "×");
    assert_eq!(Status::Failed.ascii_glyph(), "[X]");
}

#[test]
fn test_same_semantic_truth_across_three_targets() {
    let outcome = Outcome::new(Status::Ready, "Environment Check")
        .with_summary("All prerequisites are satisfied")
        .fact("OS", "Windows 11")
        .fact("Rust", "1.98.1")
        .with_action(Action::open())
        .with_action(Action::quit());

    // 1. Human view
    let human_ctx =
        RenderContext::detect().with_config(Config::new().with_color(ColorChoice::Never));
    let human_str = outcome.to_human_string(&human_ctx).unwrap();
    assert!(human_str.contains("ENVIRONMENT CHECK"));
    assert!(human_str.contains("Windows 11"));
    assert!(human_str.contains("[Enter] Open"));

    // 2. Plain view
    let plain_ctx = RenderContext::plain();
    let plain_str = outcome.to_plain_string(&plain_ctx).unwrap();
    assert!(!plain_str.contains("\x1b["));
    assert!(plain_str.contains("ENVIRONMENT CHECK"));
    assert!(plain_str.contains("Windows 11"));
    assert!(plain_str.contains("[Enter] Open"));

    // 3. Agent JSON view
    let json_str = outcome.to_agent_json(true).expect("valid JSON");
    assert!(!json_str.contains("\x1b["));

    let val: serde_json::Value = serde_json::from_str(&json_str).expect("deserialize Outcome JSON");
    assert_eq!(val["schema_version"], "sartorial.v0.1");
    assert_eq!(val["status"], "ready");
    assert_eq!(val["title"], "Environment Check");
    assert_eq!(val["facts"].as_array().unwrap().len(), 2);
    assert_eq!(val["next_actions"], serde_json::json!(["open", "quit"]));
}

#[test]
fn test_plain_rendering_is_pipe_safe() {
    let mut table = TableModel::new(vec!["Tool", "Version", "Status"]);
    table.add_row(["Git", "2.51.0", "ready"]);
    table.add_row(["ripgrep", "14.1.1", "ready"]);

    let view = TableView::new(table);
    let ctx = RenderContext::plain();
    let plain = view.to_plain_string(&ctx).unwrap();

    // Absolutely no ANSI escape sequences
    assert!(!plain.contains("\x1b"));
    assert!(plain.contains("Git"));
    assert!(plain.contains("2.51.0"));
    assert!(plain.contains("ripgrep"));
}
