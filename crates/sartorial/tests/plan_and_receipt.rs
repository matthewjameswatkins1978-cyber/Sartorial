use sartorial::*;
#[cfg(feature = "wire")]
use serde_json::Value;

#[test]
fn test_plan_presentation_and_agent_json() {
    let plan = Plan::new("PATH REPAIR")
        .with_description("Proposed changes")
        .add("C:\\Users\\Matmus\\.cargo\\bin")
        .modify("C:\\Program Files\\Git\\bin")
        .warning("User PATH will be updated; system PATH remains untouched.")
        .consequence("Existing processes will not inherit this update.")
        .reversible(true)
        .with_action(Action::new('a', "apply", "Apply"))
        .with_action(Action::quit().with_label("Cancel"));

    // Plain output test
    let ctx = RenderContext::plain();
    let plain = plan.to_plain_string(&ctx).unwrap();
    assert!(plain.contains("PATH REPAIR"));
    assert!(plain.contains("+ C:\\Users\\Matmus\\.cargo\\bin"));
    assert!(plain.contains("~ C:\\Program Files\\Git\\bin"));
    assert!(plain.contains("! User PATH will be updated"));
    assert!(plain.contains("Existing processes will not inherit"));
    assert!(plain.contains("Reversible: Yes"));
    assert!(plain.contains("[A] Apply"));
    assert!(plain.contains("[Q] Cancel"));

    // Agent JSON test (wire surface)
    #[cfg(feature = "wire")]
    {
        let json_str = plan.to_agent_json(true).unwrap();
        assert!(!json_str.contains("\x1b"));

        let val: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(val["schema_version"], "sartorial.v0.1");
        assert_eq!(val["title"], "PATH REPAIR");
        assert_eq!(val["changes"].as_array().unwrap().len(), 2);
        assert_eq!(val["changes"][0]["kind"], "add");
        assert_eq!(
            val["changes"][0]["target"],
            "C:\\Users\\Matmus\\.cargo\\bin"
        );
        assert_eq!(val["changes"][1]["kind"], "modify");
        assert_eq!(val["next_actions"], serde_json::json!(["apply", "quit"]));
    }
}

#[test]
fn test_receipt_presentation_and_agent_json() {
    let receipt = Receipt::success("PATH UPDATED")
        .change("Added", "1 directory")
        .change("Removed", "0 entries")
        .unchanged("System PATH", "unchanged")
        .guidance("Open a new shell for the changes to take effect.")
        .with_evidence_handle("receipt-ref-1082")
        .with_action(Action::open().with_label("Open shell"))
        .with_action(Action::quit());

    let ctx = RenderContext::plain();
    let plain = receipt.to_plain_string(&ctx).unwrap();
    assert!(plain.contains("PATH UPDATED"));
    assert!(plain.contains("Added"));
    assert!(plain.contains("1 directory"));
    assert!(plain.contains("Unchanged:"));
    assert!(plain.contains("System PATH"));
    assert!(plain.contains("Open a new shell"));
    assert!(plain.contains("receipt-ref-1082"));
    assert!(plain.contains("[Enter] Open shell"));

    #[cfg(feature = "wire")]
    {
        let json_str = receipt.to_agent_json(true).unwrap();
        assert!(!json_str.contains("\x1b"));

        let val: Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(val["schema_version"], "sartorial.v0.1");
        assert_eq!(val["title"], "PATH UPDATED");
        assert_eq!(val["status"], "ready");
        assert_eq!(val["changes"].as_array().unwrap().len(), 2);
        assert_eq!(val["unchanged"].as_array().unwrap().len(), 1);
        assert_eq!(val["evidence_handle"], "receipt-ref-1082");
        assert_eq!(val["next_actions"], serde_json::json!(["open", "quit"]));
    }
}
