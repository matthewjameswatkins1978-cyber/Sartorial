# Sartorial Integration Guide

One page for the application builder. **Sartorial dresses your program's
semantics; it never invents them.** Applications own truth. Sartorial owns
presentation.

## Use `sartorial-core` when … / `sartorial` when …

| You want … | Use … |
|---|---|
| Deterministic pretty-printing with no terminal machinery | `sartorial-core` |
| Capability detection, live progress, prompts, Clap, completions, wire | `sartorial` (re-exports core) |

```toml
[dependencies]
sartorial-core = "0.3"   # tiny engine: Document + Preset + Theme + renderers
# - or -
sartorial = "0.3"        # batteries-included terminal toolkit
```

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
| A compiler/test-style span | `Diagnostic` (`diagnostics` feature) + file/line/label/code/help | Renders as an error document; adapt to miette on your side if needed |
| A GitHub issue / PR / handoff | `RenderMarkdown::render_markdown()` on any `Presentable` | Headings, tables, callouts — no terminal styling |
| Machine output | Your own schema with plain `serde_json`; `RenderAgent` only for Sartorial's own envelope | Never wrap your app JSON in a Sartorial envelope |

## The Document model

Higher-level objects converge through one path:

```
SummaryScreen ──┐
Receipt ────────┤
Plan ───────────┤
Error ──────────┤                  ┌──► Terminal
Table ──────────┼──► Document ─────┼──► Plain
Custom view ────┤  (Presentable)   └──► Markdown
Detail ─────────┤
List ───────────┘
```

Meaning is resolved **before** renderer-specific formatting. Implement
`Presentable::to_document()` for custom views; the three renderers handle the
rest. Renderers decide spacing, wrapping, glyphs and emphasis — never whether
something succeeded or what evidence means.

## Preset vs Theme

- **Preset** = presentation *grammar*: density, title casing, markers, rules,
  gaps, table and progress treatment. One of `House`, `BlackTie`, `Workwear`,
  `Studio`.
- **Theme** = visual *identity*: accent, heading, success, warning, failure,
  muted, evidence, code, path, number. Built by applications:

```rust
use sartorial::{Config, Preset, Theme};
use anstyle::AnsiColor;

let theme = Theme::builder("Terrorbats")
    .accent(AnsiColor::Red)
    .success(AnsiColor::Green)
    .warning(AnsiColor::Yellow)
    .failure(AnsiColor::Red)
    .build();

let config = Config::new()
    .with_preset(Preset::Workwear)   // grammar
    .with_theme(theme);              // identity
```

Rules: presets never change facts, ordering, statuses, warnings, or agent
JSON (only layout voice). Themes never change facts either (only paint).
An explicit theme survives preset changes; without one, each preset resolves
to its familiar default visuals. After resolution, `ResolvedStyle` carries
concrete decisions — renderers never branch on preset or theme identity.

## Capabilities: detect once

Core consumes an explicit `Capabilities` value (width, TTY, colour, Unicode,
hyperlinks, motion, interactivity) and never sniffs the environment while
rendering. The full crate detects once (`RenderContext::detect()`), builds the
capabilities, and passes them down — which is why output is deterministic in
tests, pipes, and on Windows.

## Output and channel rules (the whole contract)

- Results → **stdout** via `SartorialOutput::print_result`.
- Progress, warnings, diagnostics → **stderr** (`print_progress` / `print_diagnostic`).
- Markdown documents → **stdout** via `SartorialOutput::print_markdown`.
- Agent JSON → **stdout only**; progress is suppressed entirely in Agent mode.
- `--json` machine output must be your own schema with **zero** presentation noise.
- `NO_COLOR` and `ColorChoice::Never` strip paint without changing layout.
- Plain target is ASCII-safe and redirect-safe by construction.

## TTY behavior you must know

- `RenderContext::detect()` reads the **stdout** TTY (color, symbols, width).
- `ProgressBar::start_live()` reads the **stderr** TTY (spinners draw on stderr).
- The split is deliberate: piped results stay clean while an attended
  terminal still animates, and vice versa. Pass an explicit boolean with
  `start_live_with_tty` in tests.
- `MotionMode::Never` overrides every preset; plain, Markdown and agent
  targets never animate; `NO_COLOR` kills color everywhere without changing layout.

## Progress honesty (non-negotiable)

- Unknown work gets **activity** progress, never a fabricated percentage.
- Known totals show real counts and derived percentages; contradictions and
  `current > total` are rejected at the semantic boundary.
- Zero totals stay indeterminate (`None`), never fake 100%.
- Minimal/Numeric treatments reprint one static line per semantic change —
  never busy ticks.

## Minimal wiring (one-shot CLI)

```rust
use sartorial::{Preset, RenderContext, SartorialOutput, SummaryScreen, Status};

// Human House view for terminals; explicit pipe-safe plain for pipes.
let ctx = RenderContext::human(Preset::House);
SartorialOutput::print_result(&screen, &ctx)?;

// Machine mode: your own data, your own JSON, stdout only, stderr silent.
// (Do NOT route application JSON through Sartorial types.)
```

Full worked examples: `examples/one_shot.rs`, `examples/minimal.rs`,
`examples/terrorbats.rs` (custom-theme dogfood), `examples/markdown.rs`.

## Feature flags

Default is the full toolkit (`terminal`, `progress`, `interactive`, `clap`,
`wire`). Trim by depending with `default-features = false` and adding back
only what the product needs; `sartorial-core` alone covers static rendering.
The `wire` feature is the only way `serde_json`/protocol surface enters;
`diagnostics` adds the source-diagnostic seam with no heavy dependencies.

## Traps

1. **Dead keys.** Attaching `Action::new('d', …)` when no key handling exists
   prints a fake `[D]` prompt. Attach only keys the application really
   handles; put the guidance in a notice instead.
2. **Fake progress.** `Activity` mode exists precisely so unknown work never
   needs a percentage. Totals that aren't real are lies with numbers.
3. **Guessed causes.** `ErrorModel` without `with_why` renders an explicit
   "cause undetermined" line. That honesty beats a plausible fiction.
4. **Invented schemas.** If you find yourself reshaping application data to
   fit Sartorial types for *machine* output, stop: present a projection for
   humans, serialize the original for machines.
