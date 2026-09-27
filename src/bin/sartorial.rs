use clap::{Parser, Subcommand};
use sartorial::clap_ext::SartorialArgs;
use sartorial::exit::ExitCode;
use sartorial::protocol::{ChoiceResult, ConfirmResult, ProgressEvent, ProtocolEnvelope};
use sartorial::render::{RenderHuman, RenderPlain};
use sartorial::semantic::choice::ChoiceItem;
use sartorial::*;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, IsTerminal, Read};

#[derive(Parser, Debug)]
#[command(
    name = "sartorial",
    version = "0.1.0",
    about = "Language-Neutral Terminal Presentation Driver (BL-CLI-01)"
)]
struct Cli {
    #[command(flatten)]
    sartorial: SartorialArgs,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// Render structured Sartorial JSON from a file or stdin
    Render {
        /// Input JSON file (omit or use "-" for stdin)
        file: Option<String>,
    },
    /// Stream live JSONL progress events from a file or stdin
    Stream {
        /// Input JSONL file (omit or use "-" for stdin)
        file: Option<String>,
    },
    /// Run a safe confirmation prompt
    Confirm {
        /// Question or confirmation prompt text
        prompt: String,
        /// Default response: true (yes) or false (no)
        #[arg(long, default_value = "true")]
        default: bool,
        /// Non-interactive fallback (if absent, fails closed)
        #[arg(long)]
        fallback: Option<bool>,
    },
    /// Run an interactive choice selection
    Choice {
        /// Choice prompt header
        prompt: String,
        /// Choice options formatted as id:label or id:label:description
        #[arg(short, long = "item", required = true)]
        items: Vec<String>,
        /// Non-interactive fallback index
        #[arg(long)]
        fallback: Option<usize>,
    },
}

fn read_input(file: Option<&str>) -> io::Result<String> {
    match file {
        None | Some("-") => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
        Some(path) => fs::read_to_string(path),
    }
}

fn parse_choice_item(raw: &str) -> ChoiceItem {
    let parts: Vec<&str> = raw.splitn(3, ':').collect();
    match parts.len() {
        1 => ChoiceItem::new(parts[0], parts[0]),
        2 => ChoiceItem::new(parts[0], parts[1]),
        _ => ChoiceItem::new(parts[0], parts[1]).with_description(parts[2]),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let target = cli.sartorial.target();
    let config = cli.sartorial.to_config();
    let ctx = RenderContext::detect()
        .with_config(config)
        .with_target(target);

    // If no subcommand is specified, and stdin is redirected, default to Render from stdin
    let cmd = match cli.command {
        Some(c) => c,
        None => {
            if !io::stdin().is_terminal() {
                Command::Render { file: None }
            } else {
                eprintln!("Usage: sartorial <COMMAND> or pipe JSON into sartorial");
                eprintln!("Try 'sartorial --help' for details.");
                ExitCode::UsageError.exit_process();
            }
        }
    };

    match cmd {
        Command::Render { file } => {
            let raw = match read_input(file.as_deref()) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("sartorial: failed to read input: {e}");
                    ExitCode::UsageError.exit_process();
                }
            };
            let envelope = match ProtocolEnvelope::from_json_str(&raw) {
                Ok(env) => env,
                Err(e) => {
                    eprintln!("sartorial: {e}");
                    ExitCode::UsageError.exit_process();
                }
            };

            if ctx.target.is_agent() {
                println!("{}", envelope.to_agent_json(true)?);
            } else if ctx.target.is_plain() {
                let mut out = io::stdout();
                envelope.render_plain(&ctx, &mut out)?;
            } else {
                let mut out = anstream::stdout();
                envelope.render_human(&ctx, &mut out)?;
            }
            ExitCode::Success.exit_process();
        }
        Command::Stream { file } => {
            let reader: Box<dyn BufRead> = match file.as_deref() {
                None | Some("-") => Box::new(io::BufReader::new(io::stdin())),
                Some(path) => match fs::File::open(path) {
                    Ok(f) => Box::new(io::BufReader::new(f)),
                    Err(e) => {
                        eprintln!("sartorial: failed to open stream file: {e}");
                        ExitCode::UsageError.exit_process();
                    }
                },
            };

            let mut active_bars: HashMap<String, ProgressBar> = HashMap::new();

            for line_res in reader.lines() {
                let line = match line_res {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("sartorial: error reading stream: {e}");
                        ExitCode::Failed.exit_process();
                    }
                };
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let event = match ProgressEvent::from_json_str(trimmed) {
                    Ok(ev) => ev,
                    Err(e) => {
                        eprintln!("sartorial: {e}");
                        ExitCode::UsageError.exit_process();
                    }
                };

                if ctx.target.is_agent() {
                    let serialized = serde_json::to_string(&event)?;
                    println!("{serialized}");
                    continue;
                }

                match event {
                    ProgressEvent::Start {
                        id,
                        activity,
                        subtask,
                        total,
                        unit,
                        ..
                    } => {
                        let mut pb = if let Some(tot) = total {
                            ProgressBar::count(activity, 0, tot)
                        } else {
                            ProgressBar::activity(activity)
                        };
                        if let Some(sub) = subtask {
                            pb = pb.with_subtask(sub);
                        }
                        if let (Some(tot), Some(u)) = (total, unit) {
                            pb = pb.with_progress(0, tot, u);
                        }
                        pb.start_live(&ctx)?;
                        active_bars.insert(id, pb);
                    }
                    ProgressEvent::Update {
                        id,
                        current,
                        percent,
                        rate,
                        elapsed_secs,
                        subtask,
                        ..
                    } => {
                        if let Some(pb) = active_bars.get_mut(&id) {
                            if let Some(cur) = current {
                                let tot = pb.state().total.unwrap_or(cur);
                                let unit = pb
                                    .state()
                                    .unit
                                    .clone()
                                    .unwrap_or_else(|| "units".to_string());
                                pb.state_mut().current = Some(cur);
                                pb.state_mut().total = Some(tot);
                                pb.state_mut().unit = Some(unit);
                            }
                            if let Some(pct) = percent {
                                pb.state_mut().percent = Some(pct);
                            }
                            if let Some(secs) = elapsed_secs {
                                pb.state_mut().elapsed_secs = Some(secs);
                            }
                            if let Some(r) = rate {
                                pb.state_mut().rate = Some(r);
                            }
                            if let Some(sub) = subtask {
                                pb.state_mut().subtask = Some(sub);
                            }
                            pb.update_live(&ctx)?;
                        }
                    }
                    ProgressEvent::Finish { id, status, .. } => {
                        if let Some(mut pb) = active_bars.remove(&id) {
                            pb.finish_live(status, &ctx)?;
                        }
                    }
                }
            }
            ExitCode::Success.exit_process();
        }
        Command::Confirm {
            prompt,
            default,
            fallback,
        } => {
            let mut confirm = Confirm::new(&prompt).with_default(default);
            if let Some(fb) = fallback {
                confirm = confirm.with_non_interactive_fallback(fb);
            }

            let outcome = confirm.prompt_with_config(&ctx.config)?;
            let result = ConfirmResult::from_outcome(&outcome);
            println!("{}", serde_json::to_string(&result)?);
            ExitCode::from_confirm_outcome(&outcome).exit_process();
        }
        Command::Choice {
            prompt,
            items,
            fallback,
        } => {
            let choice_items: Vec<ChoiceItem> =
                items.iter().map(|s| parse_choice_item(s)).collect();
            let mut choice = Choice::new(&prompt, choice_items);
            if let Some(fb) = fallback {
                choice = choice.with_non_interactive_fallback(fb);
            }

            let outcome = choice.select_with_config(&ctx.config)?;
            let result = ChoiceResult::from_outcome(&outcome);
            println!("{}", serde_json::to_string(&result)?);
            ExitCode::from_choice_outcome(&outcome).exit_process();
        }
    }
}
