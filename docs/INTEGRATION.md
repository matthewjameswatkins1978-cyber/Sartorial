# Integration guide

Sartorial presents application semantics; it does not define them. Keep
business state, validation, logging, execution, and application JSON in your
program. Choose Sartorial types for the human-facing view of a result.

For the crate split and data flow, see [Architecture](ARCHITECTURE.md).

## Choose a crate

| Need | Use |
|---|---|
| Deterministic semantic types, one `Document`, terminal/plain/Markdown renderers | `sartorial-core` |
| Detection, channel helpers, live progress, prompts, Clap, completions, optional wire | `sartorial` |

Until the first crates.io publication, use the Git repository:

```toml
[dependencies]
sartorial = { git = "https://github.com/matthewjameswatkins1978-cyber/Sartorial" }
# or
sartorial-core = { git = "https://github.com/matthewjameswatkins1978-cyber/Sartorial" }
```

The workspace version is `0.3.0`; switch to crates.io coordinates after that
version is actually published.

## Choose a presentation type

| Application result | Sartorial model | Keep in mind |
|---|---|---|
| A result snapshot | `SummaryScreen` / `Outcome` | Status describes the result you supply. |
| A long record list | `ListScreen` and `TableModel` | Preserve your records as the source of truth. |
| One entity with supporting detail | `DetailScreen` / `DetailView` and `Evidence` | Include only evidence the application has. |
| Work with unknown total | `ProgressBar::activity(task)` | Do not invent a count, percent, or ETA. |
| Work with known total | `ProgressBar::count(task, done, total)` | Counts must describe real work; invalid states fail. |
| A real timed wait | `ProgressBar::countdown(task, seconds)` | Use for a genuine countdown. |
| A consequential proposed change | `Plan` | Describe intent before the operation; rendering does not execute it. |
| A completed state change | `Receipt` | Build it from the actual operation result. |
| A failure | `ErrorModel` | Omit an unknown cause rather than guessing. |
| A non-fatal warning | `Notice::warning(...)` | Keep warnings distinct from failures. |
| A Markdown report | `RenderMarkdown` | Use the static Markdown renderer. |
| Application machine output | Your schema and serializer | Do not route business JSON through Sartorial presentation models. |

For a custom view, implement `Presentable::to_document()` and let the static
renderers handle terminal, plain text, and Markdown. Sartorial renderers decide
formatting, not whether the application succeeded or what its evidence means.

## Preset and Theme

```rust
use anstyle::AnsiColor;
use sartorial::{Config, Preset, Theme};

let theme = Theme::builder("Terrorbats")
    .accent(AnsiColor::Red)
    .success(AnsiColor::Green)
    .warning(AnsiColor::Yellow)
    .failure(AnsiColor::Red)
    .build();

let config = Config::new()
    .with_preset(Preset::Workwear)
    .with_theme(theme);
```

Presets control layout grammar; themes provide visual identity. Both affect
presentation only. The four built-in presets are House, Black Tie, Workwear,
and Studio. An explicit theme survives preset changes.

## Render a result

```rust
use sartorial::{Preset, RenderContext, SartorialOutput, Status, SummaryScreen};

let screen = SummaryScreen::new("Backup", Status::Ready)
    .fact("Files", "1,204");
let context = RenderContext::human(Preset::House);
SartorialOutput::print_result(&screen, &context)?;
```

For redirected output, explicitly select a plain target:

```rust
use sartorial::{Preset, RenderContext, RenderTarget};

let context = RenderContext::human(Preset::House)
    .with_target(RenderTarget::Plain);
```

Alternatively use `RenderContext::plain()` or `plain_preset(...)`. Plain
output has no ANSI and uses ASCII-safe rendering. Selecting a plain target is
an explicit presentation choice; applications decide when to make that choice.

Use the matching output helper for each format:

- Human or plain results: `SartorialOutput::print_result` → stdout.
- Markdown: `SartorialOutput::print_markdown` → stdout.
- Progress and diagnostics: `print_progress` / `print_diagnostic` → stderr.
- Supported Sartorial JSON: `print_agent_json` → stdout (`wire` feature).

The helpers reject a Markdown or Agent context passed to `print_result`,
rather than silently changing its format. Agent-mode progress and diagnostic
prose is suppressed. Application-owned machine JSON remains the application's
responsibility.

## Capabilities, pipes, and motion

Core rendering consumes explicit capabilities and does not probe the process
environment. In the full crate, `RenderContext::detect()` captures the
environment for result rendering. It reads stdout's TTY state for result
style and layout. Live progress is written to stderr and checks stderr's TTY
when it starts; this lets piped results remain clean while an attended
terminal can still show progress.

Plain, Markdown, and Agent targets are static: no ANSI colour and no motion.
`NO_COLOR`, color choice, width, Unicode, hyperlink, motion, and interactive
settings affect presentation capabilities; they do not alter application
facts or schemas. Use explicit context construction or `start_live_with_tty`
when deterministic tests need to control the environment.

Unknown work uses activity progress. Known totals use real counts; percentages
are derived. Contradictory counts and `current > total` are rejected, while a
zero total remains indeterminate. Minimal and Numeric progress treatments
reprint static state only when the semantic value changes.

## Interaction and action keys

Only show `Action` keys that the application actually handles. Otherwise omit
the key and present the guidance as text. This keeps displayed affordances
honest.

Confirm and Choice prompts use the configured interactivity capability and
fail closed when a terminal interaction is unavailable unless the caller
provides an explicit non-interactive fallback. Applications own the meaning
of accepted choices and all resulting side effects.

## Optional wire protocol

The `wire` feature exposes Sartorial's versioned presentation protocol
(`sartorial.v0.1` compatibility) and agent JSON for supported Sartorial
values. Serialized Documents use the separate
`sartorial.document.v0.1` format. These are not application business schemas.

For `--json`, serialize the application's own data directly to stdout and
keep that output free of presentation text. Do not put human-readable progress
or warnings on the same machine-output stream.

## Examples

Read the worked examples in [`crates/sartorial/examples`](../crates/sartorial/examples):

- `minimal.rs` for a small static view.
- `one_shot.rs` for target and channel selection.
- `terrorbats.rs` for a product theme across presets, Plain, and Markdown.
- `markdown.rs`, `progress.rs`, and `plan_receipt.rs` for their focused cases.
