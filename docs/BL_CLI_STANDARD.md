# Biscuit Logic CLI Presentation Standard (BL-CLI-01)

**Standard authority:** Biscuit Logic Architecture & Engineering<br>
**Standard version:** 0.2.0<br>
**Rust implementation toolkit:** Sartorial 0.5

This document defines presentation principles for Biscuit Logic command-line
applications. It is a product standard, not a claim that every language,
interaction pattern, or policy below is implemented by Sartorial. The current
Rust crate boundaries and implemented behavior are documented in the
[architecture guide](ARCHITECTURE.md) and [integration guide](INTEGRATION.md).

## 1. Product character

CLI output should be quiet, precise, legible, restrained, technical, and
slightly elegant: a well-made control instrument rather than a colourful
terminal toy.

Avoid oversized banners, borders around every element, colour without meaning,
decorative motion, novelty glyphs, and help output that buries useful context.

## 2. One truth, multiple presentations

Applications own their state, business decisions, schemas, logs, execution,
and operation results. A presentation toolkit can format a semantic view of
that truth; it must not invent or change it.

For Sartorial 0.5, Rust semantic presentation values converge on one
`Document`, rendered as Terminal, Plain, or Markdown. Sartorial's optional
Agent JSON surface is a separate presentation envelope for supported types.
Application `--json` remains application-owned.

## 3. Presentation grammar and identity

In Sartorial, a **Preset** selects grammar such as density, title treatment,
markers, spacing, and component layout. A **Theme** supplies visual identity,
including application colours. Presets and themes change presentation only;
they must not change facts, evidence, statuses, schemas, exit behavior, or
prompt safety.

Sartorial 0.5 provides four presets: House, Black Tie, Workwear, and Studio.
`ResolvedStyle` carries concrete style decisions to renderers.

## 4. Progress must mean something

Progress should communicate only state the application knows:

- Use activity for work with an unknown total; do not invent a percentage.
- Use counts when completed and total work are known; derive the percentage.
- Use a countdown only for a real timed wait.
- Do not show an ETA without sufficient real measurements.
- Keep progress output separate from primary results.

Sartorial 0.5 models activity, count, and countdown states. Invalid count
states are rejected and zero totals remain indeterminate. Its full crate can
drive live terminal progress; the core can render static progress. Plain,
Markdown, and Agent targets remain static.

## 5. Meaning should not depend on colour

Use labels, symbols, and structure as well as colour to communicate state.
Output should remain understandable when colour is absent. Plain output
should avoid ANSI escape sequences and use ASCII-safe presentation.

Sartorial supports colour policy, symbol mode, and explicit capability
resolution. These are presentation capabilities; applications remain
responsible for accessibility of their complete interaction and content.

## 6. Plans, receipts, and honest actions

A plan presents proposed work before it happens. Showing or rendering a plan
must not perform the operation. A receipt describes the operation that
actually happened. Build it from the result, including what did not change
when that matters.

Display an action key only when the application handles that key. Do not
present a dead key as an available interaction. Prompts should fail closed
when interaction is unavailable unless the caller deliberately configures a
fallback.

Sartorial provides Plan, Receipt, Action, Confirm, and Choice presentation
types. The application owns execution, accepted-choice meaning, and side
effects.

## 7. Output-channel honesty

Keep primary results on stdout and progress or diagnostics on stderr. Machine
output should be a clean, explicitly selected application schema, without
human presentation noise.

Sartorial's output helpers route human/plain and Markdown results to stdout,
progress and diagnostics to stderr, and supported Agent JSON to stdout.
Agent-mode prose progress and diagnostics are suppressed. Helpers do not
serialize an application's business JSON.

## 8. Language-neutral contracts

Sartorial's optional wire feature provides versioned presentation contracts:
`sartorial.v0.1` compatibility and the distinct
`sartorial.document.v0.1` serialized Document format. Neither is a universal
application schema. Other languages may adopt this presentation boundary,
but their business contracts remain their own.

## 9. Keep the implementation boundary explicit

The standard describes desired CLI behavior across Biscuit Logic projects.
Sartorial is the Rust implementation, not an automatic enforcement layer for
every application. Application owners still decide their semantic models,
JSON, exit codes, keyboard bindings, logging, and operation safety.

The Sartorial repository's Terrorbats example is dogfood for the presentation
path: the same campaign result uses a product theme with two presets and is
also rendered as Plain and Markdown. The theme and fixture remain example
code; Sartorial does not depend on the Terrorbats application.
