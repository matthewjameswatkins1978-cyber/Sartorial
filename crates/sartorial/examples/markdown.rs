//! Markdown as a first-class target: GitHub-ready reports from one truth.
//!
//! Run: `cargo run -p sartorial --example markdown`

use sartorial::{
    Action, Evidence, Notice, Plan, Preset, Receipt, RenderContext, RenderMarkdown, Status,
    SummaryScreen, TableModel,
};

fn main() -> std::io::Result<()> {
    let ctx = RenderContext::markdown(Preset::House);

    let screen = SummaryScreen::new("Release 0.3.0", Status::Ready)
        .fact("Crates", "2")
        .fact("Tests", "all green")
        .notice(Notice::tip("See the workspace layout in the README."))
        .action(Action::quit());
    println!("{}", screen.render_markdown(&ctx)?);

    let mut table = TableModel::new(vec!["Crate", "Role"]).with_title("Workspace");
    table.add_row(["sartorial-core", "deterministic engine"]);
    table.add_row(["sartorial", "terminal toolkit"]);
    println!("{}", table.render_markdown(&ctx)?);

    let plan = Plan::new("Publish")
        .add("sartorial-core 0.3.0")
        .reversible(false);
    println!("{}", plan.render_markdown(&ctx)?);

    let receipt = Receipt::success("Published").change("Crates", "2");
    println!("{}", receipt.render_markdown(&ctx)?);

    let error = sartorial::ErrorModel::new("Publish blocked")
        .with_why("CI red")
        .with_evidence(Evidence::new("clippy warning").at("src/main.rs:1"));
    println!("{}", error.render_markdown(&ctx)?);
    Ok(())
}
