# Biscuit Logic CLI Presentation Standard (BL-CLI-01)
**Standard Authority**: Biscuit Logic Architecture & Engineering<br>
**Version**: 0.1.0<br>
**Implementation Toolkit**: Sartorial (`sartorial`)

---

## 1. Vision & Core Philosophy

Future Biscuit Logic CLI programs must not repeatedly spend engineering time designing typography, colours, tables, status views, errors, prompts, keyboard conventions, or machine output modes.

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

---

## 2. Core Architectural Rule: One Truth, Multiple Views

One semantic result possesses several renderers:

```
                     SEMANTIC RESULT (Outcome / ErrorModel / TableModel)
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

## 3. Visual & Typographic Hierarchy

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

## 4. Spacing, Alignment & Tables

- Tables omit heavy vertical grid borders. They feature:
  - Header line with optional right-aligned count/badge (`Tools       9 / 11`).
  - Subtle single rule (`──────────` in Unicode, `----------` in ASCII).
  - Clean column alignment with 4 spaces between columns (compressed to 2 in narrow mode).
  - Semantic status colouring on status columns.
- Separators use restrained single rules, never double-lines (`===`) or heavy boxes.

---

## 5. Keyboard Interaction Grammar

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

### Mnemonic Actions
Applications may add obvious mnemonic hotkeys:
- `[I]` Install
- `[U]` Update
- `[V]` Verify
- `[F]` Find

Every screen providing interactive input **must** teach available keys via an `ActionBar` footer:
```
[Enter] Open   [/] Find   [R] Refresh   [?] Help   [Q] Quit
```

---

## 6. Context Economy & Machine / Agent Mode

Sartorial is explicitly engineered for **Context Economy**: reducing unnecessary token consumption by AI agents while preserving debugging power.

### Principles:
1. **Minimum Sufficient Evidence First**: Return only the essential summary, location, and reference handle in the primary response.
2. **Opt-in Progressive Disclosure**: Full raw build logs or compiler transcripts must not be dumped into the primary output. Provide an evidence handle (e.g. `clippy:log-042`) and let the agent or human opt in with `[D] Details`.
3. **Structured Agent JSON (`--json`)**:
   - Strict adherence to the canonical Sartorial Agent schema version (`schema_version: "sartorial.v0.1"`).
   - Zero ANSI escape sequences (`\x1b`).
   - Typed arrays instead of decorative prose paragraphs.
   - Explicit next actions exposed as machine-readable string ID arrays (`"next_actions": ["retry", "details"]`).
4. **Interaction Authority & Fail-Closed Safety**:
   - `Config::InteractiveMode` (`Auto`, `On`, `Off`) is the single authority for interactivity across all components.
   - `Auto` requires both stdin and stdout to be interactive streams.
   - Prompts (`Confirm`, `Choice`) are strictly **fail-closed** when non-interactive. They never silently assume confirmation or selection unless the caller explicitly configures a non-interactive fallback.

---

## 7. Error Design Standard

Every Sartorial error structure must answer three fundamental questions:
1. **WHAT HAPPENED?** (Clear, concise failure statement)
2. **WHY?** (Root cause established by the application; **preserve uncertainty** if undetermined)
3. **WHAT CAN I DO NEXT?** (Actionable next steps)

```
THREADMOTH NOT VISIBLE

Threadmoth is installed, but this process cannot resolve it.

Found in cargo bin directory, but missing from PATH:
C:\Users\Matmus\.cargo\bin\threadmoth.exe

[R] Recheck   [D] Details   [Q] Quit
```

> **Important**: Never manufacture or hallucinate causes that the application has not conclusively verified. If the cause is unknown, state that the cause is undetermined.

---

## 8. Progress Reporting Standard

When background tasks or long-running computations occur:
- Progress must carry tangible state:
  ```
  Checking repository… cargo test 47s
  ```
- No ornamental spinners with zero information.
- Clean degradation: In non-TTY environments (redirected pipes, CI, JSON mode), spinners are disabled and emit a clean single-line log or structured JSON snapshot.
- Terminal state (cursor and raw mode) must be unconditionally restored upon completion or interruption.

---

## 9. Responsiveness Across Terminal Widths

- **Narrow (< 60 columns)**:
  - Key-value lists automatically stack vertically (`Key:\n  Value`).
  - Table column gaps compress from 4 to 2 spaces.
  - Secondary details are dropped or truncated before producing unreadable line wrapping.
- **Normal (60..=100 columns)**: Standard balanced layout.
- **Wide (> 100 columns)**: Generous spacing with optional extended metadata.

---

## 10. When an Application May Deviate

An application may deviate from Sartorial defaults **only** when:
1. Delivering specialized visual data rendering (e.g., a hex editor, full visual diff view, or terminal charting canvas).
2. The user has explicitly selected custom branding colors (configured via `Config::with_accent(...)`).

Standard CLI output, inventory lists, status reports, and error messages should **not** invent new visual layouts or color schemes.
