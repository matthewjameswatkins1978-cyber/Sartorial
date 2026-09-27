# Sartorial

**An opinionated Rust presentation toolkit for command-line applications.**<br>
*Embodying the Biscuit Logic CLI Presentation Standard (BL-CLI-01).*

---

## Why Sartorial?

Biscuit Logic repeatedly builds terminal software. The same presentation decisions recur:
- how headings look;
- how status is presented;
- where keyboard commands live;
- how warnings differ from errors;
- how to display structured state;
- how to make interactive choices;
- how to support both humans and AI agents;
- how to avoid filling an AI context window with raw logs.

Sartorial solves this once. A future agent or human developer can say:

```rust
// "Use Sartorial."
```

and inherit sensible, professional presentation and interaction without designing a CLI from scratch.

> **Sartorial is NOT a new terminal engine.**
> It reuses mature existing Rust libraries (`anstyle`, `anstream`, `crossterm`, `indicatif`, `serde`) for terminal mechanics while owning the design language and semantic layer.

---

## Product Character

Sartorial's visual character is:
```
QUIET · PRECISE · LEGIBLE · RESTRAINED · TECHNICAL · SLIGHTLY ELEGANT
```
Think **well-made precision control instrument**, not a colourful terminal toy.

- **No** giant ASCII art logos or banners.
- **No** boxes around every element.
- **No** rainbow colouring or decorative animations.
- **No** unconstrained log dumps into AI agent context.

---

## Core Rule: One Semantic Authority, Multiple Views

The application truth is represented once in a strongly typed model and rendered across three distinct targets:

```
                     SEMANTIC RESULT (Outcome / ErrorModel / TableModel)
                                             |
                   +-------------------------+-------------------------+
                   |                         |                         |
              HUMAN VIEW                 PLAIN VIEW               AGENT VIEW
            (Styled ANSI)               (Pipe-Safe)                 (JSON)
```

The underlying truth is **never** independently scraped or reconstructed by downstream renderers.

---

## Standard Visual Grammar

```
SARTORIAL

Status                                            [OK] READY

Environment   Windows 11
Architecture  x86_64
Terminal      Windows Terminal

Tools                                                 9 / 11
--------------------------------
Git           2.51.0     ready
ripgrep       14.1.1     ready
fd                       missing
Threadmoth    1.10.0     ready

2 optional tools can be installed.

[I] Install   [/] Find   [?] Help   [Q] Quit
```

### Typographic Hierarchy
- **PROGRAM TITLE**: Visually strongest, compact, uppercase.
- **SECTION**: Bold, clear, quieter than title.
- **LABEL**: Subdued / dim.
- **VALUE**: High legibility, neutral / bold.
- **PRIMARY STATUS**: Redundant symbol + label (`✓ READY` / `[OK] READY`, `× FAILED` / `[X] FAILED`).
- **ACTION FOOTER**: Teaches keyboard grammar (`[Enter] Open   [/] Find   [?] Help   [Q] Quit`).

---

## Quickstart

Add Sartorial to your `Cargo.toml`:

```toml
[dependencies]
sartorial = "0.1"
```

### 1. Summary Screen in 5 Lines

```rust
use sartorial::*;

let screen = SummaryScreen::new("MY-TOOL", Status::Ready)
    .fact("Environment", "Production")
    .fact("Uptime", "99.98%")
    .notice(Notice::info("All systems nominal."))
    .action(Action::open())
    .action(Action::quit());

// Render to terminal (respects NO_COLOR and TTY automatically)
print_human(&screen);
```

### 2. Error Design: What, Why, What Next?

```rust
use sartorial::*;

let error = ErrorModel::new("DATABASE CONNECTION REFUSED")
    .with_why("Connection timed out after 3000ms.")
    .with_evidence(Evidence::new("Connection timeout").at("127.0.0.1:5432"))
    .with_action(Action::retry())
    .with_action(Action::details())
    .with_action(Action::quit());

print_human(&error);
```

*Note: If the cause is unestablished, omit `.with_why(...)`. Sartorial preserves uncertainty rather than hallucinating reasons.*

### 3. Progressive Disclosure (Context Economy for Agents)

AI agents receive compact, structured evidence first. Deep traces or logs remain opt-in:

```rust
use sartorial::*;

let detail = DetailView::new(
    "Clippy Diagnostics",
    Evidence::new("unused import `Path`")
        .at("src/repo.rs:184")
        .with_handle("clippy:warn-0042")
        .with_details("184 | use std::path::Path;\n    |                 ^^^^ help: remove unused import"),
)
.with_action(Action::details())
.with_action(Action::retry())
.with_action(Action::back());
```

In agent mode (`--json`), this serializes directly to clean JSON:

```json
{
  "summary": "unused import `Path`",
  "location": "src/repo.rs:184",
  "handle": "clippy:warn-0042"
}
```

### 4. Integration with `clap`

Sartorial provides ready-to-use CLI arguments:

```rust
use clap::Parser;
use sartorial::clap_ext::SartorialArgs;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    sartorial: SartorialArgs,
}

fn main() {
    let cli = Cli::parse();
    let target = cli.sartorial.target(); // Human, Plain, or Agent (JSON)
    let config = cli.sartorial.to_config(); // Inherits --color auto/always/never & NO_COLOR
}
```

---

## Interactive Keyboard Grammar

| Key | Action | Meaning |
| :--- | :--- | :--- |
| `Enter` | **Open / Accept** | Activate selected item or confirm |
| `Esc` | **Back / Cancel** | Step back or cancel |
| `↑` / `↓` | **Navigate** | Move highlight up or down |
| `Space` | **Select / Toggle** | Toggle item |
| `/` | **Find** | Search / filter |
| `?` | **Help** | Contextual help |
| `D` | **Details** | Open progressive disclosure view |
| `R` | **Retry** | Retry failed operation |
| `Q` | **Quit** | Exit |

---

## Showcase Binary

Run the executable documentation:

```bash
# Run the complete showcase
cargo run --bin sartorial-demo -- all

# Run specific showcases
cargo run --bin sartorial-demo -- summary
cargo run --bin sartorial-demo -- table
cargo run --bin sartorial-demo -- error
cargo run --bin sartorial-demo -- error --unknown-cause
cargo run --bin sartorial-demo -- progress
cargo run --bin sartorial-demo -- detail
cargo run --bin sartorial-demo -- footer
cargo run --bin sartorial-demo -- narrow

# Plain output (pipe-safe)
cargo run --bin sartorial-demo -- --plain summary

# Machine-readable JSON output for AI agents
cargo run --bin sartorial-demo -- --json summary
```

---

## Verification & CI

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
git diff --check
```

Cross-platform CI is configured for Windows, Linux, and macOS via GitHub Actions.
