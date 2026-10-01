//! Markdown goldens through the batteries-included API, including screens.

use sartorial::*;

fn ctx() -> RenderContext {
    RenderContext::markdown(Preset::House)
}

#[test]
fn markdown_summary_screen() {
    let screen = SummaryScreen::new("Mutation campaign", Status::Attention)
        .with_subtitle("nightly run")
        .fact("Generated", "184")
        .fact("Survived", "11")
        .notice(Notice::warning(
            "11 surviving mutations require inspection.",
        ))
        .action(Action::details());
    let out = screen.render_markdown(&ctx()).unwrap();
    assert!(out.contains("# MUTATION CAMPAIGN"));
    assert!(out.contains("*nightly run*"));
    assert!(out.contains("**Status:**"));
    assert!(out.contains("| Generated | 184 |"));
    assert!(out.contains("> [!WARNING]"));
    assert!(out.contains("### Actions"));
}

#[test]
fn markdown_table_screen() {
    let table = TableModel::new(vec!["File", "Line"])
        .with_title("Survivors")
        .with_badge("11")
        .with_row(["src/auth/token.rs", "84"]);
    let screen = ListScreen::new("Survivors", table);
    let out = screen.render_markdown(&ctx()).unwrap();
    assert!(out.contains("# SURVIVORS"));
    assert!(out.contains("| File | Line |"));
    assert!(out.contains("src/auth/token.rs"));
}

#[test]
fn markdown_error_and_plan_and_receipt() {
    let error = ErrorModel::new("Token check failed").with_why("expired");
    let out = error.render_markdown(&ctx()).unwrap();
    assert!(out.contains("## TOKEN CHECK FAILED"));
    assert!(out.contains("expired"));

    let plan = Plan::new("Rotate keys").add("auth-key").reversible(true);
    let out = plan.render_markdown(&ctx()).unwrap();
    assert!(out.contains("`+` auth-key"));
    assert!(out.contains("**Reversible:** Yes"));

    let receipt = Receipt::success("Rotated").change("Key", "auth-key");
    let out = receipt.render_markdown(&ctx()).unwrap();
    assert!(out.contains("## ROTATED"));
    assert!(out.contains("| Key | auth-key |"));
}

#[test]
fn markdown_detail_screen() {
    let screen = DetailScreen::new("Token", Status::Failed)
        .fact("File", "src/auth/token.rs")
        .with_evidence(Evidence::new("expiry unchecked").at("src/auth/token.rs:84"))
        .notice(Notice::error("rotate now"));
    let out = screen.render_markdown(&ctx()).unwrap();
    assert!(out.contains("# TOKEN"));
    assert!(out.contains("### EVIDENCE"));
    assert!(out.contains("`src/auth/token.rs:84`"));
    assert!(out.contains("> [!CAUTION]"));
}
