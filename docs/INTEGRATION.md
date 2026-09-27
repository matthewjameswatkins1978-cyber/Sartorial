# Sartorial Integration Guide

One page for the application builder. Sartorial dresses your program's
semantics; it never invents them.

## Decision table: I have X → use Y

| I have … | Use … | Notes |
|---|---|---|
| A result snapshot to summarize | `SummaryScreen::new(title, status)` + `.fact()` + optional `.with_table()` | Status describes *completion* (Ready/Attention/Failed), never health |
| A long inventory or record list | `ListScreen::new(title, table)` | Table carries the data; no raw struct dumps |
| One entity with deep evidence | `DetailScreen` / `DetailView` + `Evidence` | Progressive disclosure, not a wall of text |
| A scan with unknown total work | `ProgressBar::activity(task)` + `update_subtask` per phase | Never set counts or percents you don't have |
| Known completed/total items | `ProgressBar::count(task, done, total)` | Totals must be real; contradictions are rejected |
| A real future wait | `ProgressBar::countdown(task, secs)` | Only for genuine timing events |
| A consequential dry run | `Plan::new(title)` + `add_change` / `warning` / `consequence` | Rendering a Plan must never mutate anything |
| A completed state change | `Receipt::success(title)` + `.change()` from the **actual** result | Never reconstruct what "probably happened" |
| A fatal failure | `ErrorModel::new(what)` + `.with_why()` when genuinely known | Omit `why` rather than guessing; uncertainty is preserved |
| A non-fatal warning | `Notice::warning(msg)` | Warnings stay notices; never promote them to errors |
| A next step with no working key | Omit it; attach only keys the app really handles | Dead keys are worse than no keys — a notice can carry the guidance instead |
| A next step with a working key | `Action::new(key, id, label)` | Only if the keypress is really handled |
| Machine output | `RenderAgent::to_agent_json()` on the semantic type | Never wrap your own JSON in a Sartorial envelope |

## Minimal wiring (one-shot CLI)

```rust
use sartorial::{Preset, RenderContext, SartorialOutput, SummaryScreen, Status};

// Human House view for terminals; explicit pipe-safe plain for pipes.
// human() does NOT auto-switch: it sniffs the stdout TTY for color/symbols
// but the target stays Human. Call plain_preset() when stdout is redirected.
let ctx = RenderContext::human(Preset::House);

// Machine mode: your own data, your own JSON, stdout only, stderr silent.
// (Do NOT route application JSON through Sartorial types.)
```

Full worked example: `examples/one_shot.rs` (`cargo run --example one_shot`).

## Channel rules (the whole contract)

- Results → **stdout** via `SartorialOutput::print_result`.
- Progress, warnings, diagnostics → **stderr** (`print_progress` / `print_diagnostic`).
- Agent JSON → **stdout only**; progress is suppressed entirely in Agent mode.
- `--json` machine output must be your own schema with **zero** presentation noise on stderr.

## TTY behavior you must know

- `RenderContext::detect()` reads the **stdout** TTY (color, symbols, width).
- `ProgressBar::start_live()` reads the **stderr** TTY (spinners draw on stderr).
- The split is deliberate: piped results stay clean while an attended
  terminal still animates, and vice versa. Pass an explicit boolean with
  `start_live_with_tty` in tests.
- `MotionMode::Never` overrides every preset; plain and agent targets never
  animate; `NO_COLOR` kills color everywhere without changing layout.

## Preset contract

Presets are rendering grammars, not color themes: with color disabled, each
preset keeps its silhouette (title casing, rules, markers, density, gaps).
Presets never change facts, ordering, statuses, warnings, or agent JSON.

## Traps

1. **Dead keys.** Attaching `Action::new('d', …)` when no key handling exists
   prints a fake `[D]` prompt. Attach only keys the application really
   handles; put the guidance in a notice instead.
2. **Fake progress.** `Activity` mode exists precisely so unknown work never
   needs a percentage. Totals that aren't real are lies with numbers.
3. **Guessed causes.** `ErrorModel` without `with_why` renders an explicit
   "cause undetermined" line. That honesty beats a plausible fiction.
