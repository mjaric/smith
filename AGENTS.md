# Repository Guidelines

## Project Overview

**Smith** is a desktop UML modeling tool: a Rust core with a Tauri-hosted React/TypeScript
frontend. Diagrams are **views** into one authoritative model — they never own elements; the
model's package ownership tree does. Smith embeds an **MCP server on `127.0.0.1`** so AI agents
(Claude Code, Codex, Gemini CLI, web chat) mutate the same model as the human user — an
agent-native modeling tool. It extends UML with SysML-inspired **Requirement** and **TestCase**
elements plus a traceability relation set, making the chain
`Requirement → UseCase → Activity → Sequence → Class` first-class and continuously analyzed
(RTM, adjacency, orphan/coverage detection).

**Repo state: this repository contains only the specification (`docs/`). No source code, tests,
build config, or CI exists yet** (`docs/CONTRIBUTING.md` §0: "This directory is **the
specification** — no implementation exists yet"). Two modes of work, governed by
`docs/CONTRIBUTING.md`: **Mode A — implement** from the spec; **Mode B — refine** the spec.
Everything "planned" below is documented but unbuilt.

## Architecture & Data Flow

Single OS process (`docs/architecture/01-system-architecture.md`, `06-tech-stack.md`):

- **Tauri 2 webview** — React 19 + strict TS; Konva.js canvas, explorer, inspector, RTM/issue
  panels. Talks to the core over Tauri commands/events.
- **Model API** — the **single writer**: all mutations serialize through one async task owning
  the write lock (REQ-ARCH-002). UI and MCP clients call the same API and share one undo stack
  (REQ-ARCH-003). Command-pattern undo, invariant enforcement, incremental defect/coverage
  recomputation.
- **SQLite (`rusqlite`, bundled) is the store of record** — a single `.smith` file; WAL;
  ownership closure table; FTS5. Graph databases were explicitly rejected (server-based ones
  violate single-binary; Kùzu archived; Sled beta; see `docs/research/03`). The **`petgraph`
  in-memory projection is a cache**, rebuilt from SQLite, never the source of truth.
- **Analysis engine** — read-only analytics (RTM, adjacency, SCC, centrality, reachability) on
  cheaply cloned petgraph snapshots so long analyses don't block the writer.
- **MCP server** — `rmcp` (official Rust SDK) on axum, Streamable HTTP, loopback-only with
  ephemeral port announced via `~/.smith/mcp.json`; 83 tools in 8 domains (`model.*`,
  `diagram.*`, `analysis.*`, `graph.*`, `trace.*`, `search.*`, `project.*`, `batch.*`).

**Mutation data flow:** Tauri command or MCP `tools/call` → Model API validates, builds a
`Command`, applies it to the projection → writes through to SQLite in one transaction (element
row + closure table + FTS5 commit atomically with the projection) → one coalesced change-event
bundle broadcasts → Tauri event re-renders the webview; MCP emits
`notifications/resources/updated`.

**Read flow:** webview Tauri command → read-locked snapshot from Model API → JSON render.
**Search:** FTS5 in-transaction, composable with structured filters (closure-table subtree,
stereotype, defect state); graph queries run on the projection via `smith-analysis`.

Planned Rust crate layout (`docs/99-implementation-guide.md` §Module boundaries) — strictly
one-way dependencies, no cycles:

```
smith-core      — domain types (ElementId, metaclass enums, Multiplicity), no IO
smith-store     — SQLite schema, migrations, closure table, FTS5
smith-model     — Model API, invariants, command stack, petgraph projection
smith-analysis  — RTM, adjacency, orphan/coverage (read-only)
smith-mcp       — MCP server: tools/resources/prompts
smith-ui-bridge — Tauri command handlers
smith-app       — binary wiring runtime, DB, MCP, Tauri
```

## Key Directories

Everything is in `docs/` (spec status via frontmatter banner: `DRAFT` < `REVIEW` < `STABLE`):

| Path | Purpose |
|------|---------|
| `docs/00-README.md` | Spec index: reading order + full document map. |
| `docs/CONTRIBUTING.md` | **Read first.** Two work modes, reading order, load-bearing decisions, conventions. |
| `docs/AGENTS.md` | **Reverse-index**: every `REQ-*`/`INV-*`/defect code/principle/tool/error code → definition site + per-doc outlines (auto-generated; 333 REQ IDs). Search it; don't read linearly. |
| `docs/architecture/` | Topology, data model, persistence, traceability, search, tech stack. |
| `docs/research/` | Background references (OMG UML, MCP transport, graph DBs, canvas stacks). **Not implementation specs.** |
| `docs/uml-model/` | Metamodel contract: core, structural, behavioral, Requirement/TestCase, traceability chain, diagram interchange. |
| `docs/analytics/` | RTM, adjacency/graph analysis, orphan & coverage (defect codes D1–D7), quality rules. |
| `docs/ui-ux/` | Design system, canvas & gizmo interaction, inspector/explorer, search/analysis UI. |
| `docs/mcp/` | Server design, tools, resources, prompts, client registration. |
| `docs/99-implementation-guide.md` | Vertical-slice build order (Slices 0–6), module boundaries, acceptance template. |

Doc naming: zero-padded `NN-topic.md` (kebab-case) per subsystem; `00-` indexes, `99-`
implementer tie-ins. New top-level docs must be added to both `docs/00-README.md` and the
`docs/AGENTS.md` outline.

## Development Commands

**No build system exists yet** — no `Cargo.toml`, `package.json`, or CI. These are the
documented commands for implementation work (`docs/CONTRIBUTING.md` §2, `docs/99-implementation-guide.md`):

```bash
cargo test                                        # Rust tests
cargo clippy --all-targets -- -D warnings         # zero warnings is the baseline
bunx vitest                                       # frontend tests (Bun only — see below)
bunx oxlint                                       # TS lint
cargo run                                         # Slice 0 acceptance: opens a window
```

Spec editing (Mode B) has no commands: grep-driven cross-reference propagation
(e.g. `grep -rn 'REQ-MM-001' docs/`) plus a consistency review before finishing.

**Owner override (2026-08-09): the JS toolchain is Bun — runtime, package manager, and
runner. No Node, npm, yarn, or pnpm, anywhere (scripts, CI, docs).** Where the spec says
`pnpm`, use Bun instead; treat the spec wording as a known deviation to reconcile in a Mode B
refinement pass.

## Code Conventions & Common Patterns

Spec conventions (binding for docs; the implementation mirrors them):

- **RFC 2119 keywords** (MUST/SHOULD/MAY) with standard meaning.
- **Stable requirement IDs**: every normative requirement is `[REQ-<DOMAIN>-<NNN>]` plus
  invariants `INV-*`, defect codes `D1–D7`, principles `P1–P10`, goals `G1–G10`. Never
  renumber; insert with letter suffixes (`REQ-MM-001a`). `docs/AGENTS.md` maps ID → definition.
- **Status banners gate work**: don't implement against `DRAFT` docs; treat `STABLE`
  ambiguities as issues to file, not guesses.
- Cross-references = relative markdown links; external refs = full URL + one-line citation;
  algorithms are **referenced, never re-derived** (P6).
- One term, one definition: `docs/02-glossary.md`.

Load-bearing architecture patterns (refuse violations in review):

- **P1 — model facts never live on diagrams**; diagrams are views only. Deleting a diagram
  deletes zero elements (REQ-DI-002).
- **P3 — every UI capability has an MCP equivalent** (and vice versa); agents and humans are
  peers sharing one undo stack.
- **Single-writer model API**; SQLite authoritative, projection is rebuildable cache.
- **P9 — no destructive operation bypasses undo**; no panic crosses the model API boundary.
- **REQ-DS-002 — color never carries meaning alone** (shape/text backup required).
- Decision ladder for ambiguity (`docs/03-design-principles.md`): subsystem doc → principles →
  OMG UML spec → MCP spec → ADR → file a spec issue.

## Important Files

- `docs/00-README.md` — entry point and document map.
- `docs/CONTRIBUTING.md` — operating manual (modes, gates, conventions).
- `docs/AGENTS.md` — navigation hub (reverse-index; decision map §11; open questions §12).
- `docs/99-implementation-guide.md` — build order (Slices 0–6), module boundaries, acceptance
  template, anti-patterns list.
- `docs/architecture/06-tech-stack.md` — chosen stack, rejected alternatives, version pins.
- `docs/architecture/03-persistence.md` — SQLite schema, transactions, undo semantics.

## Slice Progress Tracking

**Milestones are the single source of truth for which slices remain.** One GitHub milestone
per slice (`Slice N — <title>`); issue counts and completion percentage render automatically
in the GitHub UI (repo → Milestones). Implementation issues carry labels `impl` + `slice-N`
and belong to exactly one milestone. The Projects v2 board **Smith** tracks per-task status;
milestones track per-slice progress.

## Runtime/Tooling Preferences

Planned stack (`docs/architecture/06-tech-stack.md`); versions are **not pinned yet** —
REQ-STACK-004 requires exact pins (`=` in Cargo.toml, no `^`/`~`) at implementation start:

| Layer | Choice |
|-------|--------|
| Backend | Rust stable, `cargo` workspace |
| App shell | Tauri 2.x (baseline 2.9) |
| Frontend | React 19 + strict TypeScript, **Bun** workspace (owner override — spec says pnpm) |
| Canvas | Konva.js 10 (Canvas 2D); elkjs auto-layout in a Web Worker |
| Persistence | SQLite via `rusqlite` (bundled) — single `.smith` file |
| Graph algorithms | `petgraph` (in-memory projection) |
| MCP | `rmcp` on axum + tokio; serde/serde_json; schemars; UUID v7 |
| Errors/logging | `thiserror` (lib) / `anyhow` (app); `tracing` |
| Lint/format | `cargo clippy` (pedantic, `-D warnings`); `oxlint`/`oxfmt` |

Explicitly rejected: graph DBs as store of record, custom serialization/query languages,
separate backend services, cloud sync, WebGPU/WebGL for v1, pure-Rust GUI (egui = contingency
only), JointJS (fallback only).

## Testing & QA

No tests exist yet. Planned strategy (`docs/99-implementation-guide.md` §Testing strategy):

- **One test per `REQ-*`** — the acceptance-criteria template is the implementer/reviewer
  contract; each criterion cites the `REQ-*`/`INV-*` it satisfies.
- **Levels:** unit (`#[test]`/`#[tokio::test]`; `smith-core`/`smith-model` are pure-Rust
  unit-testable) → property tests (`proptest`: ownership-tree invariants, closure-table
  consistency, ID uniqueness) → integration (`smith-app` round-trip) → frontend (Vitest: gizmo,
  canvas interactions, inspector, explorer) → **MCP conformance harness** (connects as a client,
  exercises every tool) → automated WCAG contrast assertion on design tokens.
- **QA gate (both modes):** `cargo test`, `cargo clippy --all-targets -- -D warnings`,
  `vitest`, `oxlint` — zero warnings is the baseline. Cross-cutting acceptance (ownership tree
  acyclic + connected, diagrams-as-views, single-writer, loopback-only MCP, no panic across the
  model API) must hold after every slice.
- No numeric coverage threshold; "coverage" in this spec usually means traceability coverage,
  not code coverage.

## Notes for AI Assistants

1. **Pick your mode first** (`docs/CONTRIBUTING.md`): implement (A) or refine (B). The rules
   differ — Mode B edits are minimal, propagated to every citing doc, and update the
   `docs/AGENTS.md` index when IDs change.
2. **Navigate by ID via `docs/AGENTS.md`**, then read only the relevant spec docs. Don't read
   `research/` front-to-back.
3. **Don't relitigate settled decisions** (`docs/AGENTS.md` §11); flag open questions (§12),
   don't guess.
4. Asked to create code? Scaffolding doesn't exist — propose the crate/workspace layout from
   `docs/99-implementation-guide.md` Slice 0 before writing files, and keep the cross-cutting
   acceptance list green.
