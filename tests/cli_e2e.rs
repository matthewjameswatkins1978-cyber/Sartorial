use sartorial::exit::ExitCode;
use sartorial::protocol::{ChoiceResult, ConfirmResult};
use std::io::Write;
use std::process::{Command, Stdio};

fn sartorial_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sartorial"))
}

#[test]
fn test_1_json_summary_via_stdin() {
    let payload = r#"{
        "type": "summary",
        "schema_version": "sartorial.v0.1",
        "title": "E2E Test Run",
        "status": "ready",
        "subtitle": "Execution completed",
        "facts": [
            {"key": "Target", "value": "x86_64-pc-windows-msvc"}
        ],
        "notices": [],
        "actions": []
    }"#;

    let mut child = sartorial_bin()
        .arg("render")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn sartorial");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(ExitCode::Success.as_i32()));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.to_uppercase().contains("E2E TEST RUN"));
    assert!(stdout.contains("Target"));
    assert!(stdout.contains("x86_64-pc-windows-msvc"));
}

#[test]
fn test_2_unsupported_schema_version_rejected_with_exit_2() {
    let payload = r#"{
        "type": "summary",
        "schema_version": "sartorial.v9.9",
        "title": "Future Payload",
        "status": "ready"
    }"#;

    let mut child = sartorial_bin()
        .arg("render")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn sartorial");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(ExitCode::UsageError.as_i32()));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "unsupported protocol schema_version: got sartorial.v9.9, expected sartorial.v0.1"
        ),
        "Expected error message, got: {stderr}"
    );
}

#[test]
fn test_3_choice_with_quotes_backslashes_tabs_unicode() {
    let complex_label = "Item \"with quotes\", \\backslashes\\, \t tabs, and 🚀 Unicode";
    let arg_item = format!("id_complex:{complex_label}");

    let output = sartorial_bin()
        .arg("choice")
        .arg("Select configuration:")
        .arg("--item")
        .arg(&arg_item)
        .arg("--fallback")
        .arg("0")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to run choice");

    assert_eq!(output.status.code(), Some(ExitCode::Success.as_i32()));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: ChoiceResult = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("Failed to parse ChoiceResult JSON: {e}\nOutput was: {stdout}"));

    assert_eq!(parsed.status, "non_interactive_fallback");
    assert_eq!(parsed.id.as_deref(), Some("id_complex"));
    assert_eq!(parsed.label.as_deref(), Some(complex_label));
}

#[test]
fn test_4_confirm_non_interactive_fails_closed_with_exit_4() {
    let output = sartorial_bin()
        .arg("confirm")
        .arg("Perform destructive operation?")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to run confirm");

    assert_eq!(output.status.code(), Some(ExitCode::Failed.as_i32()));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: ConfirmResult = serde_json::from_str(stdout.trim()).expect("Valid JSON");
    assert_eq!(parsed.status, "non_interactive_denied");
    assert_eq!(parsed.confirmed, None);
}

#[test]
fn test_5_canonical_exit_code_authority() {
    // 1. Confirm with fallback false -> ExitCode::Declined (1)
    let out_declined = sartorial_bin()
        .arg("confirm")
        .arg("Question?")
        .arg("--fallback")
        .arg("false")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(
        out_declined.status.code(),
        Some(ExitCode::Declined.as_i32())
    );

    // 2. Confirm with fallback true -> ExitCode::Success (0)
    let out_success = sartorial_bin()
        .arg("confirm")
        .arg("Question?")
        .arg("--fallback")
        .arg("true")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(out_success.status.code(), Some(ExitCode::Success.as_i32()));

    // 3. Choice non-interactive without fallback -> ExitCode::Failed (4)
    let out_choice_fail = sartorial_bin()
        .arg("choice")
        .arg("Pick:")
        .arg("--item")
        .arg("1:Option 1")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(
        out_choice_fail.status.code(),
        Some(ExitCode::Failed.as_i32())
    );

    // 4. Invalid arguments -> ExitCode::UsageError (2)
    let out_usage = sartorial_bin().arg("--invalid-flag-xyz").output().unwrap();
    assert_eq!(out_usage.status.code(), Some(ExitCode::UsageError.as_i32()));
}

#[test]
fn test_6_stream_non_tty_bounded_output_no_spam() {
    let stream_data = "\
{\"type\":\"progress.start\",\"id\":\"dl\",\"activity\":\"Downloading payload\",\"total\":100}\n\
{\"type\":\"progress.update\",\"id\":\"dl\",\"current\":10}\n\
{\"type\":\"progress.update\",\"id\":\"dl\",\"current\":20}\n\
{\"type\":\"progress.update\",\"id\":\"dl\",\"current\":30}\n\
{\"type\":\"progress.update\",\"id\":\"dl\",\"current\":50}\n\
{\"type\":\"progress.update\",\"id\":\"dl\",\"current\":80}\n\
{\"type\":\"progress.update\",\"id\":\"dl\",\"current\":100}\n\
{\"type\":\"progress.finish\",\"id\":\"dl\",\"status\":\"ready\"}\n";

    let mut child = sartorial_bin()
        .arg("stream")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stream_data.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(ExitCode::Success.as_i32()));

    let stderr = String::from_utf8_lossy(&output.stderr);
    let lines: Vec<&str> = stderr.lines().filter(|l| !l.trim().is_empty()).collect();

    // In non-TTY, start emits 1 line, updates emit 0 lines (no spam!), finish emits 1 final line
    assert_eq!(
        lines.len(),
        2,
        "Non-TTY progress must be bounded to start and finish lines, got {}: {:?}",
        lines.len(),
        lines
    );
    assert!(lines[0].contains("Starting: Downloading payload"));
    assert!(lines[1].contains("Downloading payload"));
}

#[test]
fn test_7_stream_json_mode_pure_stdout_zero_stderr() {
    let stream_data = "\
{\"type\":\"progress.start\",\"id\":\"task1\",\"activity\":\"Processing queue\",\"total\":50}\n\
{\"type\":\"progress.update\",\"id\":\"task1\",\"current\":25}\n\
{\"type\":\"progress.finish\",\"id\":\"task1\",\"status\":\"ready\"}\n";

    let mut child = sartorial_bin()
        .arg("stream")
        .arg("--json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stream_data.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(ExitCode::Success.as_i32()));

    // Stderr must be completely empty (0 bytes)
    assert!(
        output.stderr.is_empty(),
        "Expected 0 stderr bytes, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Stdout must be clean parseable JSONL with no ANSI codes
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("\x1b["));

    let lines: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 3);
    for line in lines {
        let v: serde_json::Value =
            serde_json::from_str(line).expect("Each line must be valid JSON");
        assert!(v.get("type").is_some());
        assert_eq!(v.get("schema_version").unwrap(), "sartorial.v0.1");
    }
}

#[test]
fn test_8_motion_never_disables_animation() {
    let stream_data = "\
{\"type\":\"progress.start\",\"id\":\"task2\",\"activity\":\"Analyzing dependencies\"}\n\
{\"type\":\"progress.finish\",\"id\":\"task2\",\"status\":\"ready\"}\n";

    let mut child = sartorial_bin()
        .arg("stream")
        .arg("--motion")
        .arg("never")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stream_data.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(ExitCode::Success.as_i32()));
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Disabling motion prevents dynamic cursor repositioning / clearing codes
    assert!(stderr.contains("Starting: Analyzing dependencies"));
}

#[test]
fn test_9_all_four_presets_preserve_semantic_content() {
    let payload = r#"{
        "type": "summary",
        "schema_version": "sartorial.v0.1",
        "title": "System Audit",
        "status": "ready",
        "facts": [
            {"key": "Cluster", "value": "production-eu-1"},
            {"key": "Nodes", "value": "128"}
        ],
        "notices": [],
        "actions": [
            {"id": "inspect", "label": "Inspect cluster", "trigger": {"char": "i"}}
        ]
    }"#;

    let presets = ["house", "black-tie", "workwear", "studio"];
    for preset in presets {
        let mut child = sartorial_bin()
            .arg("render")
            .arg("--preset")
            .arg(preset)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(payload.as_bytes())
            .unwrap();

        let output = child.wait_with_output().unwrap();
        assert_eq!(
            output.status.code(),
            Some(ExitCode::Success.as_i32()),
            "Failed on preset {preset}"
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.to_uppercase().contains("SYSTEM AUDIT"),
            "Missing title in {preset}"
        );
        assert!(
            stdout.contains("Cluster"),
            "Missing key 'Cluster' in {preset}"
        );
        assert!(
            stdout.contains("production-eu-1"),
            "Missing value in {preset}"
        );
        assert!(
            stdout.contains("Inspect cluster"),
            "Missing action label in {preset}"
        );
    }
}

#[test]
fn test_10_preset_visual_differences_beyond_accent_colour() {
    let payload = r#"{
        "type": "summary",
        "schema_version": "sartorial.v0.1",
        "title": "Preset Verification",
        "status": "ready",
        "facts": [
            {"key": "Environment", "value": "Staging"}
        ],
        "notices": [],
        "actions": [
            {"id": "run", "label": "Execute", "trigger": {"char": "x"}}
        ]
    }"#;

    // Render plain/text with BlackTie
    let mut child_bt = sartorial_bin()
        .arg("render")
        .arg("--preset")
        .arg("black-tie")
        .arg("--plain")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child_bt
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();
    let out_bt = child_bt.wait_with_output().unwrap();
    let stdout_bt = String::from_utf8_lossy(&out_bt.stdout);

    // BlackTie action bar uses parentheses "(X) Execute"
    assert!(
        stdout_bt.contains("(X) Execute"),
        "BlackTie must format key delimiters as ( ), got:\n{stdout_bt}"
    );

    // Render plain/text with Workwear
    let mut child_ww = sartorial_bin()
        .arg("render")
        .arg("--preset")
        .arg("workwear")
        .arg("--plain")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child_ww
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();
    let out_ww = child_ww.wait_with_output().unwrap();
    let stdout_ww = String::from_utf8_lossy(&out_ww.stdout);

    // Workwear action bar uses brackets "[X] Execute"
    assert!(
        stdout_ww.contains("[X] Execute"),
        "Workwear must format key delimiters as [ ], got:\n{stdout_ww}"
    );
}

#[test]
fn test_11_workwear_stream_progress_state_coherence() {
    let stream_data = "\
{\"type\":\"progress.start\",\"id\":\"scan\",\"activity\":\"Scanning repository\",\"total\":100}\n\
{\"type\":\"progress.update\",\"id\":\"scan\",\"current\":50}\n\
{\"type\":\"progress.update\",\"id\":\"scan\",\"current\":100}\n\
{\"type\":\"progress.finish\",\"id\":\"scan\",\"status\":\"ready\"}\n";

    let mut child = sartorial_bin()
        .arg("stream")
        .arg("--preset")
        .arg("workwear")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stream_data.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(ExitCode::Success.as_i32()));
    let stderr = String::from_utf8_lossy(&output.stderr);

    // In Workwear, [100/100] must be accompanied by 100%, and NEVER 0%
    assert!(
        stderr.contains("[100/100] 100%"),
        "Workwear must produce [100/100] 100% on completion, got:\n{stderr}"
    );
    assert!(
        !stderr.contains("[100/100] 0%"),
        "Workwear must NEVER produce [100/100] 0%!"
    );
}

#[test]
fn test_12_percent_only_streaming() {
    let stream_data = "\
{\"type\":\"progress.start\",\"id\":\"comp\",\"activity\":\"Compiling crate\"}\n\
{\"type\":\"progress.update\",\"id\":\"comp\",\"percent\":42}\n\
{\"type\":\"progress.update\",\"id\":\"comp\",\"percent\":100}\n\
{\"type\":\"progress.finish\",\"id\":\"comp\",\"status\":\"ready\"}\n";

    let mut child = sartorial_bin()
        .arg("stream")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stream_data.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(ExitCode::Success.as_i32()));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("100%"),
        "Percent-only stream must render final percentage, got:\n{stderr}"
    );
}

#[test]
fn test_13_motion_always_on_non_tty_stays_static_no_frame_spam() {
    let stream_data = "\
{\"type\":\"progress.start\",\"id\":\"task3\",\"activity\":\"Processing data\",\"total\":100}\n\
{\"type\":\"progress.update\",\"id\":\"task3\",\"current\":20}\n\
{\"type\":\"progress.update\",\"id\":\"task3\",\"current\":40}\n\
{\"type\":\"progress.update\",\"id\":\"task3\",\"current\":60}\n\
{\"type\":\"progress.update\",\"id\":\"task3\",\"current\":80}\n\
{\"type\":\"progress.update\",\"id\":\"task3\",\"current\":100}\n\
{\"type\":\"progress.finish\",\"id\":\"task3\",\"status\":\"ready\"}\n";

    let mut child = sartorial_bin()
        .arg("stream")
        .arg("--motion")
        .arg("always")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stream_data.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(ExitCode::Success.as_i32()));
    let stderr = String::from_utf8_lossy(&output.stderr);
    let lines: Vec<&str> = stderr.lines().filter(|l| !l.trim().is_empty()).collect();

    // Bounded static output on non-TTY even with --motion always: exactly 2 lines
    assert_eq!(
        lines.len(),
        2,
        "--motion always on non-TTY must not emit frame spam, got {} lines:\n{:?}",
        lines.len(),
        lines
    );
    assert!(lines[0].contains("Starting: Processing data"));
    assert!(lines[1].contains("Processing data"));
}

#[test]
fn test_14_missing_schema_version_rejected_with_exit_2() {
    let payload = r#"{
        "type": "summary",
        "title": "Missing Schema Version",
        "status": "ready"
    }"#;

    let mut child = sartorial_bin()
        .arg("render")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(ExitCode::UsageError.as_i32()));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("schema_version"),
        "Error message must mention missing schema_version, got: {stderr}"
    );
}
