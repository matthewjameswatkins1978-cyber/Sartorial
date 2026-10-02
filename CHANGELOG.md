# Changelog

## 0.5.0 — 2026-10-02

First public release of the new architecture. Two crates, one product:
`sartorial-core 0.5.0` and `sartorial 0.5.0`.

### Architecture

- New `sartorial-core`: a tiny deterministic presentation engine
  (semantic model, `Document`, resolved styling, renderers) with no
  live-terminal machinery. Depends only on `anstyle`, `unicode-width`,
  and `serde`.
- One presentation `Document` with the `Presentable` trait: every
  semantic object converts once, then Terminal, Plain, and Markdown
  renderers format the same structure instead of each type drifting
  through per-target render paths.
- Preset / Theme separation: `Preset` is layout grammar (House,
  Black Tie, Workwear, Studio); `Theme` is application visual identity,
  built by applications with `Theme::builder`. Themes compose with every
  preset and survive preset changes.
- Explicit deterministic `Capabilities` (width, TTY, colour, Unicode,
  motion, interactivity) resolved once; renderers never sniff the
  environment while rendering.

### Rendering and output

- First-class Markdown renderer for issues, PRs, release notes, and
  agent handoffs (headings, tables, lists, GitHub callouts).
- Output-channel hardening: results to stdout, progress/diagnostics to
  stderr, agent JSON to stdout only; plain output is ASCII-safe and
  pipe-safe.
- Progress honesty: unknown work is activity, never a fabricated
  percentage; known totals use real counts; contradictions and zero
  totals stay indeterminate.
- Clean human/agent output boundaries; the wire protocol became optional
  (`wire` feature) with the versioned `sartorial.v0.1` envelope plus a
  versioned `sartorial.document.v0.1` presentation wire format.

### Project

- Terrorbats dogfood example: one semantic result through two presets,
  a custom theme, Plain, and Markdown.
- MIT licence, product documentation (`docs/ARCHITECTURE.md`,
  refreshed `docs/INTEGRATION.md`), Sartorial mark, and a README with
  visual Harbour examples.
- Verified on Windows, Linux, and macOS.
