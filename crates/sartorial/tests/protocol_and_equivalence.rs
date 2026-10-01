#![cfg(feature = "wire")]
use sartorial::components::error::ErrorView;
use sartorial::components::table::TableView;
use sartorial::protocol::{
    ChoicePayload, ConfirmPayload, ErrorPayload, PlanPayload, ProgressEvent, ProtocolEnvelope,
    ReceiptPayload, SummaryPayload, TablePayload,
};
use sartorial::render::{RenderPlain, SARTORIAL_SCHEMA_VERSION};
use sartorial::screens::summary::SummaryScreen;
use sartorial::semantic::action::Action;
use sartorial::semantic::error::ErrorModel;
use sartorial::semantic::evidence::Evidence;
use sartorial::semantic::fact::Fact;
use sartorial::semantic::notice::Notice;
use sartorial::semantic::plan::{Plan, PlanChange};
use sartorial::semantic::receipt::Receipt;
use sartorial::semantic::status::Status;
use sartorial::semantic::table::TableModel;
use sartorial::RenderContext;

#[test]
fn test_summary_native_and_protocol_equivalence() {
    let ctx = RenderContext::plain().with_width(80);

    // 1. Native Rust Screen
    let native = SummaryScreen::new("System Health", Status::Ready)
        .with_subtitle("All nodes verified")
        .fact("Uptime", "99.98%")
        .fact("Active Workers", "16")
        .notice(Notice::info("Cache warm-up finished"))
        .action(Action::new('s', "status", "Inspect running jobs"));

    let native_out = native.to_plain_string(&ctx).unwrap();

    // 2. Protocol Envelope
    let envelope = ProtocolEnvelope::Summary(SummaryPayload {
        schema_version: SARTORIAL_SCHEMA_VERSION.to_string(),
        title: "System Health".to_string(),
        status: Status::Ready,
        subtitle: Some("All nodes verified".to_string()),
        facts: vec![
            Fact::new("Uptime", "99.98%"),
            Fact::new("Active Workers", "16"),
        ],
        notices: vec![Notice::info("Cache warm-up finished")],
        actions: vec![Action::new('s', "status", "Inspect running jobs")],
    });

    let protocol_out = envelope.to_plain_string(&ctx).unwrap();

    assert_eq!(
        native_out, protocol_out,
        "Native Summary and Protocol Summary must render identically"
    );
}

#[test]
fn test_table_native_and_protocol_equivalence() {
    let ctx = RenderContext::plain().with_width(80);

    // Native Table
    let mut native_model = TableModel::new(vec!["Service", "Region", "Latency"])
        .with_title("Cluster Status")
        .with_badge("prod");
    native_model.add_row(["auth", "eu-west-1", "12ms"]);
    native_model.add_row(["db", "eu-west-1", "2ms"]);
    let native_view = TableView::new(native_model);
    let native_out = native_view.to_plain_string(&ctx).unwrap();

    // Protocol Envelope
    let envelope = ProtocolEnvelope::Table(TablePayload {
        schema_version: SARTORIAL_SCHEMA_VERSION.to_string(),
        title: Some("Cluster Status".to_string()),
        badge: Some("prod".to_string()),
        headers: vec!["Service".into(), "Region".into(), "Latency".into()],
        rows: vec![
            vec!["auth".into(), "eu-west-1".into(), "12ms".into()],
            vec!["db".into(), "eu-west-1".into(), "2ms".into()],
        ],
    });
    let protocol_out = envelope.to_plain_string(&ctx).unwrap();

    assert_eq!(
        native_out, protocol_out,
        "Native Table and Protocol Table must render identically"
    );
}

#[test]
fn test_error_native_and_protocol_equivalence() {
    let ctx = RenderContext::plain().with_width(80);

    // Native Error
    let native = ErrorModel::new("Database connection refused")
        .with_why("Connection timed out after 30s")
        .with_evidence(Evidence::new("Host: db.internal.net:5432"))
        .with_action(Action::new('p', "ping-db", "Test raw TCP socket"));
    let native_view = ErrorView::new(native);
    let native_out = native_view.to_plain_string(&ctx).unwrap();

    // Protocol Envelope
    let envelope = ProtocolEnvelope::Error(ErrorPayload {
        schema_version: SARTORIAL_SCHEMA_VERSION.to_string(),
        what: "Database connection refused".to_string(),
        why: Some("Connection timed out after 30s".to_string()),
        evidence: vec![Evidence::new("Host: db.internal.net:5432")],
        actions: vec![Action::new('p', "ping-db", "Test raw TCP socket")],
    });
    let protocol_out = envelope.to_plain_string(&ctx).unwrap();

    assert_eq!(
        native_out, protocol_out,
        "Native Error and Protocol Error must render identically"
    );
}

#[test]
fn test_plan_native_and_protocol_equivalence() {
    let ctx = RenderContext::plain().with_width(80);

    // Native Plan
    let native = Plan::new("Apply Database Migrations")
        .with_description("Upgrade schema to v4")
        .add_change(
            PlanChange::add("table `audit_logs`").with_detail("Creates new audit tracking table"),
        )
        .add_change(PlanChange::modify("column `users.email`").with_detail("Add unique index"))
        .consequence("Existing transactions will briefly acquire a write lock")
        .warning("Requires backup before execution")
        .reversible(true)
        .with_action(Action::new('m', "db-migrate", "Run migrations"));
    let native_out = native.to_plain_string(&ctx).unwrap();

    // Protocol Envelope
    let envelope = ProtocolEnvelope::Plan(PlanPayload {
        schema_version: SARTORIAL_SCHEMA_VERSION.to_string(),
        title: "Apply Database Migrations".to_string(),
        description: Some("Upgrade schema to v4".to_string()),
        changes: vec![
            PlanChange::add("table `audit_logs`").with_detail("Creates new audit tracking table"),
            PlanChange::modify("column `users.email`").with_detail("Add unique index"),
        ],
        consequences: vec!["Existing transactions will briefly acquire a write lock".into()],
        warnings: vec!["Requires backup before execution".into()],
        reversible: Some(true),
        actions: vec![Action::new('m', "db-migrate", "Run migrations")],
    });
    let protocol_out = envelope.to_plain_string(&ctx).unwrap();

    assert_eq!(
        native_out, protocol_out,
        "Native Plan and Protocol Plan must render identically"
    );
}

#[test]
fn test_receipt_native_and_protocol_equivalence() {
    let ctx = RenderContext::plain().with_width(80);

    // Native Receipt
    let native = Receipt::success("Deployment Complete")
        .with_status(Status::Ready)
        .change("Version", "1.4.0 -> 1.5.0")
        .change("Replicas", "12 healthy")
        .unchanged("Database", "v4.2 schema")
        .guidance("Monitor APM dashboard for 15 minutes")
        .warning(Notice::warning("High memory observed on node 3"))
        .with_evidence_handle("s3://deploys/run-8924.log")
        .with_action(Action::new('c', "curl-check", "Check endpoint"));
    let native_out = native.to_plain_string(&ctx).unwrap();

    // Protocol Envelope
    let envelope = ProtocolEnvelope::Receipt(ReceiptPayload {
        schema_version: SARTORIAL_SCHEMA_VERSION.to_string(),
        title: "Deployment Complete".to_string(),
        status: Status::Ready,
        changes: vec![
            Fact::new("Version", "1.4.0 -> 1.5.0"),
            Fact::new("Replicas", "12 healthy"),
        ],
        unchanged: vec![Fact::new("Database", "v4.2 schema")],
        guidance: Some("Monitor APM dashboard for 15 minutes".to_string()),
        warnings: vec![Notice::warning("High memory observed on node 3")],
        evidence_handle: Some("s3://deploys/run-8924.log".to_string()),
        actions: vec![Action::new('c', "curl-check", "Check endpoint")],
    });
    let protocol_out = envelope.to_plain_string(&ctx).unwrap();

    assert_eq!(
        native_out, protocol_out,
        "Native Receipt and Protocol Receipt must render identically"
    );
}

#[test]
fn test_json_foreign_client_deserialization() {
    let json_input = r#"{
        "type": "summary",
        "schema_version": "sartorial.v0.1",
        "title": "Foreign Client Job",
        "status": "ready",
        "subtitle": "Executed via Python client",
        "facts": [
            {"name": "Runtime", "value": "Python 3.12"},
            {"name": "Exit", "value": "0"}
        ],
        "notices": [],
        "actions": []
    }"#;

    let envelope: ProtocolEnvelope = serde_json::from_str(json_input).expect("Valid JSON envelope");
    let ctx = RenderContext::plain().with_width(80);
    let output = envelope.to_plain_string(&ctx).expect("Render success");

    assert!(output.contains("READY"));
    assert!(output.contains("FOREIGN CLIENT JOB"));
    assert!(output.contains("Executed via Python client"));
    assert!(output.contains("Runtime  Python 3.12"));
}

#[test]
fn test_jsonl_progress_stream_deserialization() {
    let lines = [
        r#"{"type":"progress.start","id":"scan","activity":"Scanning repo","total":100,"unit":"files"}"#,
        r#"{"type":"progress.update","id":"scan","current":42,"subtask":"crates/core"}"#,
        r#"{"type":"progress.finish","id":"scan","status":"ready"}"#,
    ];

    let events: Vec<ProgressEvent> = lines
        .iter()
        .map(|l| serde_json::from_str(l).expect("Valid progress event"))
        .collect();

    assert_eq!(events.len(), 3);
    match &events[0] {
        ProgressEvent::Start {
            id,
            activity,
            total,
            unit,
            ..
        } => {
            assert_eq!(id, "scan");
            assert_eq!(activity, "Scanning repo");
            assert_eq!(*total, Some(100));
            assert_eq!(unit.as_deref(), Some("files"));
        }
        _ => panic!("Expected start event"),
    }

    match &events[1] {
        ProgressEvent::Update {
            id,
            current,
            subtask,
            ..
        } => {
            assert_eq!(id, "scan");
            assert_eq!(*current, Some(42));
            assert_eq!(subtask.as_deref(), Some("crates/core"));
        }
        _ => panic!("Expected update event"),
    }

    match &events[2] {
        ProgressEvent::Finish { id, status, .. } => {
            assert_eq!(id, "scan");
            assert_eq!(*status, Status::Ready);
        }
        _ => panic!("Expected finish event"),
    }
}

#[test]
fn test_confirm_and_choice_protocol_payloads() {
    let confirm_json = r#"{
        "type": "confirm",
        "schema_version": "sartorial.v0.1",
        "prompt": "Apply changes?",
        "default": true,
        "non_interactive_fallback": false
    }"#;

    let confirm_envelope: ProtocolEnvelope = serde_json::from_str(confirm_json).unwrap();
    match confirm_envelope {
        ProtocolEnvelope::Confirm(ConfirmPayload {
            prompt,
            default,
            non_interactive_fallback,
            ..
        }) => {
            assert_eq!(prompt, "Apply changes?");
            assert!(default);
            assert_eq!(non_interactive_fallback, Some(false));
        }
        _ => panic!("Expected confirm envelope"),
    }

    let choice_json = r#"{
        "type": "choice",
        "schema_version": "sartorial.v0.1",
        "prompt": "Select target environment:",
        "items": [
            {"id": "dev", "label": "Development", "description": "Local sandbox"},
            {"id": "prod", "label": "Production", "description": "Live traffic"}
        ],
        "default_index": 0,
        "non_interactive_fallback": 0
    }"#;

    let choice_envelope: ProtocolEnvelope = serde_json::from_str(choice_json).unwrap();
    match choice_envelope {
        ProtocolEnvelope::Choice(ChoicePayload {
            prompt,
            items,
            default_index,
            non_interactive_fallback,
            ..
        }) => {
            assert_eq!(prompt, "Select target environment:");
            assert_eq!(items.len(), 2);
            assert_eq!(default_index, 0);
            assert_eq!(non_interactive_fallback, Some(0));
        }
        _ => panic!("Expected choice envelope"),
    }
}

#[test]
fn test_render_schema_version_validation() {
    // 1. Missing schema_version -> Rejected
    let missing_json = r#"{
        "type": "summary",
        "title": "Missing Schema",
        "status": "ready"
    }"#;
    let res_missing = ProtocolEnvelope::from_json_str(missing_json);
    assert!(
        res_missing.is_err(),
        "Missing schema_version must be rejected on render envelopes"
    );

    // 2. Correct schema_version -> Accepted
    let correct_json = r#"{
        "type": "summary",
        "schema_version": "sartorial.v0.1",
        "title": "Valid Schema",
        "status": "ready"
    }"#;
    let res_correct = ProtocolEnvelope::from_json_str(correct_json);
    assert!(res_correct.is_ok(), "sartorial.v0.1 must be accepted");

    // 3. Unknown schema_version -> Rejected
    let unknown_json = r#"{
        "type": "summary",
        "schema_version": "sartorial.v9.9",
        "title": "Future Schema",
        "status": "ready"
    }"#;
    let res_unknown = ProtocolEnvelope::from_json_str(unknown_json);
    assert!(
        res_unknown.is_err(),
        "Unsupported schema_version must be rejected"
    );
    if let Err(sartorial::protocol::ProtocolError::UnsupportedVersion { expected, actual }) =
        res_unknown
    {
        assert_eq!(expected, "sartorial.v0.1");
        assert_eq!(actual, "sartorial.v9.9");
    } else {
        panic!("Expected UnsupportedVersion error");
    }
}
