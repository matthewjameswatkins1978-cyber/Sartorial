use sartorial::*;

#[test]
fn test_error_with_known_cause() {
    let err = ErrorModel::new("THREADMOTH NOT VISIBLE")
        .with_why("Threadmoth is installed, but this process cannot resolve it.")
        .with_evidence(Evidence::new("C:\\Users\\Matmus\\.cargo\\bin\\threadmoth.exe").at("Found:"))
        .with_action(Action::new('r', "recheck", "Recheck"))
        .with_action(Action::details());

    assert!(err.is_cause_known());

    let ctx = RenderContext::plain();
    let rendered = err.to_plain_string(&ctx);

    assert!(rendered.contains("THREADMOTH NOT VISIBLE"));
    assert!(rendered.contains("Threadmoth is installed, but this process cannot resolve it."));
    assert!(rendered.contains("Found:"));
    assert!(rendered.contains("C:\\Users\\Matmus\\.cargo\\bin\\threadmoth.exe"));
    assert!(rendered.contains("[R] Recheck"));
    assert!(rendered.contains("[D] Details"));
}

#[test]
fn test_error_with_unknown_cause_preserves_uncertainty() {
    let err = ErrorModel::new("DATABASE CONNECTION REFUSED")
        .with_evidence(Evidence::new("Connection timeout after 3000ms").at("127.0.0.1:5432"))
        .with_action(Action::retry());

    assert!(!err.is_cause_known());

    let ctx = RenderContext::plain();
    let rendered = err.to_plain_string(&ctx);

    assert!(rendered.contains("DATABASE CONNECTION REFUSED"));
    // Must preserve uncertainty and NOT manufacture a reason
    assert!(rendered.contains("Cause undetermined"));
    assert!(rendered.contains("127.0.0.1:5432"));
    assert!(rendered.contains("[R] Retry"));
}
