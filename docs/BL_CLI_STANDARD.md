# Biscuit Logic CLI Presentation Standard (BL-CLI-01)
**Standard Authority**: Biscuit Logic Architecture & Engineering<br>
**Version**: 0.1.0<br>
**Implementation Toolkit**: Sartorial (`sartorial`)

---

## 1. Vision & Core Philosophy

Future Biscuit Logic CLI programs must not repeatedly spend engineering time designing typography, colours, tables, status views, errors, prompts, keyboard conventions, machine output modes, or progress motion.

A future engineer or AI agent should be able to declare:
> **"Use Sartorial."**
and inherit sensible, professional presentation and interaction behaviour out of the box.

### Product Character
Sartorial's visual character is:
- **QUIET**
- **PRECISE**
- **LEGIBLE**
- **RESTRAINED**
- **TECHNICAL**
- **SLIGHTLY ELEGANT**

Think: **well-made precision control instrument**, not a colourful terminal toy.

### Anti-Patterns (Forbidden)
- Giant ASCII art logos or loud banners.
- Excessive borders or boxes enclosing every single element.
- Rainbow colouring or decorative animations without state.
- Novelty glyph overload.
- Huge multi-page help dumps dumped into AI agent context.

> [!IMPORTANT]
> **Cardinal Rule of Presentation**:
> **Sartorial presets alter presentation, not meaning.**
> Applications never change their semantic state, JSON contracts, exit codes, or prompt safety depending on visual presets.

---

## 2. Core Architectural Rule: One Truth, Multiple Views

One semantic result possesses several renderers:

```
                     SEMANTIC RESULT (Outcome / Plan / Receipt / TableModel)
                                             |
                   +-------------------------+-------------------------+
                   |                         |                         |
              HUMAN VIEW                 PLAIN VIEW               AGENT VIEW
            (Styled ANSI)               (Pipe-Safe)                 (JSON)
```

The underlying application truth **MUST NOT** be independently reconstructed or parsed across renderers.
- **One semantic authority.**
- **Multiple views.**

---

## 3. Sartorial Preset System

Sartorial provides exactly **four first-party presets**, resolving through a single token authority (`ResolvedStyle`):

| Preset | Character | Primary Accent | Density | Typical Use |
| :--- | :--- | :--- | :--- | :--- |
| **House** *(Default Authority)* | Quiet, cool, precise, modern, restrained | Restrained Cyan/Slate | Standard | Standard for almost all tools |
| **Black Tie** | Formal, nearly monochrome, elegant, controlled | Bright White / Mono | Standard | Release tools, security, administrative audit |
| **Workwear** | Dense, fast, practical, operator-focused | Industrial Amber/Yellow | Compact | High-frequency developer workflows, operators |
| **Studio** | Refined, slightly expressive, presentation-friendly | Magenta / Violet | Roomy | Demos, showcases, onboarding guides |

### Token Resolution Order:
```
House Default Preset -> Selected Preset -> Accessibility Overrides -> Explicit Caller Overrides -> ResolvedStyle
```
Components consume `ResolvedStyle` from `RenderContext` rather than querying which preset is active.

---

## 4. BL Motion Standard

Motion in Sartorial is an honest representation of operational state. A tool doing real work should not appear dead, but movement must never fabricate progress.

### Semantic Progress Modes:
1. **Activity**: Use when total work is **unknown**. Favours an honest **elapsed count-up**. Never invents fake percentages.
   ```
   ◐ Checking repository · cargo test                 14s
   ```
2. **Count**: Use when completed and total items are known.
   ```
   Scanning files                         38 / 60
   ```
3. **Percent**: Use only when percentage is genuinely derived from known progress.
   ```
   Building        ━━━━━━━━━━━╸━━━━━━     63%
   ```
4. **Countdown**: Use only for real future timing events. Never used decoratively.
   ```
   Retrying connection in 17s
   ```
5. **Rate**: Use when meaningful throughput exists.
   ```
   Downloading artifacts      84 MB / 140 MB      11 MB/s
   ```
6. **ETA**: Derived only when sufficient real samples exist; prefer no ETA to a nonsense ETA.

### Preset Motion Character:
- **House**: Restrained spinner (`◐`), clean progress bar (`━━━━╸───`), honest elapsed time.
- **Black Tie**: Almost static, tiny pulse dot (`•`), numeric count and elapsed time.
- **Workwear**: Compact numeric format prioritizing screen real estate:
  ```
  [38/60] 63%  00:14  cargo test
  ```
- **Studio**: Smoothest motion, expressive tick, refined progress bar.

### Motion Policy (`MotionMode`):
- `Auto`: Animate only on attended interactive TTYs. Disabled in CI, non-TTY pipes, and redirected streams.
- `Always`: Force animation (testing/demos).
- `Never`: Emit static snapshots.
- **JSON Guard**: Agent JSON output **MUST NEVER** contain spinner frames or animation tokens.

---

## 5. Visual & Typographic Hierarchy

The visual hierarchy guides the eye with weight and alignment rather than loud colours:

| Level | Component | Style & Treatment | Purpose |
| :--- | :--- | :--- | :--- |
| **L1** | **PROGRAM TITLE** | Bold, Accent color, Uppercase | Visually strongest textual identifier. Compact. |
| **L2** | **SECTION** | Bold, normal weight/accent, subtle padding | Clear division of concerns. Quieter than Title. |
| **L3** | **LABEL** | Subdued / Dim (`BrightBlack`) | Contextual key (e.g. `Environment`, `Path`). |
| **L4** | **VALUE** | High legibility (Normal / White) | The substantive data point. |
| **L5** | **PRIMARY STATUS** | Symbol + Uppercase label (`✓ READY`, `× FAILED`) | Immediately scannable state. |
| **L6** | **SECONDARY NOTE** | Muted / Dim | Explanations, units, references (`ms`, `GB`). |
| **L7** | **KEY COMMAND** | Bracketed hotkey `[I] Install   [Q] Quit` | Teaches interaction directly in view footer. |
| **L8** | **ERROR** | Bold Red headline, structured body | What happened, why, and what next. |

### Color Rules & Redundancy
Colour is never used as the sole conveyor of meaning. Status glyphs ensure legibility even in monochrome terminals or with colour blindness:

| Semantic State | Unicode Glyph | ASCII Fallback | Color Treatment | Meaning |
| :--- | :---: | :---: | :--- | :--- |
| **Ready** | `✓` | `[OK]` | Bold Green | Succeeded, verified, operational |
| **Attention** | `!` | `[!]` | Bold Yellow | Warning, incomplete, attention needed |
| **Failed** | `×` | `[X]` | Bold Red | Error, verification failed |
| **Running** | `●` | `[*]` | Bold Cyan | In-flight execution |
| **Pending** | `○` | `[.]` | Muted Gray | Queued, waiting |
| **Skipped** | `–` | `[-]` | Muted Gray | Intentionally bypassed |

---

## 6. Operation UX: Plan & Receipt

### 1. Plan (Dry-Run Presentation)
For consequential operations **before** they happen. Presents intent without executing:
```
PATH REPAIR

Proposed changes

+ C:\Users\Matmus\.cargo\bin
~ C:\Program Files\Git\cmd

User PATH will be updated; system PATH remains untouched.
Existing terminal processes will not inherit this update.

[A] Apply   [D] Details   [Q] Cancel
```
Supports additions (`+`), removals (`-`), modifications (`~`), consequences, warnings, and reversibility indicators.

### 2. Receipt (Post-Operation Presentation)
For after a state-changing operation completes:
```
✓ PATH UPDATED

Added       1 directory
Removed     0 entries
Duplicates  0

Open a new shell for the change to take effect.
```
Truthfully reflects what changed, what remained unchanged, warnings, next actions, and evidence handles.

---

## 7. Keyboard Interaction & Safety Grammar

Interactive tools must adhere strictly to the BL standard keyboard mapping:

| Key | Canonical Action | Description |
| :--- | :--- | :--- |
| `Enter` | **Open / Accept** | Activate selected item or confirm default choice |
| `Esc` | **Back / Cancel** | Step back one screen level or abort |
| `↑` / `↓` | **Navigate** | Move highlight up or down |
| `←` / `→` | **Switch** | Toggle between related views or tabs |
| `Space` | **Select / Toggle** | Toggle checkbox or selection |
| `/` | **Find / Filter** | Open inline search filter |
| `?` | **Help** | Display contextual keybindings & explanations |
| `D` | **Details** | Open deep progressive disclosure view |
| `R` | **Retry / Refresh** | Re-run check or refresh data |
| `Q` | **Quit** | Gracefully exit application |

### Interaction Authority & Non-Interactive Safety
- `Config::InteractiveMode` (`Auto`, `On`, `Off`) is the single authority governing interactivity.
- `Auto` verifies both `stdin` and `stdout` are interactive TTYs.
- `Confirm` and `Choice` are strictly **fail-closed** when non-interactive. They never silently assume confirmation or selection unless the caller explicitly configures a non-interactive fallback.
- `Choice` redraw uses event-driven blocking reads with wrapped-row accounting, eliminating idle reprint spam and cursor drift.

---

## 8. Output Channels & Hygiene

Output channels must be deliberate and segregated:
- **STDOUT**: Primary operation results, final human output, and clean JSON.
- **STDERR**: Progress diagnostics, animation frames, non-fatal warnings, and notices.
- **Agent JSON (`--json`)**: Emitted exclusively to STDOUT. Completely free of ANSI sequences, progress animation, or prose warnings.

---

## 9. Semantic Exit Contract

Downstream applications consume typed exit codes adhering to cross-platform conventions:
- `0`: Success (POSIX standard).
- `2`: Invalid request / command line usage error.
- `3`: Requested capability or environment dependency unavailable.
- `4`: Verification or primary operation failed.
- `130`: Interrupted or cancelled by user (`SIGINT`).

---

## 10. Long-Line Sanity & Table Boundaries

No absurd path, URL, or compiler error should break terminal presentation:
- Unicode display width accuracy is enforced across all cells and strings.
- Long paths are truncated from the middle (`truncate_path`), preserving both directory root and target filename (`C:\Users\...\threadmoth.exe`).
- Tables strictly bound total rendered width to terminal width. On absurdly tiny terminals (e.g. width < 15), columns shrink down to 1 cell and rightmost non-primary columns drop gracefully, guaranteeing the table line never exceeds the claimed terminal width.

---

## 11. Paging Policy

Long interactive output should be pleasant without trapping machine pipelines:
- `PagerMode::Auto`: Pages only when stdout is an attended TTY and output exceeds screen height.
- **Machine Safety**: Plain output (`--plain`) and Agent JSON (`--json`) are **never** piped into an interactive pager.

---

## 12. Configuration Provenance

Applications explaining origin values can attach provenance without polluting generic facts:
- `Flag` (e.g. `--endpoint`)
- `Environment` (e.g. `SARTORIAL_COLOR`)
- `ProjectConfig` (e.g. `biscuit.toml`)
- `UserConfig` (e.g. `~/.config/biscuit.json`)
- `Default`
- `Other`

---

## 13. Accessibility Standard

Accessibility is an override layer across all presets, never a separate visual skin:
- **Colour Redundancy**: Meaning is always conveyed via symbols and text alongside colour.
- **Reduced Motion**: Disables animation frames; preserves static counts, elapsed time, and percentages.
- **Plain Mode**: Pure ASCII glyphs, zero ANSI escape codes, screen-reader friendly.

---

## 14. When an Application May Deviate

An application may deviate from Sartorial defaults **only** when:
1. Delivering specialized visual data rendering (e.g., a hex editor, full visual diff view, or terminal charting canvas).
2. The user has explicitly selected custom branding colors (configured via `Config::with_accent(...)`).

Standard CLI output, inventory lists, status reports, and error messages should **not** invent new visual layouts or color schemes.
