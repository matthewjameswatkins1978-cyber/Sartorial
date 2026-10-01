//! Markdown goldens: summary, table, error, plan, receipt, notice, actions.

use sartorial_core::{
    Action, Capabilities, ErrorModel, Evidence, Notice, Outcome, Plan, Presentable, Preset,
    Receipt, ResolvedStyle, Status, SymbolMode, TableModel, Theme,
};

fn style() -> ResolvedStyle {
    ResolvedStyle::resolve(
        Preset::House,
        &Theme::preset_default(Preset::House),
        Preset::House.default_density(),
        sartorial_core::preset::BorderStyle::Subtle,
        SymbolMode::Ascii,
        false,
    )
}

fn caps() -> Capabilities {
    Capabilities::piped(100)
}

fn md(doc: &sartorial_core::Document) -> String {
    sartorial_core::render::MarkdownRenderer::render_to_string(doc, &style(), &caps()).unwrap()
}

#[test]
fn markdown_summary() {
    let outcome = Outcome::new(Status::Ready, "Environment")
        .with_summary("All good")
        .fact("OS", "Windows")
        .with_action(Action::quit());
    let out = md(&outcome.to_document());
    assert!(out.contains("# ENVIRONMENT"));
    assert!(out.contains("**Status:**"));
    assert!(out.contains("READY"));
    assert!(out.contains("| OS | Windows |"));
    assert!(out.contains("### Actions"));
}

#[test]
fn markdown_table() {
    let table = TableModel::new(vec!["Tool", "Version"])
        .with_title("Tools")
        .with_badge("2 / 2")
        .with_row(["Git", "2.51.0"])
        .with_row(["rg", "14.1.1"]);
    let out = md(&table.to_document());
    assert!(out.contains("## TOOLS"));
    assert!(out.contains("| Tool | Version |"));
    assert!(out.contains("| --- | --- |"));
    assert!(out.contains("| Git | 2.51.0 |"));
}

#[test]
fn markdown_error() {
    let error = ErrorModel::new("Deploy failed")
        .with_why("health check timed out")
        .with_evidence(Evidence::new("connection refused").at("svc:health"))
        .with_action(Action::retry());
    let out = md(&error.to_document());
    assert!(out.contains("## DEPLOY FAILED"));
    assert!(out.contains("**Cause:** health check timed out"));
    assert!(out.contains("`svc:health`"));
    assert!(out.contains("### Next steps"));
}

#[test]
fn markdown_plan() {
    let plan = Plan::new("Migrate")
        .with_description("Move tables")
        .add("users")
        .remove("legacy")
        .warning("Downtime.")
        .consequence("Backfill required.")
        .reversible(false)
        .with_action(Action::open());
    let out = md(&plan.to_document());
    assert!(out.contains("# MIGRATE"));
    assert!(out.contains("- `+` users"));
    assert!(out.contains("- `-` legacy"));
    assert!(out.contains("> [!WARNING]"));
    assert!(out.contains("### Consequences"));
    assert!(out.contains("**Reversible:** No"));
}

#[test]
fn markdown_receipt() {
    let receipt = Receipt::success("Deployed")
        .change("Service", "api v2")
        .unchanged("Config", "same")
        .guidance("Smoke-test now.")
        .with_evidence_handle("ref-1");
    let out = md(&receipt.to_document());
    assert!(out.contains("## DEPLOYED"));
    assert!(out.contains("**Status:**"));
    assert!(out.contains("### Changes"));
    assert!(out.contains("### Unchanged"));
    assert!(out.contains("Smoke-test now."));
    assert!(out.contains("ref-1"));
}

#[test]
fn markdown_notices_become_callouts() {
    for (notice, tag) in [
        (Notice::info("fyi"), "> [!NOTE]"),
        (Notice::tip("try this"), "> [!TIP]"),
        (Notice::warning("careful"), "> [!WARNING]"),
        (Notice::error("broken"), "> [!CAUTION]"),
    ] {
        let out = md(&notice.to_document());
        assert!(out.contains(tag), "missing {tag} for {}", notice.message);
        assert!(out.contains(&notice.message));
    }
}

#[test]
fn markdown_cells_escape_pipes() {
    let table = TableModel::new(vec!["A"]).with_row(["a|b"]);
    let out = md(&table.to_document());
    assert!(out.contains("a\\|b"));
}
