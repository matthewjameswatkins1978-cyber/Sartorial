//! Machine-schema boundary: applications own their JSON schemas.
//! Sartorial's wire surface carries presentation only, and the small
//! serialized Document format round-trips without involving app schemas.

#![cfg(feature = "wire")]

use sartorial::render::document::{document_from_json, document_to_json, DOCUMENT_SCHEMA_VERSION};
use sartorial::*;

/// An application-owned machine schema. It never becomes a Sartorial
/// schema; Sartorial only presents projections of it.
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct TerrorbatsReport {
    generated: u32,
    killed: u32,
    survived: u32,
}

#[test]
fn application_schema_stays_application_owned() {
    let report = TerrorbatsReport {
        generated: 184,
        killed: 173,
        survived: 11,
    };
    // The app serializes itself with plain serde_json; no Sartorial types.
    let json = serde_json::to_string(&report).unwrap();
    assert!(json.contains("\"survived\":11"));
    assert!(!json.contains("sartorial"));

    // Presentation is a separate projection into Sartorial semantics.
    let screen = SummaryScreen::new("Mutation campaign", Status::Attention)
        .fact("Generated", report.generated.to_string())
        .fact("Survived", report.survived.to_string());
    let ctx = RenderContext::plain().with_width(100);
    let plain = screen.to_plain_string(&ctx).unwrap();
    assert!(plain.contains("184"));
    assert!(plain.contains("11"));
}

#[test]
fn document_wire_format_round_trips() {
    let screen = SummaryScreen::new("Mutation campaign", Status::Attention)
        .fact("Generated", "184")
        .notice(Notice::warning("inspect survivors"));
    let doc = screen.to_document();
    let json = document_to_json(&doc, false).unwrap();
    assert!(!json.contains('\x1b'));
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["schema"], DOCUMENT_SCHEMA_VERSION);
    assert!(value.get("document").is_some());
    let back = document_from_json(&json).unwrap();
    assert_eq!(doc, back);

    // The wire document still renders identically.
    let ctx = RenderContext::plain().with_width(100);
    let before = screen.to_plain_string(&ctx).unwrap();
    let after_md = back.render_markdown(&ctx).unwrap();
    assert!(after_md.contains("184"));
    assert!(after_md.contains("inspect survivors"));
    assert!(before.contains("184"));
}

#[test]
fn agent_json_carries_no_presentation_noise() {
    let outcome = Outcome::new(Status::Failed, "Deploy")
        .fact("Host", "prod-1")
        .with_action(Action::retry());
    let json = outcome.to_agent_json(false).unwrap();
    assert!(!json.contains('\x1b'));
    assert!(!json.contains('✓'));
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(val["status"], "failed");
    assert_eq!(val["schema_version"], "sartorial.v0.1");
}

#[test]
fn document_wire_rejects_unknown_schema_versions() {
    let doc = SummaryScreen::new("Mutation campaign", Status::Ready).to_document();
    let json = document_to_json(&doc, false).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
    value["schema"] = serde_json::Value::String("sartorial.document.v999".to_string());
    let bad = serde_json::to_string(&value).unwrap();
    let err = document_from_json(&bad).unwrap_err();
    assert!(err
        .to_string()
        .contains("unsupported Sartorial Document schema"));
}
