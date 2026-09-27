use clap::{Parser, Subcommand};
use sartorial::clap_ext::SartorialArgs;
use sartorial::render::{RenderHuman, RenderPlain};
use sartorial::*;
use std::io::stdout;

#[derive(Parser, Debug)]
#[command(
    name = "sartorial-demo",
    version = "0.1.0",
    about = "Biscuit Logic CLI Presentation Standard Showcase"
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
    /// 4. Choice and confirmation interactive components
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
                println!("{}", serde_json::to_string_pretty(&screen)?);
            } else {
                render_visual(&screen, &ctx)?;
            }
        }
        DemoCommand::Table => {
            let table = build_tools_table();
            if ctx.target.is_agent() {
                println!("{}", serde_json::to_string_pretty(&table)?);
            } else {
                let view = TableView::new(table);
                render_visual(&view, &ctx)?;
            }
        }
        DemoCommand::Error { unknown_cause } => {
            let err = build_error(unknown_cause);
            if ctx.target.is_agent() {
                println!("{}", serde_json::to_string_pretty(&err)?);
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
            let choice = Choice::new("Select deployment environment:", items.clone());
            if ctx.target.is_agent() {
                println!("{}", serde_json::to_string_pretty(&items)?);
            } else if ctx.target.is_plain() {
                choice.render_plain(&ctx, &mut stdout())?;
            } else {
                choice.render_human(&ctx, &mut stdout())?;
            }
        }
        DemoCommand::Confirm => {
            let confirm = Confirm::new("Apply configuration changes?").with_default(true);
            if ctx.target.is_agent() {
                println!(r#"{{"prompt": "Apply configuration changes?", "default": true}}"#);
            } else if ctx.target.is_plain() {
                confirm.render_plain(&ctx, &mut stdout())?;
                println!();
            } else {
                confirm.render_human(&ctx, &mut stdout())?;
                println!();
            }
        }
        DemoCommand::Progress => {
            let mut pb = ProgressBar::new("Checking repository")
                .with_subtask("cargo test")
                .with_elapsed(47);
            pb.finish_with_status(Status::Ready);

            if ctx.target.is_agent() {
                println!("{}", serde_json::to_string_pretty(pb.state())?);
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
                println!("{}", serde_json::to_string_pretty(&screen)?);
            } else {
                render_visual(&screen, &ctx)?;
            }
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
            let mut pb = ProgressBar::new("Checking repository")
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
        }
    }

    Ok(())
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
