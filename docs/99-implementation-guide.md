---
status: DRAFT
---

# 99 — Implementation Guide

This document ties the specification together for implementers (human or AI agent). It gives
a suggested build order, module boundaries, an acceptance-criteria template, and the "first
vertical slice" that proves the architecture end to end.

Read first: [00-README.md](./00-README.md) (index), [01-vision.md](./01-vision.md),
[02-glossary.md](./02-glossary.md), [03-design-principles.md](./03-design-principles.md).

## How to implement a feature from this spec

1. Identify the subsystem (UML model, analytics, UI, MCP) from the index.
2. Read the normative spec doc(s) for that subsystem — every `REQ-*` and `INV-*` is a
   contract.
3. Read the linked research doc(s) for background (especially algorithm references).
4. Implement; for each `REQ-*`, write a test that asserts the requirement holds.
5. Run `cargo test` / `vitest`; run clippy/oxlint (zero warnings, per repo rules).
6. Verify against the acceptance criteria below.

## Module boundaries (from [architecture/01](./architecture/01-system-architecture.md))

```
smith-core    — domain types, no IO
smith-model   — model API, invariants, command stack, petgraph projection
smith-store   — SQLite persistence, migrations, closure table, FTS5
smith-analysis — RTM, adjacency, orphan/coverage, graph algorithms (read-only)
smith-mcp     — MCP server (rmcp), tools/resources/prompts
smith-ui-bridge — Tauri command handlers (webview-facing API)
smith-app     — binary: wires runtime, DB, MCP server, Tauri
```

Dependencies are one-way (core ← model ← store/analysis ← mcp/bridge ← app). No cycles.
`smith-core` and `smith-model` are pure-Rust and unit-testable without IO.

## Suggested build order (vertical slices, not layers)

Each slice is independently demoable and tests the architecture through a new axis. Do not
build "all of the model, then all of persistence, then all of UI" — that defers integration
risk.

### Slice 0 — Skeleton (1–2 days)
- Empty Tauri 2 + React app launches and shows a blank window.
- `smith-app` crate wired; tokio runtime starts; logs emit.
- `cargo` workspace + `pnpm` workspace; clippy/oxlint/`cargo test`/`vitest` green.
- **Acceptance:** `cargo run` opens a window. CI runs on all pushes.

### Slice 1 — Model core + SQLite (round-trip)
- `smith-core`: `ElementId`, metaclass enums, `Multiplicity`, `Visibility`.
- `smith-store`: SQLite schema ([architecture/03](./architecture/03-persistence.md) §SQLite
  schema); migrations runner; closure-table maintenance.
- `smith-model`: create/get/rename/reparent/delete for Package + Class + Association +
  Generalization; command stack (undo/redo); in-memory projection hydrated from SQLite.
- **Tests:** every `REQ-PERS-*` and `REQ-MM-*` has a test. Round-trip: create model, close,
  reopen, hydrate, assert equal.
- **Acceptance:** a `#[test]` creates a 3-class hierarchy, closes the DB, reopens, and the
  ownership tree + relationships match.

### Slice 2 — First diagram (class) on canvas
- `smith-ui-bridge`: Tauri commands for `get_diagram`, `add_shape`, `move_shape`.
- Frontend: React shell, Konva canvas, class-shape renderer (3-compartment box), select tool,
  drag-to-move, inline rename.
- `smith-model`: `Diagram`, `ShapeView`, `EdgeView` per
  [uml-model/06](./uml-model/06-diagram-interchange.md).
- Diagram-as-view invariant: deleting a diagram deletes zero elements (test it).
- **Acceptance:** user creates a package, adds a class diagram, drops two classes, draws an
  association, renames a class (updates both shapes showing it), deletes the diagram (classes
  survive in the explorer).

### Slice 3 — MCP server (first tools)
- `smith-mcp`: rmcp on axum, Streamable HTTP on `127.0.0.1`; discovery file.
- Implement `initialize`, `tools/list`, and the first tools: `model.createClass`,
  `model.createAssociation`, `model.get`, `model.listChildren`, `search.elements`.
- **Acceptance:** register Smith in Claude Code; an agent creates a class via MCP and the UI
  updates live (single model API, shared undo).

### Slice 4 — Traceability + analytics
- Trace relations ([architecture/04](./architecture/04-traceability-relations.md)).
- `Requirement`, `TestCase` elements ([uml-model/04](./uml-model/04-requirements-and-tests.md)).
- `smith-analysis`: RTM, orphan/coverage detection, cycle detection (incremental).
- Issue panel UI ([ui-ux/04](./ui-ux/04-search-and-analysis-ui.md) §Issue panel).
- **Acceptance:** create a requirement with no `«satisfy»` → D1 defect appears live in the
  issue panel and via `analysis.issues` tool; add a use case + `«satisfy»` → defect clears.

### Slice 5 — The gizmo + the rest of the diagrams
- Context-sensitive gizmo ([ui-ux/02](./ui-ux/02-canvas-and-interaction.md) §Gizmo) — radial,
  marking-menu, keyboard.
- Use case, activity, sequence, state machine diagram kinds
  ([uml-model/03](./uml-model/03-behavioral-diagrams.md)).
- Auto-layout (ELK in a Web Worker) for class + activity.
- **Acceptance:** on each diagram kind, the gizmo offers the right element kinds; an agent
  can derive a use case from a requirement via the `deriveUseCases` prompt end to end.

### Slice 6 — Polish & remaining diagram kinds
- Component, composite, deployment, package, profile, object, communication, timing,
  interaction overview diagrams.
- Design system pass: dark mode, contrast tests ([ui-ux/01](./ui-ux/01-design-system.md)
  REQ-DS-006), motion tokens, accessibility pass.
- Search UI, RTM view, adjacency heat-map, dashboard.
- Export (SVG/PDF/XMI/JSON).

## Acceptance-criteria template

For every feature, the implementer writes acceptance criteria that trace to spec `REQ-*` IDs:

```
Feature: <name>

Acceptance:
- [ ] REQ-<ID>: <one-line restatement> — verified by <test name or manual step>.
- [ ] REQ-<ID>: ...
- [ ] Invariants INV-<ID> hold after the feature runs.
- [ ] No new defects introduced (analysis.issues returns no new HIGH).
- [ ] Works via UI AND via MCP tool (P3 — agents and humans are peers).
- [ ] Undoable (Cmd/Ctrl+Z reverses; batch.undo reverses).
- [ ] clippy/oxlint/`cargo test`/`vitest` green (zero warnings).
```

## Cross-cutting acceptance (every slice must keep these green)

- `[X]` Ownership tree is acyclic + connected (INV-MM-001, INV-MM-002).
- `[X]` Every diagram is a view — deleting it removes zero model elements (REQ-DI-002).
- `[X]` Model API is single-writer; UI and MCP share the undo stack (P3, REQ-ARCH-003).
- `[X]` MCP server binds 127.0.0.1 only (REQ-MCP-002).
- `[X]` No panic crosses the model API boundary (REQ-ARCH-018).
- `[X]` Performance budgets ([architecture/03](./architecture/03-persistence.md) §Performance,
  [analytics/01](./analytics/01-rtm.md) REQ-RTM-005) hold.

## Anti-patterns to refuse in review

- Storing model facts (name, attribute, multiplicity) on the diagram — violates P1.
- A UI capability with no MCP equivalent (or vice versa) — violates P3.
- Re-deriving an algorithm instead of referencing it — violates P6.
- A destructive operation that bypasses undo — violates P9.
- Color carrying meaning without a shape/text backup — violates REQ-DS-002.
- A new serialization format / query language when an existing one works — violates P5.

## Testing strategy

- **Unit tests** in each crate (`#[test]` / `#[tokio::test]`). `smith-core`/`smith-model`
  are 100% pure-Rust unit-testable.
- **Property tests** (`proptest`) for: ownership-tree invariants under random
  create/reparent/delete sequences; closure-table consistency; ID uniqueness.
- **Integration tests** (`smith-app` tests): full-stack — model API + store + projection,
  asserting round-trip and incremental defect updates.
- **Frontend tests** (Vitest): canvas interactions (gizmo open/select, connection drop,
  marquee); inspector edits; explorer reparent.
- **MCP conformance**: a test harness that connects to the running MCP server as a client and
  exercises every tool (input validation, error codes, undo).
- **Contrast test**: automated WCAG assertion on design tokens (REQ-DS-006).

## Open questions across the spec (consolidated)

Each subsystem doc has an "Open questions" section. Before implementation starts, the team
SHOULD resolve these and move the docs from DRAFT to STABLE. The most consequential:

1. `.smith` single file vs. directory ([architecture/03](./architecture/03-persistence.md)).
2. Exact accent hue + category-tint default ([ui-ux/01](./ui-ux/01-design-system.md)).
3. Gizmo gesture threshold ([ui-ux/02](./ui-ux/02-canvas-and-interaction.md)).
4. Whether `«realize»` allows inverse direction
   ([architecture/04](./architecture/04-traceability-relations.md)).
5. Timing/Communication/Interaction Overview polish level
   ([uml-model/03](./uml-model/03-behavioral-diagrams.md)).
