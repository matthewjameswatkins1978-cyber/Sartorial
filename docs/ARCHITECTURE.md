# Sartorial v0.5 architecture

Sartorial separates application meaning from command-line presentation. The
application remains the authority for its state, decisions, business schemas,
and operation results. Sartorial receives semantic values and presents them.

## Two crates, two jobs

```text
Your application
      │ semantic values
      ▼
sartorial-core
  semantic model → Document → Terminal / Plain / Markdown
      ▲
      │ re-exported and extended by
sartorial
  detection · output channels · live progress · interaction · integrations
```

### `sartorial-core`

The core crate provides semantic presentation types, `Presentable`, the
`Document` intermediate representation, preset/theme resolution, explicit
capabilities, and three static renderers. It has no live-terminal detection or
animation. Rendering decisions depend on the supplied context and document,
which keeps the core usable in tests and other deterministic contexts.

Direct dependencies are `anstyle`, `unicode-width`, and `serde`. In particular,
the core does not bring in `crossterm`, `indicatif`, `clap`, or `serde_json`.

### `sartorial`

The full crate depends on and re-exports core. It adds environment capability
detection, stdout/stderr routing helpers, live progress, prompts and keyboard
interaction, Clap support, shell completions, optional diagnostics, and the
optional `wire` feature. Default features enable `terminal`, `progress`,
`interactive`, `clap`, and `wire`; consumers can disable defaults and select
features explicitly.

The `sartorial` CLI executable is built only when the `clap` and `wire`
features are enabled. Non-Rust programs can use its protocol surface where
appropriate, but this does not make Sartorial the authority for their business
schemas.

## One semantic path for static formats

Built-in semantic values and custom `Presentable` implementations converge on
one `Document`. Terminal, Plain, and Markdown renderers consume that document.
They may differ in layout, text treatment, glyphs, or emphasis, but they do
not independently decide the underlying result or evidence.

The Plain renderer produces no ANSI styling and uses ASCII-safe output for
redirected or constrained environments. The Markdown renderer is static; it
does not animate. The core does not produce HTML, PDF, GUI, or live terminal
views.

## Preset, Theme, resolved style

- A **Preset** selects presentation grammar: density, title casing, markers,
  spacing, rules, and component treatment.
- A **Theme** supplies visual identity, such as accent, status, evidence, and
  code colours.
- **ResolvedStyle** holds concrete style decisions for rendering.

An explicit application theme composes with a preset and is retained when the
preset changes. Presets and themes affect presentation; neither changes
application data, status, evidence, or operation behavior.

## Capabilities and determinism

Core consumes explicit `Capabilities`, including width, TTY state, colour,
Unicode, hyperlinks, motion, and interactivity. It does not inspect process
state while rendering.

In the full crate, `RenderContext::detect()` captures terminal-related
environment information and resolves style. Context target changes resolve
again from the captured environment snapshot, so switching between Human,
Plain, Markdown, and Agent targets does not accumulate stale target overrides.
Plain, Markdown, and Agent contexts are static, without ANSI colour or motion.

Capability detection is a convenience boundary, not a claim that every
environment is inferred perfectly. Callers and tests can use explicit
configuration and TTY values where they need deterministic control.

## Output targets and channels

The render target identifies the intended presentation: Human, Plain,
Markdown, or Agent. `SartorialOutput` provides separate entry points rather
than silently widening a format:

| Output | Entry point | Channel |
|---|---|---|
| Human or Plain result | `print_result` | stdout |
| Markdown document | `print_markdown` | stdout |
| Progress or diagnostic | `print_progress` / `print_diagnostic` | stderr |
| Supported Sartorial agent JSON | `print_agent_json` | stdout |

`print_result` rejects Markdown and Agent contexts, directing callers to the
matching method. Progress and diagnostic helpers suppress their prose in
Agent mode. They do not automatically redirect or serialize an application's
own JSON; the application owns that contract.

## Agent JSON and the wire protocol

Agent JSON is a separate, optional `wire`-gated surface for supported
Sartorial types. Its schema version constant is `sartorial.v0.1`. The
serialized `Document` format has its own version identifier,
`sartorial.document.v0.1`.

These are presentation protocol contracts. They do not replace an
application's public JSON format. For machine output, serialize the
application's own data directly and keep it free of human presentation text.

## Progress semantics

The semantic progress API distinguishes activity with unknown total work,
counted work with known totals, and countdowns for real timed waits. A count's
percentage is derived from its counts; invalid or contradictory counts are
rejected. A zero total remains indeterminate. The core can render progress
states statically; the full crate can drive live terminal progress when the
feature and target allow it.

TTY animation is an optional presentation of ongoing work, not a measurement
of completion. Plain, Markdown, and Agent targets stay static. Progress output
uses stderr so a command's primary result can remain clean on stdout.

## Dogfood example

`crates/sartorial/examples/terrorbats.rs` demonstrates a campaign report with
a Terrorbats-specific theme and Workwear or Studio grammar, then renders the
same semantic result as Plain and Markdown. The theme and example data live in
the Sartorial repository; the framework itself does not depend on Terrorbats.

## Current boundaries

Sartorial is a presentation toolkit. It does not own application persistence,
business validation, logging, execution, or the truth of a result. It does not
promise a universal machine schema for applications, and it does not turn
static `Document` rendering into a live UI. Keep those responsibilities at
their owning boundary.
