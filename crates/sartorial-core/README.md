# sartorial-core

Tiny deterministic presentation engine underneath Sartorial. Applications own
truth; this crate owns presentation — and nothing else.

- **Semantic model**: `Outcome`, `Status`, `Fact`, `Evidence`, `Notice`,
  `Action`, `TableModel`, `Plan`, `Receipt`, `ErrorModel`, `ProgressState`.
- **`Document` IR + `Presentable`**: every semantic object converts once;
  renderers consume only the document, so meaning can't drift per target.
- **`Preset` (grammar) + `Theme` (identity)**: layout behaviour and visual
  identity compose freely; applications build themes with `Theme::builder`.
- **Explicit `Capabilities`**: width, TTY, colour, Unicode, hyperlinks,
  motion, interactivity. Core never sniffs the environment while rendering.
- **`ResolvedStyle`**: one resolution stage; renderers are pleasantly stupid.
- **Renderers**: terminal (ANSI), plain (pipe-safe ASCII), Markdown
  (issues, PRs, agent handoffs). No HTML/PDF/GUI — Markdown earns its place
  because it is actually used.

Dependencies: `anstyle`, `unicode-width`, `serde`. No `crossterm`, no
`indicatif`, no `clap`, no `serde_json`. Verify with
`cargo tree -p sartorial-core`.

Part of the [Sartorial](../../README.md) workspace (`sartorial-core 0.5.0`).
