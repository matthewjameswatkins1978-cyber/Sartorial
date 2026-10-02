<p align="center">
  <img src="assets/sartorial-mark.svg" alt="Sartorial mark" width="112" height="112">
</p>

<h1 align="center">Sartorial</h1>

<p align="center"><strong>One meaning. A considered way to present it.</strong><br>
An opinionated Rust presentation toolkit for command-line applications.</p>

Your application knows what happened. Sartorial decides how to show it.
That boundary keeps presentation consistent without taking ownership of your
state, business rules, or machine-readable contract.

```text
Application truth  →  semantic model  →  one Document  →  Terminal / Plain / Markdown
       │                     │                 │
       └── owns state        └── Sartorial     └── same meaning, different presentation
```

## What sets Sartorial apart

- **One meaning, several views.** Semantic values become a `Document` once.
  Terminal, plain text, and Markdown renderers format that shared structure;
  they do not decide whether an operation succeeded or reinterpret its evidence.
- **Your schemas stay yours.** Sartorial presents a view of application
  results. It does not replace your business data or require your `--json`
  contract to become a Sartorial schema.
- **Grammar and identity are separate.** Choose a `Preset` for layout character
  and density; compose it with a `Theme` for application colours. A product
  theme can move between presets without creating a new layout system.
- **A small deterministic core, with terminal tools when you need them.**
  `sartorial-core` renders from explicit capabilities and has no live-terminal
  machinery. `sartorial` adds detection, progress, interaction, Clap support,
  and the optional wire surface.
- **Honest output in pipes and progress displays.** Plain rendering is
  pipe-safe and ASCII-safe. Unknown work is activity, not a made-up percentage;
  known progress uses real counts. Results, diagnostics, and agent JSON have
  explicit output paths.
- **Human and agent output have different jobs.** Human presentation can carry
  layout and emphasis. Sartorial's agent JSON is a versioned presentation
  envelope; your application remains responsible for its own machine schema.
- **Dogfood with the Terrorbats example.** The repository's campaign report
  shows one result with a product theme, two presets, plain text, and Markdown.
  It exercises the same framework path available to downstream applications.

Sartorial is a Rust library, not a hosted service or a new application data
format. It is designed for teams that want their command-line tools to feel
coherent while keeping each tool's decisions and contracts in its own hands.

## Choose your crate

| Need | Crate |
|---|---|
| Deterministic semantic model, `Document`, styles, terminal/plain/Markdown rendering | `sartorial-core` |
| Capability detection, channels, live progress, prompts, Clap, completions, optional wire protocol | `sartorial` (re-exports core) |

Until the first crates.io publication, depend on the Git repository:

```toml
[dependencies]
sartorial = { git = "https://github.com/matthewjameswatkins1978-cyber/Sartorial" }
# or, for the deterministic engine only:
sartorial-core = { git = "https://github.com/matthewjameswatkins1978-cyber/Sartorial" }
```

The workspace version is `0.3.0`; crates.io coordinates will become the preferred
installation form once that version is actually published.

## Preset and Theme

A **Preset** describes presentation grammar: density, title treatment, markers,
rules, spacing, and table or progress layout. A **Theme** describes visual
identity: accent, status, muted, evidence, and code colours. They compose:

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

The four built-in presets are **House** (quiet and restrained), **Black Tie**
(formal and nearly monochrome), **Workwear** (compact and operator-focused),
and **Studio** (roomier and presentation-friendly). An explicit theme survives
preset changes; a preset never changes the facts being rendered.

## One Document, three static renderers

Use the semantic types that fit the result—such as `SummaryScreen`, `Plan`,
`Receipt`, `ErrorModel`, or `TableModel`—and turn a presentable value into its
shared document:

```rust
use sartorial::{Presentable, Status, SummaryScreen};

let screen = SummaryScreen::new("Backup", Status::Ready)
    .fact("Files", "1,204");
let document = screen.to_document();
```

The static renderers are:

- **Terminal** for styled, width-aware human output.
- **Plain** for no-ANSI, ASCII-safe, pipe-friendly output.
- **Markdown** for reports, issues, pull requests, and handoffs.

The full `sartorial` crate also provides `RenderAgent` for supported Sartorial
types. That JSON surface is separate from `Document` rendering and from your
application's own machine-output schema.

## Quick start

### Render a result

```rust
use sartorial::*;

let screen = SummaryScreen::new("MY-TOOL", Status::Ready)
    .fact("Environment", "Production")
    .fact("Uptime", "99.98%")
    .notice(Notice::info("All systems nominal."));

let context = RenderContext::human(Preset::House);
SartorialOutput::print_result(&screen, &context)?;
```

Use a plain context when output must be safe to redirect. Markdown and agent
output have dedicated helpers; `print_result` rejects those targets instead of
silently changing formats.

### Report progress truthfully

```rust
use sartorial::ProgressBar;

let activity = ProgressBar::activity("Checking repository");
let counted = ProgressBar::count("Scanning files", 38, 60)?;
```

Unknown totals stay indeterminate. Known totals use real counts, and
contradictory states are rejected. Live terminal progress belongs to the full
crate; the core can render static progress snapshots.

### Show a plan and a receipt

```rust
use sartorial::Plan;

let plan = Plan::new("PATH REPAIR")
    .add("C:\\Tools\\bin")
    .warning("User PATH will be updated.")
    .reversible(true);
```

A `Plan` describes proposed work; constructing or rendering it does not perform
that work. Build a `Receipt` from the operation's actual result.

### Integrate Clap

```rust
use clap::Parser;
use sartorial::clap_ext::SartorialArgs;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    sartorial: SartorialArgs,
}
```

## Output channels and feature flags

`SartorialOutput` routes primary human/plain results and Markdown to stdout,
progress and diagnostics to stderr, and supported agent JSON to stdout. Agent
mode suppresses prose diagnostics. Your own machine JSON should be serialized
through your own schema, without Sartorial presentation text mixed into it.

The `sartorial` crate enables `terminal`, `progress`, `interactive`, `clap`,
and `wire` by default. Disable default features to select a smaller surface.
`sartorial-core` has no default features and does not depend on `crossterm`,
`indicatif`, `clap`, or `serde_json`.

| Feature | Adds | Use |
|---|---|---|
| `terminal` | `crossterm` | terminal sizing and pager support |
| `progress` | `indicatif` | live progress bars and spinners |
| `interactive` | `crossterm` | prompts and keyboard handling |
| `clap` / `completions` | `clap` / `clap_complete` | CLI options and shell completions |
| `wire` | `serde_json` | versioned Sartorial protocol and agent JSON |
| `diagnostics` | no additional dependency | structured source-diagnostic presentation |

## Examples

```bash
cargo run -p sartorial --example minimal
cargo run -p sartorial --example styles
cargo run -p sartorial --example markdown
cargo run -p sartorial --example custom_theme
cargo run -p sartorial --example terrorbats
cargo run -p sartorial --example progress
cargo run -p sartorial --example plan_receipt
```

## Documentation

- [Architecture and boundaries](docs/ARCHITECTURE.md)
- [Integration guide](docs/INTEGRATION.md)
- [Biscuit Logic CLI Presentation Standard](docs/BL_CLI_STANDARD.md)
- [Core crate guide](crates/sartorial-core/README.md)
- [Full toolkit guide](crates/sartorial/README.md)

## Development checks

```bash
cargo fmt --all --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo check -p sartorial-core --no-default-features
cargo test -p sartorial-core --no-default-features
git diff --check
```

Sartorial's architectural rule is simple: **your application owns truth;
Sartorial owns presentation.**
