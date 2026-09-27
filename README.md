# Sartorial

**An opinionated Rust presentation toolkit for command-line applications.**<br>
*Embodying the Biscuit Logic CLI Presentation Standard (BL-CLI-01).*

---

## Why Sartorial?

Biscuit Logic repeatedly builds terminal software. The same presentation decisions recur:
- typography hierarchy;
- spacing;
- colours;
- tables;
- status views;
- errors;
- prompts;
- keyboard conventions;
- progress and motion;
- dry-run plans and receipts;
- human vs agent output;
- terminal-width behaviour.

Sartorial solves this once. A future builder or AI agent can say:

```rust
// "Use Sartorial."
```

and inherit sensible, professional presentation and interaction without designing a CLI from scratch.

> **Sartorial is NOT a new terminal engine.**
> It reuses mature existing Rust libraries (`anstyle`, `anstream`, `crossterm`, `indicatif`, `serde`) for terminal mechanics while owning the design language and semantic layer.

---

## Visual Presets

Sartorial provides four first-party presets resolving through a single token authority:

### 1. House (Default Authority)
Quiet, cool, precise, modern, restrained. Designed for almost all Biscuit Logic tools:

```
SARTORIAL

Environment                              READY

Tools                                    9 / 11
──────────────────────────────────────────────
Git             2.51.0     ready
ripgrep         14.1.1     ready
Threadmoth      1.10.0     ready

[I] Install   [/] Find   [?] Help   [Q] Quit
```

### Alternative House Interpretations
Downstream tools can switch styles via flag without changing application semantics:
- `--style black-tie`: Formal, nearly monochrome, elegant, controlled.
- `--style workwear`: Dense, fast, practical, numeric emphasis for operators.
- `--style studio`: Refined, slightly roomier, creative, presentation-friendly.

```bash
# Instant 4-way visual style comparison
cargo run --bin sartorial-demo -- styles

# Full visual regression runway
cargo run --bin sartorial-demo -- runway
```

> **Cardinal Rule**: *Sartorial presets alter presentation, not meaning.*

---

## Core Rule: One Semantic Truth, Multiple Views

```
                     SEMANTIC RESULT (Outcome / Plan / Receipt / TableModel)
                                             |
                   +-------------------------+-------------------------+
                   |                         |                         |
              HUMAN VIEW                 PLAIN VIEW               AGENT VIEW
            (Styled ANSI)               (Pipe-Safe)                 (JSON)
```

The underlying truth is **never** independently scraped or reconstructed by downstream renderers.

---

## Quickstart

Add Sartorial to your `Cargo.toml`:

```toml
[dependencies]
sartorial = "0.2"
```

### 1. Summary Screen

```rust
use sartorial::*;

let screen = SummaryScreen::new("MY-TOOL", Status::Ready)
    .fact("Environment", "Production")
    .fact("Uptime", "99.98%")
    .notice(Notice::info("All systems nominal."))
    .action(Action::open())
    .action(Action::quit());

// Render to terminal (respects NO_COLOR, presets, and terminal dimensions)
print_human(&screen)?;
```

### 2. Honest Progress & Motion

```rust
use sartorial::*;

// Unknown total favors honest elapsed count-up (never fake percentage)
let mut pb = ProgressBar::activity("Checking repository")
    .with_subtask("cargo test")
    .with_elapsed(14);

print_human(&pb)?;

// Known progress derives percentage honestly
let count_pb = ProgressBar::count("Scanning files", 38, 60);
print_human(&count_pb)?;
```

### 3. Consequential Plans (Dry-Run)

```rust
use sartorial::*;

let plan = Plan::new("PATH REPAIR")
    .with_description("Proposed changes")
    .add("C:\\Users\\Matmus\\.cargo\\bin")
    .modify("C:\\Program Files\\Git\\bin")
    .consequence("Existing processes will not inherit this update.")
    .reversible(true)
    .with_action(Action::new('a', "apply", "Apply"))
    .with_action(Action::quit().with_label("Cancel"));

print_human(&plan)?;
```

### 4. Operation Receipts

```rust
use sartorial::*;

let receipt = Receipt::success("PATH UPDATED")
    .change("Added", "1 directory")
    .change("Removed", "0 entries")
    .unchanged("System PATH", "unchanged")
    .guidance("Open a new shell for the changes to take effect.");

print_human(&receipt)?;
```

### 5. Integration with `clap`

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
    let config = cli.sartorial.to_config(); // Configured with preset, color, motion
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

```bash
# Compare all four presets
cargo run --bin sartorial-demo -- styles

# Visual regression runway
cargo run --bin sartorial-demo -- runway

# Proposed dry-run plan
cargo run --bin sartorial-demo -- plan

# Post-operation receipt
cargo run --bin sartorial-demo -- receipt

# Plain pipe-safe mode
cargo run --bin sartorial-demo -- --plain summary

# Machine-readable JSON mode
cargo run --bin sartorial-demo -- --json summary
```

---

## Verification & CI

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
git diff --check
```

See [Biscuit Logic CLI Presentation Standard (BL-CLI-01)](docs/BL_CLI_STANDARD.md) for full design specification.
