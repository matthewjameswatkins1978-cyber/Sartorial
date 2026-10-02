#![cfg(feature = "wire")]
use sartorial::render::SARTORIAL_SCHEMA_VERSION;
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

    let json_str = outcome.to_agent_json(true).expect("valid JSON");

    // Strictly ensure no ANSI formatting leaked into machine output
    assert!(!json_str.contains("\x1b"));
    assert!(!json_str.contains("\u{1b}"));

    let val: Value = serde_json::from_str(&json_str).expect("parse JSON");

    // Validate stable schema and canonical version
    assert_eq!(val["schema_version"], SARTORIAL_SCHEMA_VERSION);
    assert_eq!(val["status"], "failed");
    assert_eq!(val["title"], "Clippy Check");
    assert_eq!(val["summary"], "1 warning found");

    let evidence = &val["evidence"][0];
    assert_eq!(evidence["summary"], "unused import `Path`");
    assert_eq!(evidence["location"], "src/repo.rs:184");
    assert_eq!(evidence["handle"], "clippy:err-01");

    // Reconciled next_actions array of IDs conforming to BL contract
    let next_actions = val["next_actions"].as_array().expect("next_actions array");
    assert_eq!(next_actions.len(), 2);
    assert_eq!(next_actions[0], "details");
    assert_eq!(next_actions[1], "retry");

    let warnings = val["warnings"].as_array().expect("warnings array");
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0], "Dead code detected");
}

#[test]
fn test_error_agent_json_schema() {
    let err = ErrorModel::new("COMPILER FAILURE")
        .with_why("Type mismatch at line 42")
        .with_evidence(Evidence::new("expected u32, found String").at("src/main.rs:42"))
        .with_action(Action::retry())
        .with_action(Action::details());

    let json_str = err.to_agent_json(true).expect("valid JSON");
    assert!(!json_str.contains("\x1b"));

    let val: Value = serde_json::from_str(&json_str).expect("parse JSON");
    assert_eq!(val["schema_version"], SARTORIAL_SCHEMA_VERSION);
    assert_eq!(val["status"], "failed");
    assert_eq!(val["what"], "COMPILER FAILURE");
    assert_eq!(val["why"], "Type mismatch at line 42");

    let next_actions = val["next_actions"].as_array().expect("next_actions");
    assert_eq!(next_actions[0], "retry");
    assert_eq!(next_actions[1], "details");
}

#[test]
fn test_progress_state_agent_json() {
    let mut pb = ProgressBar::new("Checking repository")
        .with_subtask("cargo test")
        .with_progress(9, 11, "crates")
        .unwrap()
        .with_elapsed(47);
    pb.finish_with_status(Status::Ready);

    let json_str = pb.state().to_agent_json(false).expect("valid JSON");
    assert!(!json_str.contains("\x1b"));

    let val: Value = serde_json::from_str(&json_str).expect("parse JSON");
    assert_eq!(val["schema_version"], SARTORIAL_SCHEMA_VERSION);
    assert_eq!(val["task"], "Checking repository");
    assert_eq!(val["subtask"], "cargo test");
    assert_eq!(val["current"], 9);
    assert_eq!(val["total"], 11);
    assert_eq!(val["unit"], "crates");
    assert_eq!(val["elapsed_secs"], 47);
    assert_eq!(val["status"], "ready");
}
