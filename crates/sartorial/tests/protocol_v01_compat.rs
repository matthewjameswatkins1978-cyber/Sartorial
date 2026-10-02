#![cfg(feature = "wire")]
//! sartorial.v0.1 protocol compatibility: the visual/framework release
//! must not expand the versioned wire Action vocabulary.
//!
//! Display-only hints were removed from 0.2.0 for exactly this reason:
//! native Rust convenience must not silently change the language-neutral
//! protocol. These tests pin the v0.1 trigger vocabulary in place.

use sartorial::render::SARTORIAL_SCHEMA_VERSION;
use sartorial::semantic::KeyTrigger;
use sartorial::*;

/// The complete v0.1 trigger vocabulary. Every `KeyTrigger` variant must
/// serialize to one of these tags — no more, no less.
const V01_TRIGGERS: &[&str] = &[
    "char", "enter", "esc", "space", "up", "down", "left", "right", "custom",
];

fn trigger_tag(trigger: &KeyTrigger) -> String {
    let v = serde_json::to_value(trigger).unwrap();
    if let Some(s) = v.as_str() {
        s.to_string()
    } else {
        v.as_object()
            .unwrap()
            .keys()
            .next()
            .expect("trigger must be a single-tag value")
            .clone()
    }
}

#[test]
fn trigger_vocabulary_is_exactly_v01() {
    let all = vec![
        KeyTrigger::Char('r'),
        KeyTrigger::Enter,
        KeyTrigger::Esc,
        KeyTrigger::Space,
        KeyTrigger::Up,
        KeyTrigger::Down,
        KeyTrigger::Left,
        KeyTrigger::Right,
        KeyTrigger::Custom("f1".to_string()),
    ];
    let mut tags: Vec<String> = all.iter().map(trigger_tag).collect();
    tags.sort();
    let mut expected: Vec<String> = V01_TRIGGERS.iter().map(|s| s.to_string()).collect();
    expected.sort();
    assert_eq!(tags, expected, "wire trigger vocabulary drifted from v0.1");
}

#[test]
fn v01_action_fixtures_round_trip() {
    // Representative v0.1-era wire JSON for every trigger kind.
    let fixtures = [
        r#"{"id":"retry","label":"Retry","trigger":{"char":"r"}}"#,
        r#"{"id":"open","label":"Open","trigger":"enter"}"#,
        r#"{"id":"back","label":"Back","trigger":"esc"}"#,
        r#"{"id":"accept","label":"Accept","trigger":"space"}"#,
        r#"{"id":"up","label":"Up","trigger":"up"}"#,
        r#"{"id":"down","label":"Down","trigger":"down"}"#,
        r#"{"id":"left","label":"Left","trigger":"left"}"#,
        r#"{"id":"right","label":"Right","trigger":"right"}"#,
        r#"{"id":"help-fn","label":"Help","trigger":{"custom":"f1"}}"#,
    ];
    for json in fixtures {
        let action: Action = serde_json::from_str(json).unwrap();
        let back = serde_json::to_string(&action).unwrap();
        let again: Action = serde_json::from_str(&back).unwrap();
        assert_eq!(action, again, "v0.1 fixture must round-trip: {json}");
        assert!(
            V01_TRIGGERS.contains(&trigger_tag(&action.trigger).as_str()),
            "fixture trigger outside v0.1 vocabulary: {json}"
        );
    }
}

#[test]
fn display_only_none_trigger_is_rejected_on_the_wire() {
    // There is no display-only trigger value in v0.1: a `"none"` trigger
    // must fail to deserialize as an Action.
    let json = r#"{"id":"next","label":"Do the thing","trigger":"none"}"#;
    assert!(
        serde_json::from_str::<Action>(json).is_err(),
        "wire must reject the removed display-only trigger value"
    );
}

#[test]
fn action_envelope_stays_on_schema_v01() {
    use sartorial::protocol::SummaryPayload;

    assert_eq!(SARTORIAL_SCHEMA_VERSION, "sartorial.v0.1");
    let envelope = ProtocolEnvelope::Summary(SummaryPayload {
        schema_version: SARTORIAL_SCHEMA_VERSION.to_string(),
        title: "Demo".to_string(),
        status: Status::Ready,
        subtitle: None,
        facts: vec![],
        notices: vec![],
        actions: vec![
            Action::new('r', "retry", "Retry"),
            Action::open(),
            Action::quit(),
        ],
    });
    let json = serde_json::to_string(&envelope).unwrap();
    assert!(
        json.contains("sartorial.v0.1"),
        "agent JSON must stay on v0.1, got:\n{json}"
    );
    let parsed = ProtocolEnvelope::from_json_str(&json).unwrap();
    parsed.validate_version().unwrap();
    let back = serde_json::to_string(&parsed).unwrap();
    assert_eq!(
        ProtocolEnvelope::from_json_str(&back).unwrap(),
        envelope,
        "action-bearing envelope must round-trip on v0.1"
    );
}
