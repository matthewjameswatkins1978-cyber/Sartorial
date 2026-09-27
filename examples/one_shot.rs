//! Complete non-interactive one-shot CLI built on Sartorial.
//!
//! Run: `cargo run --example one_shot [--style house|black-tie|workwear|studio] [--plain]`
//!
//! Demonstrates the canonical wiring in ~100 lines: context constructors,
//! stderr progress with honest phases, a summary screen, a dry-run Plan, a
//! Receipt from the actual result, and an ErrorView on failure — with no
//! blocking interaction anywhere.

use sartorial::{
    Action, ErrorModel, Evidence, MotionMode, Notice, Plan, PlanChange, Preset, ProgressBar,
    Receipt, RenderContext, SartorialOutput, Status, SummaryScreen, TableModel,
};

fn parse_style(args: &[String]) -> Preset {
    let mut style = Preset::House;
    let mut it = args.iter().skip(1);
    while let Some(a) = it.next() {
        if a == "--style" {
            style = match it.next().map(String::as_str) {
                Some("black-tie") => Preset::BlackTie,
                Some("workwear") => Preset::Workwear,
                Some("studio") => Preset::Studio,
                _ => Preset::House,
            };
        }
    }
    style
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let plain = args.iter().any(|a| a == "--plain");
    let preset = parse_style(&args);
    let ctx = if plain {
        RenderContext::plain_preset(preset)
    } else {
        RenderContext::human_motion(preset, MotionMode::Auto)
    };

    // 1. Honest progress on stderr: phases retitle an Activity bar.
    let mut progress = ProgressBar::activity("Checking demo project");
    progress.start_live(&ctx)?;
    for phase in ["discover", "walk", "languages", "finalize"] {
        progress.update_subtask(phase);
        progress.update_live(&ctx)?;
    }

    // 2. The operation "fails" here when --fail is passed (ErrorView path).
    if args.iter().any(|a| a == "--fail") {
        progress.finish_live(Status::Failed, &ctx)?;
        let view = sartorial::ErrorView::new(
            ErrorModel::new("Demo check failed")
                .with_why("The --fail flag was passed; nothing was actually scanned.")
                .with_evidence(Evidence::new("No evidence: this error is synthetic."))
                .with_action(Action::new('r', "retry", "Rerun without --fail")),
        );
        SartorialOutput::print_diagnostic(&view, &ctx)?;
        std::process::exit(1);
    }
    progress.finish_live(Status::Ready, &ctx)?;

    // 3. Primary result on stdout.
    let mut table = TableModel::new(vec!["Tool", "Version", "Status"]).with_title("Tools");
    table.add_row(["Git", "2.51.0", "ready"]);
    table.add_row(["fd", "", "missing"]);
    let summary = SummaryScreen::new("ONE-SHOT", Status::Ready)
        .with_subtitle("demo project")
        .fact("Environment", "Example")
        .fact("Checks", "42 passed")
        .with_table(table)
        .notice(Notice::info("1 optional tool can be installed."))
        .action(Action::new('i', "install", "Install fd for faster search"));
    SartorialOutput::print_result(&summary, &ctx)?;

    // 4. Consequential dry-run: a Plan that mutates nothing.
    let plan = Plan::new("Demo changes")
        .with_description("Dry run — no files were written.".to_string())
        .add_change(PlanChange::add("/demo/output").with_detail("Create directory".to_string()))
        .consequence("Nothing on disk changes until applied.".to_string());
    SartorialOutput::print_result(&plan, &ctx)?;

    // 5. Receipt from the actual result (here: the plan previewed above).
    let receipt = Receipt::success("Demo applied")
        .change("Created", "/demo/output")
        .change("Files written", "3");
    SartorialOutput::print_result(&receipt, &ctx)?;
    Ok(())
}
