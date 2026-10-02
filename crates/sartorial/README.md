# sartorial

Batteries-included terminal presentation toolkit. Depends on and re-exports
[`sartorial-core`](../sartorial-core/README.md), adding what genuinely needs
runtime terminal behaviour or application integration:

- **Detection**: `RenderContext::detect()` reads the stdout TTY once and
  builds deterministic core `Capabilities` + `ResolvedStyle`.
- **Channel hygiene**: results → stdout, progress/diagnostics → stderr,
  machine JSON → stdout only (`SartorialOutput`).
- **Live progress** (`progress` feature, `indicatif`): honest spinners and
  bars; unknown work never shows fabricated percentages.
- **Interaction** (`interactive` feature, `crossterm`): prompts, keyboard
  grammar, fail-closed non-interactive fallbacks.
- **Clap integration** (`clap`/`completions` features) and shell completions.
- **Wire** (`wire` feature): `sartorial.v0.1` protocol compatibility, agent
  JSON, and the small serialized-`Document` presentation format.
  Application business schemas stay application-owned.
- **Diagnostics** (`diagnostics` feature): structured source-diagnostic →
  `Document` seam, miette-adapter-ready, with no heavy dependencies.

Use `sartorial-core` when you want deterministic presentation; use
`sartorial` when you want terminal behaviour and integrations.

Part of the [Sartorial](../../README.md) workspace (`sartorial 0.5.0`).
See the [integration guide](../../docs/INTEGRATION.md).
