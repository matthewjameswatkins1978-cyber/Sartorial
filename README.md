# Sartorial

**An opinionated Rust presentation framework for command-line applications.**<br>
*Embodying the Biscuit Logic CLI Presentation Standard (BL-CLI-01).*

Your program knows what happened. Sartorial knows how to present it.

```
                     APPLICATION
                         │
                    semantic truth
                         │
                         ▼
                ┌─────────────────┐
                │ sartorial-core  │
                │                 │
                │ Document        │
                │ Preset          │
                │ Theme           │
                │ Capabilities    │
                │ Rendering       │
                └───────┬─────────┘
                        │
           ┌────────────┼────────────┐
           ▼            ▼            ▼
        TERMINAL       PLAIN       MARKDOWN


                ┌─────────────────┐
                │    sartorial    │
                │                 │
                │ detection       │
                │ progress        │
                │ interaction     │
                │ clap            │
                │ completions     │
                │ optional wire   │
                │ diagnostics     │
                └─────────────────┘
```

---

## Which crate?

| You want … | Use … |
|---|---|
| Deterministic presentation: semantic types → one `Document` → terminal / plain / Markdown, no live-terminal machinery | `sartorial-core` |
| Terminal behaviour and integrations: capability detection, channel hygiene, live progress, prompts, Clap, completions, optional wire protocol | `sartorial` (depends on and re-exports core) |

```toml
[dependencies]
sartorial = "0.3"        # batteries-included toolkit
# - or -
sartorial-core = "0.3"   # tiny deterministic engine only
```

> **Architectural law**: *Sartorial dresses your program's semantics; it never invents them.*
> Applications own truth (schemas, state, logging, execution). Sartorial owns presentation.

---

## Why Sartorial?

Biscuit Logic repeatedly builds terminal software — Terrorbats, Omen, Tethers and what's next.
The same presentation decisions recur: typography, spacing, colours, tables, status views,
errors, prompts, keyboard conventions, progress honesty, dry-run plans, receipts, human vs
plain vs agent output, terminal-width behaviour. Sartorial solves this once. A future builder
or agent can simply be told:

```rust
// "Use Sartorial."
```

Sartorial reuses mature libraries (`anstyle`, `unicode-width`, `serde`, plus `crossterm`,
`indicatif`, `clap` in the full crate) for mechanics while owning the design language.

---

## Preset vs Theme

The core compositional idea. **Presets** are presentation *grammar* (density, title casing,
markers, rules, gaps, table and progress treatment). **Themes** are visual *identity*
(accent, success, warning, failure, muted, evidence, code colours). They compose freely:

```rust
use sartorial::{Config, Preset, Theme};
use anstyle::AnsiColor;

let terrorbats = Theme::builder("Terrorbats")
    .accent(AnsiColor::Red)
    .success(AnsiColor::Green)
    .warning(AnsiColor::Yellow)
    .failure(AnsiColor::Red)
    .build();

// Workwear + Terrorbats. Studio + Terrorbats. No new preset per product.
let config = Config::new()
    .with_preset(Preset::Workwear)
    .with_theme(terrorbats);
```

Four first-party grammars: **House** (default, quiet and restrained), **Black Tie**
(formal, nearly monochrome), **Workwear** (dense, operator-focused), **Studio**
(roomier, presentation-friendly). Without an explicit theme, each resolves to its
familiar v0.2 visuals. An explicit theme is never dropped by a preset change.

---

## One Document, three renderers

Every semantic object (`Outcome`, `Receipt`, `Plan`, `ErrorModel`, `TableModel`,
`Notice`, screens, …) converts once into a presentation `Document` via `Presentable`:

```rust
use sartorial::{Presentable, SummaryScreen, Status};

let screen = SummaryScreen::new("Backup", Status::Ready).fact("Files", "1,204");
let doc = screen.to_document(); // meaning resolved once
```

Renderers decide spacing, glyphs and emphasis — never meaning:

- **Terminal**: styled ANSI, responsive layout, preset grammar.
- **Plain**: pipe-safe, no ANSI, ASCII-safe, log-friendly. Same facts, zero paint.
- **Markdown**: headings, tables, lists, GitHub callouts (`> [!WARNING]`), code —
  built for issues, PRs, release notes, agent reports and handoffs.

```rust
use sartorial::{RenderContext, RenderMarkdown, Preset};

let ctx = RenderContext::markdown(Preset::House);
println!("{}", screen.render_markdown(&ctx)?);
```

---

## Quickstart

### 1. Summary screen

```rust
use sartorial::*;

let screen = SummaryScreen::new("MY-TOOL", Status::Ready)
    .fact("Environment", "Production")
    .fact("Uptime", "99.98%")
    .notice(Notice::info("All systems nominal."))
    .action(Action::open())
    .action(Action::quit());

print_human(&screen)?;   // stdout, respects NO_COLOR / preset / width
```

### 2. Honest progress

```rust
use sartorial::*;

// Unknown total: activity with elapsed count-up, never a fake percentage.
let pb = ProgressBar::activity("Checking repository")
    .with_subtask("cargo test")
    .with_elapsed(14);

// Known totals: real counts, derived percentage, contradictions rejected.
let count = ProgressBar::count("Scanning files", 38, 60)?;
```

Live spinners live in `sartorial` (`indicatif`); static snapshots live in core.
Results → stdout, progress/diagnostics → stderr, machine JSON → stdout only.

### 3. Plans and receipts

```rust
use sartorial::*;

let plan = Plan::new("PATH REPAIR")      // BEFORE: intent, never execution
    .add("C:\\Tools\\bin")
    .warning("User PATH will be updated.")
    .reversible(true);

let receipt = Receipt::success("PATH UPDATED")  // AFTER: truthful changes
    .change("Added", "1 directory")
    .unchanged("System PATH", "unchanged")
    .guidance("Open a new shell for the changes to take effect.");
```

### 4. Clap integration

```rust
use clap::Parser;
use sartorial::clap_ext::SartorialArgs;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    sartorial: SartorialArgs,
}
```

---

## Feature flags (`sartorial`)

Default = full batteries: `terminal`, `progress`, `interactive`, `clap`, `wire`.

| Feature | Pulls in | For |
|---|---|---|
| `terminal` | `crossterm` | width/TTY detection, pager |
| `progress` | `indicatif` | live spinners and bars |
| `interactive` | `crossterm` | prompts, keyboard, raw-mode guards |
| `clap` / `completions` | `clap` / `clap_complete` | CLI wiring, shell completions |
| `wire` | `serde_json` | `sartorial.v0.1` protocol compat, agent JSON, serialized `Document` |
| `diagnostics` | nothing heavy | structured source-diagnostic → `Document` seam |

Pretty static rendering needs none of the above:
`cargo check -p sartorial-core` proves the tiny engine stays tiny.

---

## Machine-output boundary

Terrorbats machine output is a Terrorbats schema — it never has to become a
Sartorial schema to print nicely. The `wire` feature carries Sartorial's own
presentation envelope plus `sartorial.v0.1` compatibility, and a small
serialized-`Document` format for language-neutral presentation. Nothing else.

---

## Examples

```bash
cargo run -p sartorial --example minimal        # ten-line static rendering
cargo run -p sartorial --example one_shot       # canonical one-shot CLI wiring
cargo run -p sartorial --example styles         # one truth, four grammars
cargo run -p sartorial --example markdown       # GitHub-ready reports
cargo run -p sartorial --example custom_theme   # application identity, no new preset
cargo run -p sartorial --example terrorbats     # dogfood: Terrorbats-style campaign report
cargo run -p sartorial --example progress       # honest activity vs counts
cargo run -p sartorial --example plan_receipt   # dry-run plan, then receipt
cargo run -p sartorial --bin sartorial-demo -- styles   # preset runway
```

---

## Verification

```bash
cargo fmt --all --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo check -p sartorial-core --no-default-features
cargo test -p sartorial-core --no-default-features
git diff --check
```

See [Biscuit Logic CLI Presentation Standard (BL-CLI-01)](docs/BL_CLI_STANDARD.md) and
the [integration guide](docs/INTEGRATION.md).
