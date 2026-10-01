//! Document equivalence: converting a semantic object to a Document and
//! rendering it preserves meaning across terminal, plain, and Markdown.

use sartorial_core::{
    Action, Block, Capabilities, Document, ErrorModel, Evidence, Fact, Notice, Outcome, Plan,
    Presentable, Preset, Receipt, ResolvedStyle, Status, SymbolMode, TableModel, Theme,
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

fn caps(width: usize) -> Capabilities {
    Capabilities::piped(width)
}

fn terrorbats_like() -> Outcome {
    Outcome::new(Status::Attention, "Mutation campaign")
        .fact("Generated", "184")
        .fact("Killed", "173")
        .fact("Survived", "11")
        .with_evidence(Evidence::new("remove expiry validation").at("src/auth/token.rs:84"))
        .with_warning(Notice::warning(
            "11 surviving mutations require inspection.",
        ))
        .with_action(Action::details())
}

#[test]
fn outcome_converts_to_expected_blocks() {
    let doc = terrorbats_like().to_document();
    assert!(matches!(doc.blocks[0], Block::Title { .. }));
    assert!(matches!(
        doc.blocks[1],
        Block::StatusSection { ref label, status } if label == "Status" && status == Status::Attention
    ));
    assert!(matches!(doc.blocks[2], Block::Facts { .. }));
    assert!(matches!(doc.blocks[3], Block::Evidence { .. }));
    assert!(matches!(doc.blocks[4], Block::Notices { .. }));
    assert!(matches!(doc.blocks[5], Block::Actions { .. }));
}

#[test]
fn meaning_survives_all_three_renderers() {
    let outcome = terrorbats_like();
    let doc = outcome.to_document();
    let style = style();

    let terminal =
        sartorial_core::render::TerminalRenderer::render_to_string(&doc, &style, &caps(100))
            .unwrap();
    let plain =
        sartorial_core::render::PlainRenderer::render_to_string(&doc, &style, &caps(100)).unwrap();
    let markdown =
        sartorial_core::render::MarkdownRenderer::render_to_string(&doc, &style, &caps(100))
            .unwrap();

    for rendered in [&terminal, &plain, &markdown] {
        for fact in [
            "184",
            "173",
            "11",
            "src/auth/token.rs:84",
            "remove expiry validation",
        ] {
            assert!(rendered.contains(fact), "renderer lost fact {fact:?}");
        }
        assert!(
            rendered.contains("ATTENTION"),
            "renderer lost status meaning"
        );
        assert!(
            rendered.contains("11 surviving mutations require inspection."),
            "renderer lost notice"
        );
    }

    // Plain carries no ANSI; Markdown carries structure, not ANSI.
    assert!(!plain.contains('\x1b'));
    assert!(!markdown.contains('\x1b'));
    assert!(markdown.contains('#'));
}

#[test]
fn plan_receipt_error_table_round_trip_through_document() {
    let style = style();
    let caps = caps(100);

    let plan = Plan::new("Deploy")
        .add("service/api")
        .warning("Downtime expected.")
        .reversible(true);
    let plan_md = sartorial_core::render::MarkdownRenderer::render_to_string(
        &plan.to_document(),
        &style,
        &caps,
    )
    .unwrap();
    assert!(plan_md.contains("service/api"));
    assert!(plan_md.contains("Downtime expected."));

    let receipt = Receipt::success("Deployed").change("Service", "api v2");
    let receipt_md = sartorial_core::render::MarkdownRenderer::render_to_string(
        &receipt.to_document(),
        &style,
        &caps,
    )
    .unwrap();
    assert!(receipt_md.contains("api v2"));

    let error = ErrorModel::new("Deploy failed").with_why("health check timed out");
    let error_md = sartorial_core::render::MarkdownRenderer::render_to_string(
        &error.to_document(),
        &style,
        &caps,
    )
    .unwrap();
    assert!(error_md.contains("health check timed out"));

    let table = TableModel::new(vec!["A", "B"]).with_row(["1", "2"]);
    let table_md = sartorial_core::render::MarkdownRenderer::render_to_string(
        &table.to_document(),
        &style,
        &caps,
    )
    .unwrap();
    assert!(table_md.contains("| A | B |"));
    assert!(table_md.contains("| 1 | 2 |"));
}

#[test]
fn document_identity_and_fact_helpers() {
    let facts = vec![Fact::new("K", "V")];
    let doc = facts.to_document();
    assert_eq!(doc.len(), 1);
    assert!(!doc.is_empty());
    assert_eq!(Document::new().to_document(), Document::new());
}
