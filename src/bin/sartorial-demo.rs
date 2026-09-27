use clap::{Parser, Subcommand};
use sartorial::clap_ext::SartorialArgs;
use sartorial::render::{RenderHuman, RenderPlain};
use sartorial::*;
use std::io::{stdout, Write};

#[derive(Parser, Debug)]
#[command(
    name = "sartorial-demo",
    version = "0.1.0",
    about = "Biscuit Logic CLI Presentation Standard Showcase",
    disable_help_subcommand = true
)]
struct Cli {
    #[command(flatten)]
    sartorial: SartorialArgs,

    #[command(subcommand)]
    command: Option<DemoCommand>,
}

#[derive(Subcommand, Debug, Clone)]
enum DemoCommand {
    /// 1. Summary and status screen
    Summary,
    /// 2. Tool and inventory table
    Table,
    /// 3. Known error and unknown error design
    Error {
        #[arg(long)]
        unknown_cause: bool,
    },
    /// 4. Choice interactive component
    Choice,
    /// 4b. Confirmation prompt
    Confirm,
    /// 5. Stateful progress indicator
    Progress,
    /// 6. Detail view / progressive disclosure
    Detail,
    /// 7. Standard BL keyboard footer
    Footer,
    /// 8. Narrow terminal simulation (< 60 columns)
    Narrow,
    /// 9. Proposed dry-run plan
    Plan,
    /// 10. Post-operation receipt
    Receipt,
    /// 11. House-formatted command help
    Help,
    /// 12. Instant 4-way visual style comparison (House, Black Tie, Workwear, Studio)
    Styles,
    /// 13. Full visual regression runway across all presets
    Runway,
    /// 14. Short live progress animation demo
    LiveMotion,
    /// Run all visual showcases sequentially
    All,
}

fn build_tools_table() -> TableModel {
    let mut table = TableModel::new(vec!["Tool", "Version", "Status"])
        .with_title("Tools")
        .with_badge("9 / 11");

    table.add_row(["Git", "2.51.0", "ready"]);
    table.add_row(["ripgrep", "14.1.1", "ready"]);
    table.add_row(["fd", "", "missing"]);
    table.add_row(["Threadmoth", "1.10.0", "ready"]);
    table
}

fn build_summary_screen() -> SummaryScreen {
    SummaryScreen::new("SARTORIAL", Status::Ready)
        .with_subtitle("Environment READY")
        .fact("Environment", "Windows 11")
        .fact("Architecture", "x86_64")
        .fact("Terminal", "Windows Terminal")
        .with_table(build_tools_table())
        .notice(Notice::info("2 optional tools can be installed."))
        .action(Action::new('i', "install", "Install"))
        .action(Action::find())
        .action(Action::help())
        .action(Action::quit())
}

fn build_error(unknown: bool) -> ErrorModel {
    if unknown {
        ErrorModel::new("DATABASE CONNECTION REFUSED")
            .with_evidence(
                Evidence::new("Connection timeout after 3000ms")
                    .at("127.0.0.1:5432")
                    .with_handle("conn-err-782"),
            )
            .with_action(Action::retry())
            .with_action(Action::details())
            .with_action(Action::quit())
    } else {
        ErrorModel::new("THREADMOTH NOT VISIBLE")
            .with_why("Threadmoth is installed, but this process cannot resolve it.")
            .with_evidence(
                Evidence::new("C:\\Users\\Matmus\\.cargo\\bin\\threadmoth.exe")
                    .at("Found in cargo bin directory, but missing from PATH"),
            )
            .with_action(Action::new('r', "recheck", "Recheck"))
            .with_action(Action::details())
            .with_action(Action::quit())
    }
}

fn build_detail_view() -> (DetailView, Evidence) {
    let evidence = Evidence::new("unused import `Path`")
        .at("src/repo.rs:184")
        .with_handle("clippy:warn-0042")
        .with_details(
            "182 | use std::fs;\n183 | use std::io;\n184 | use std::path::Path;\n    |                 ^^^^ help: remove unused import",
        );

    let view = DetailView::new("Clippy Diagnostics", evidence.clone())
        .with_action(Action::details())
        .with_action(Action::new('l', "log", "Full log"))
        .with_action(Action::retry())
        .with_action(Action::back());

    (view, evidence)
}

fn build_plan() -> Plan {
    Plan::new("PATH REPAIR")
        .with_description("Proposed changes")
        .add("C:\\Users\\Matmus\\.cargo\\bin")
        .modify("C:\\Program Files\\Git\\cmd")
        .warning("User PATH will be updated; system PATH remains untouched.")
        .consequence("Existing terminal processes will not inherit this update.")
        .reversible(true)
        .with_action(Action::new('a', "apply", "Apply"))
        .with_action(Action::details())
        .with_action(Action::quit().with_label("Cancel"))
}

fn build_receipt() -> Receipt {
    Receipt::success("PATH UPDATED")
        .change("Added", "1 directory")
        .change("Removed", "0 entries")
        .change("Duplicates", "0")
        .unchanged("System PATH", "unchanged")
        .guidance("Open a new shell for the changes to take effect.")
        .with_evidence_handle("receipt-ref-1082")
        .with_action(Action::open().with_label("Open shell"))
        .with_action(Action::quit())
}

fn build_help_view() -> HelpView {
    HelpView::new("supertools", "supertools <command> [options]")
        .with_description("Understand and operate developer environments.")
        .command("tools", "Show available developer tools")
        .command("doctor", "Check environment health")
        .command("search", "Search repository contents")
        .command("verify", "Run project verification")
        .output_flag("--json", "Structured agent output")
        .output_flag("--plain", "Pipe-safe text")
        .output_flag("--quiet", "Essential output only")
        .example("supertools doctor")
        .example("supertools tools --json")
}

fn render_visual<T>(item: &T, ctx: &RenderContext) -> Result<(), Box<dyn std::error::Error>>
where
    T: RenderHuman + RenderPlain,
{
    match ctx.target {
        RenderTarget::Human => {
            let mut out = anstream::stdout();
            item.render_human(ctx, &mut out)?;
        }
        RenderTarget::Plain | RenderTarget::Agent => {
            let mut out = stdout();
            item.render_plain(ctx, &mut out)?;
        }
    }
    Ok(())
}

fn run_preset_runway(
    preset: Preset,
    base_ctx: &RenderContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = base_ctx.config.clone().with_preset(preset);
    let ctx = RenderContext::detect()
        .with_config(cfg.clone())
        .with_target(base_ctx.target);

    println!("\n============================================================");
    println!(
        "RUNWAY PRESET: {} — {}",
        preset.name(),
        preset.description()
    );
    println!("============================================================");

    println!("\n--- 1. SUMMARY SCREEN ---");
    let summary = build_summary_screen();
    render_visual(&summary, &ctx)?;

    println!("\n--- 2. TOOL TABLE ---");
    let table = TableView::new(build_tools_table());
    render_visual(&table, &ctx)?;

    println!("\n--- 3. STATUS BADGES ---");
    let mut out = anstream::stdout();
    StatusBadge::new(Status::Ready).render_human(&ctx, &mut out)?;
    write!(out, "  ")?;
    StatusBadge::new(Status::Attention).render_human(&ctx, &mut out)?;
    write!(out, "  ")?;
    StatusBadge::new(Status::Failed).render_human(&ctx, &mut out)?;
    write!(out, "  ")?;
    StatusBadge::new(Status::Running).render_human(&ctx, &mut out)?;
    writeln!(out)?;

    println!("\n--- 4. HELP VIEW ---");
    let help = build_help_view();
    render_visual(&help, &ctx)?;

    println!("\n--- 5. ERROR (KNOWN CAUSE) ---");
    let err_known = ErrorView::new(build_error(false));
    render_visual(&err_known, &ctx)?;

    println!("\n--- 6. ERROR (UNKNOWN CAUSE) ---");
    let err_unknown = ErrorView::new(build_error(true));
    render_visual(&err_unknown, &ctx)?;

    println!("\n--- 7. CHOICE (PREVIEW) ---");
    let choice_items = vec![
        ChoiceItem::new("prod", "Production").with_description("Primary production cluster"),
        ChoiceItem::new("staging", "Staging").with_description("Pre-release test cluster"),
        ChoiceItem::new("local", "Local Dev").with_description("Developer workstation sandbox"),
    ];
    let choice = Choice::new("Select deployment environment:", choice_items);
    render_visual(&choice, &ctx)?;

    println!("\n--- 8. CONFIRMATION (PREVIEW) ---");
    let confirm = Confirm::new("Apply configuration changes?").with_default(true);
    render_visual(&confirm, &ctx)?;
    println!();

    println!("\n--- 9. PLAN (DRY-RUN) ---");
    let plan = build_plan();
    render_visual(&plan, &ctx)?;

    println!("\n--- 10. RECEIPT (POST-OPERATION) ---");
    let receipt = build_receipt();
    render_visual(&receipt, &ctx)?;

    println!("\n--- 11. PROGRESS: ACTIVITY (UNKNOWN TOTAL, ELAPSED COUNT-UP) ---");
    let p_activity = ProgressBar::activity("Checking repository")
        .with_subtask("cargo test")
        .with_elapsed(14);
    render_visual(&p_activity, &ctx)?;

    println!("\n--- 12. PROGRESS: COUNT (KNOWN ITEMS) ---");
    let p_count = ProgressBar::count("Scanning files", 38, 60)?.with_elapsed(4);
    render_visual(&p_count, &ctx)?;

    println!("\n--- 13. PROGRESS: PERCENT (KNOWN PROGRESS BAR) ---");
    let p_percent = ProgressBar::percent("Building", 63)
        .with_progress(38, 60, "crates")?
        .with_elapsed(9);
    render_visual(&p_percent, &ctx)?;

    println!("\n--- 14. PROGRESS: COUNTDOWN (REAL FUTURE EVENT) ---");
    let p_countdown = ProgressBar::countdown("Retrying connection", 17);
    render_visual(&p_countdown, &ctx)?;

    println!("\n--- 15. PROGRESS: RATE (THROUGHPUT) ---");
    let p_rate =
        ProgressBar::rate("Downloading artifacts", 84, 140, "MB", "11 MB/s")?.with_elapsed(7);
    render_visual(&p_rate, &ctx)?;

    println!("\n--- 16. PROGRESS: COMPLETED ---");
    let mut p_comp = ProgressBar::activity("Verification suite");
    p_comp.finish_with_status(Status::Ready);
    render_visual(&p_comp, &ctx)?;

    println!("\n--- 17. NARROW TERMINAL SIMULATION (45 COLUMNS) ---");
    let narrow_ctx = RenderContext::detect()
        .with_config(cfg.clone().with_width(45))
        .with_target(base_ctx.target);
    render_visual(&summary, &narrow_ctx)?;

    println!("\n--- 18. LONG-LINE SANITY / PATH TRUNCATION ---");
    let long_path = "C:\\Users\\Matmus\\.cargo\\bin\\tools\\external\\deeply\\nested\\vendor\\subproject\\target\\release\\threadmoth.exe";
    let truncated = sartorial::text::truncate_path(long_path, 48, "...");
    println!("  Original:  {}", long_path);
    println!("  Truncated: {}", truncated);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let target = cli.sartorial.target();
    let mut config = cli.sartorial.to_config();
    let cmd = cli.command.unwrap_or(DemoCommand::Summary);

    if matches!(cmd, DemoCommand::Narrow) {
        config = config.with_width(45);
    }

    let ctx = RenderContext::detect()
        .with_config(config)
        .with_target(target);

    match cmd {
        DemoCommand::Summary => {
            let screen = build_summary_screen();
            if ctx.target.is_agent() {
                println!("{}", screen.to_agent_json(true)?);
            } else {
                render_visual(&screen, &ctx)?;
            }
        }
        DemoCommand::Table => {
            let table = build_tools_table();
            if ctx.target.is_agent() {
                println!("{}", table.to_agent_json(true)?);
            } else {
                let view = TableView::new(table);
                render_visual(&view, &ctx)?;
            }
        }
        DemoCommand::Error { unknown_cause } => {
            let err = build_error(unknown_cause);
            if ctx.target.is_agent() {
                println!("{}", err.to_agent_json(true)?);
            } else {
                let view = ErrorView::new(err);
                render_visual(&view, &ctx)?;
            }
        }
        DemoCommand::Choice => {
            let items = vec![
                ChoiceItem::new("prod", "Production")
                    .with_description("Primary production cluster"),
                ChoiceItem::new("staging", "Staging").with_description("Pre-release test cluster"),
                ChoiceItem::new("local", "Local Dev")
                    .with_description("Developer workstation sandbox"),
            ];
            let mut choice = Choice::new("Select deployment environment:", items.clone())
                .with_non_interactive_fallback(0);
            if ctx.target.is_agent() {
                println!("{}", serde_json::to_string_pretty(&items)?);
            } else if ctx.target.is_plain() {
                choice.render_plain(&ctx, &mut stdout())?;
            } else if ctx.config.is_interactive() {
                let outcome = choice.select_with_config(&ctx.config)?;
                match outcome {
                    ChoiceOutcome::Selected(item) => {
                        println!("\nSelected: {} ({})", item.label, item.id)
                    }
                    ChoiceOutcome::Cancelled => println!("\nSelection cancelled."),
                    ChoiceOutcome::NonInteractiveFallback(item) => {
                        println!("\nNon-interactive fallback: {}", item.label)
                    }
                    ChoiceOutcome::NonInteractiveDenied => {
                        println!("\nNon-interactive selection denied.")
                    }
                }
            } else {
                choice.render_human(&ctx, &mut stdout())?;
            }
        }
        DemoCommand::Confirm => {
            let confirm = Confirm::new("Apply configuration changes?")
                .with_default(true)
                .with_non_interactive_fallback(false);
            if ctx.target.is_agent() {
                println!(r#"{{"prompt": "Apply configuration changes?", "default": true}}"#);
            } else if ctx.target.is_plain() {
                confirm.render_plain(&ctx, &mut stdout())?;
                println!();
            } else if ctx.config.is_interactive() {
                let outcome = confirm.prompt_with_config(&ctx.config)?;
                println!("\nOutcome: {:?}", outcome);
            } else {
                confirm.render_human(&ctx, &mut stdout())?;
                println!();
            }
        }
        DemoCommand::Progress => {
            let mut pb = ProgressBar::activity("Checking repository")
                .with_subtask("cargo test")
                .with_elapsed(47);
            pb.finish_with_status(Status::Ready);

            if ctx.target.is_agent() {
                println!("{}", pb.state().to_agent_json(true)?);
            } else {
                render_visual(&pb, &ctx)?;
            }
        }
        DemoCommand::Detail => {
            let (detail, evidence) = build_detail_view();
            if ctx.target.is_agent() {
                println!("{}", serde_json::to_string_pretty(&evidence)?);
            } else {
                render_visual(&detail, &ctx)?;
            }
        }
        DemoCommand::Footer => {
            let actions = vec![
                Action::open(),
                Action::find(),
                Action::retry(),
                Action::help(),
                Action::quit(),
            ];
            if ctx.target.is_agent() {
                println!("{}", serde_json::to_string_pretty(&actions)?);
            } else {
                let ab = ActionBar::from_actions(actions);
                render_visual(&ab, &ctx)?;
            }
        }
        DemoCommand::Narrow => {
            let screen = build_summary_screen();
            if ctx.target.is_agent() {
                println!("{}", screen.to_agent_json(true)?);
            } else {
                render_visual(&screen, &ctx)?;
            }
        }
        DemoCommand::Plan => {
            let plan = build_plan();
            if ctx.target.is_agent() {
                println!("{}", plan.to_agent_json(true)?);
            } else {
                render_visual(&plan, &ctx)?;
            }
        }
        DemoCommand::Receipt => {
            let receipt = build_receipt();
            if ctx.target.is_agent() {
                println!("{}", receipt.to_agent_json(true)?);
            } else {
                render_visual(&receipt, &ctx)?;
            }
        }
        DemoCommand::Help => {
            let help = build_help_view();
            render_visual(&help, &ctx)?;
        }
        DemoCommand::Styles => {
            println!("╔══════════════════════════════════════════════════════════════╗");
            println!("║          SARTORIAL FOUR-PRESET INSTANT COMPARISON            ║");
            println!("╚══════════════════════════════════════════════════════════════╝\n");

            let screen = build_summary_screen();
            let presets = [
                Preset::House,
                Preset::BlackTie,
                Preset::Workwear,
                Preset::Studio,
            ];

            for p in presets {
                println!("────────────────────────────────────────────────────────────");
                println!("PRESET: {} ({})", p.name(), p.description());
                println!("────────────────────────────────────────────────────────────");
                let p_cfg = ctx.config.clone().with_preset(p);
                let p_ctx = RenderContext::detect()
                    .with_config(p_cfg)
                    .with_target(ctx.target);
                render_visual(&screen, &p_ctx)?;
                println!();
            }
        }
        DemoCommand::Runway => {
            run_preset_runway(Preset::House, &ctx)?;
            run_preset_runway(Preset::BlackTie, &ctx)?;
            run_preset_runway(Preset::Workwear, &ctx)?;
            run_preset_runway(Preset::Studio, &ctx)?;
        }
        DemoCommand::LiveMotion => {
            println!("Starting 1.5s live motion demonstration...");
            let mut pb =
                ProgressBar::activity("Optimizing repository cache").with_subtask("building index");
            pb.start_live(&ctx)?;
            std::thread::sleep(std::time::Duration::from_millis(1500));
            pb.finish_live(Status::Ready, &ctx)?;
        }
        DemoCommand::All => {
            println!("=== 1. SUMMARY SCREEN ===");
            let summary = build_summary_screen();
            render_visual(&summary, &ctx)?;
            println!("\n=== 2. TOOL TABLE ===");
            let table = TableView::new(build_tools_table());
            render_visual(&table, &ctx)?;
            println!("\n=== 3. ERROR (KNOWN CAUSE) ===");
            let err_known = ErrorView::new(build_error(false));
            render_visual(&err_known, &ctx)?;
            println!("\n=== 4. ERROR (UNKNOWN CAUSE) ===");
            let err_unknown = ErrorView::new(build_error(true));
            render_visual(&err_unknown, &ctx)?;
            println!("\n=== 5. PROGRESS ===");
            let mut pb = ProgressBar::activity("Checking repository")
                .with_subtask("cargo test")
                .with_elapsed(47);
            pb.finish_with_status(Status::Ready);
            render_visual(&pb, &ctx)?;
            println!("\n=== 6. DETAIL VIEW ===");
            let (detail, _) = build_detail_view();
            render_visual(&detail, &ctx)?;
            println!("\n=== 7. KEYBOARD FOOTER ===");
            let footer = ActionBar::new()
                .with_action(Action::new('i', "install", "Install"))
                .with_action(Action::find())
                .with_action(Action::help())
                .with_action(Action::quit());
            render_visual(&footer, &ctx)?;
            println!("\n=== 8. PLAN ===");
            let plan = build_plan();
            render_visual(&plan, &ctx)?;
            println!("\n=== 9. RECEIPT ===");
            let receipt = build_receipt();
            render_visual(&receipt, &ctx)?;
        }
    }

    Ok(())
}
