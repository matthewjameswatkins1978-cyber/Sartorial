use sartorial::*;
use serde_json::Value;

#[test]
fn test_agent_json_structure_and_no_ansi() {
    let outcome = Outcome::new(Status::Failed, "Clippy Check")
        .with_summary("1 warning found")
        .with_evidence(
            Evidence::new("unused import `Path`")
                .at("src/repo.rs:184")
                .with_handle("clippy:err-01"),
        )
        .with_warning(Notice::warning("Dead code detected"))
        .with_action(Action::details())
        .with_action(Action::retry());

    let json_str = outcome.render_agent_json(true).expect("valid JSON");

    // Strictly ensure no ANSI formatting leaked into machine output
    assert!(!json_str.contains("\x1b"));
    assert!(!json_str.contains("\u{1b}"));

    let val: Value = serde_json::from_str(&json_str).expect("parse JSON");

    // Validate stable schema
    assert_eq!(val["status"], "failed");
    assert_eq!(val["title"], "Clippy Check");
    assert_eq!(val["summary"], "1 warning found");

    let evidence = &val["evidence"][0];
    assert_eq!(evidence["summary"], "unused import `Path`");
    assert_eq!(evidence["location"], "src/repo.rs:184");
    assert_eq!(evidence["handle"], "clippy:err-01");

    let actions = val["actions"].as_array().expect("actions array");
    assert_eq!(actions.len(), 2);
    assert_eq!(actions[0]["id"], "details");
    assert_eq!(actions[1]["id"], "retry");
}

#[test]
fn test_progress_state_agent_json() {
    let mut pb = ProgressBar::new("Checking repository")
        .with_subtask("cargo test")
        .with_progress(9, 11, "crates")
        .with_elapsed(47);
    pb.finish_with_status(Status::Ready);

    let json_str = pb.state().render_agent_json(false).expect("valid JSON");
    assert!(!json_str.contains("\x1b"));

    let val: Value = serde_json::from_str(&json_str).expect("parse JSON");
    assert_eq!(val["task"], "Checking repository");
    assert_eq!(val["subtask"], "cargo test");
    assert_eq!(val["current"], 9);
    assert_eq!(val["total"], 11);
    assert_eq!(val["unit"], "crates");
    assert_eq!(val["elapsed_secs"], 47);
    assert_eq!(val["status"], "ready");
}
