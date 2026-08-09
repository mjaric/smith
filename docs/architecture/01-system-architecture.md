---
status: DRAFT
---

# Architecture 01 — System Architecture

Component topology and data flow for Smith. Ties together the tech stack
([06-tech-stack.md](./06-tech-stack.md)), the data model
([uml-model/01-metamodel-core.md](../uml-model/01-metamodel-core.md)),
persistence ([03-persistence.md](./03-persistence.md)), and the MCP server
([mcp/01-server-design.md](../mcp/01-server-design.md)).

## Process model (single process)

`[REQ-ARCH-001]` Smith runs as a **single OS process** (P10). Inside it:

- **Rust core** (tokio runtime) — owns the model API, the in-memory `petgraph` projection,
  the SQLite store, the command/undo stack, the analysis engine, and the MCP server.
- **Tauri webview** (OS-supplied, out-of-process but spawned by Smith) — the React + Konva UI,
  communicating with the core via Tauri commands and events.
- **MCP HTTP server** (axum, in the Rust core) — bound to `127.0.0.1`, serving MCP clients.

```
┌─────────────────────────── Smith process (Rust, tokio) ───────────────────────────┐
│                                                                                   │
│   ┌──────────────────────────── Model API ────────────────────────────────────┐   │
│   │  (single writer; all mutations serialize here)                            │   │
│   │   • create/read/update/delete elements & relationships                    │   │
│   │   • command/undo stack                                                    │   │
│   │   • invariant enforcement (ownership tree, metamodel kinds)               │   │
│   │   • incremental defect/coverage recomputation                             │   │
│   └──────────────┬──────────────────────────────┬─────────────────────────────┘   │
│                  │ reads                          │ reads/writes                     │
│   ┌──────────────▼──────────────┐  ┌─────────────▼─────────────────────────────┐  │
│   │  In-memory projection        │  │  SQLite store (rusqlite, bundled)         │  │
│   │  (petgraph DiGraph)          │◄─┤  • elements, relationships, ownership     │  │
│   │  • nodes = ElementId         │  │    (closure table), features, stereotypes │  │
│   │  • edges = RelationshipId    │  │  • diagrams + view references (DI)        │  │
│   │  • derived: trace graph,     │  │  • FTS5 search index                      │  │
│   │    coverage index, defects   │  │  • UI state                               │  │
│   └──────────────┬──────────────┘  └────────────────────────────────────────────┘  │
│                  │ analyses (read)                                                  │
│   ┌──────────────▼──────────────────────────────────────────────────────────────┐  │
│   │  Analysis engine (petgraph algorithms: BFS, DFS, SCC, topo, reachability)   │  │
│   │  RTM, adjacency, orphan/coverage, cycles                                    │  │
│   └─────────────────────────────────────────────────────────────────────────────┘  │
│                  │                                                                 │
│   ┌──────────────▼──────────────────────────────────────────────────────────────┐  │
│   │  MCP server (rmcp on axum, Streamable HTTP, 127.0.0.1:<port>/mcp)           │  │
│   │  tools / resources / prompts → Model API + Analysis engine                  │  │
│   └─────────────────────────────────────────────────────────────────────────────┘  │
│                  │                                                                 │
│   ┌──────────────▼──────────────────────────────────────────────────────────────┐  │
│   │  Tauri IPC (commands/events) — bridge to the webview                        │  │
│   └─────────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────┬─────────────────────────────────────────────────────┘
                              │ Tauri IPC
                ┌─────────────▼─────────────┐
                │  Tauri webview (separate) │
                │  React + Konva UI         │
                │  • canvas                 │
                │  • explorer, inspector    │
                │  • issue panel, RTM, etc. │
                └─────────────┬─────────────┘
                              │ owned by human user
                              │
            ┌─────────────────┴──────────────────┐
            │  MCP clients (HTTP/SSE, localhost) │
            │  Claude Code · Codex · Gemini CLI  │
            │  · web chat                        │
            └────────────────────────────────────┘
```

## Concurrency model

`[REQ-ARCH-002]` The **model API is single-writer**: all mutations serialize through one
tokio task holding a write lock on the model. Reads (UI render requests, MCP queries,
analysis) run concurrently against a consistent snapshot.

`[REQ-ARCH-003]` Two sources of mutation: the UI (via Tauri commands) and MCP clients (via
rmcp tools). Both call the same model API; both feed the same undo stack. Neither bypasses
the other.

`[REQ-ARCH-004]` Long-running analyses (full transitive closure, centrality, large RTM) run
on a **separate read-only snapshot** (the petgraph projection is cheaply cloneable for
read-only algorithms) so they don't block the writer or the UI. They are cancellable (see
[mcp/01-server-design.md](../mcp/01-server-design.md) §Concurrency).

## Module boundaries (Rust workspace)

`[REQ-ARCH-005]` Smith's Rust code is organized into crates with one-way dependencies:

| Crate | Depends on | Purpose |
|-------|-----------|---------|
| `smith-core` | — | Domain types: `ElementId`, metamodel enums, `Multiplicity`, `Visibility`, etc. No IO. |
| `smith-store` | `smith-core` | SQLite persistence (rusqlite), schema, migrations, closure-table maintenance, FTS5 indexing. |
| `smith-model` | `smith-core`, `smith-store` | The model API: create/read/update/delete, invariants, command stack. Owns the `petgraph` projection; writes through to `smith-store` for persistence. |
| `smith-analysis` | `smith-core`, `smith-model` | Analysis engine: RTM, adjacency, orphan/coverage, graph algorithms. Read-only over `smith-model`. |
| `smith-mcp` | `smith-core`, `smith-model`, `smith-analysis` | MCP server (rmcp), tools/resources/prompts, axum mounting. |
| `smith-ui-bridge` | `smith-model`, `smith-analysis` | Tauri command handlers (the webview-facing API). |
| `smith-app` | all | The binary: wires tokio runtime, opens the SQLite DB, starts the MCP server, launches Tauri. |

`[REQ-ARCH-006]` Dependencies are strictly one-way (core ← store ← model ← analysis ← mcp/bridge ← app). No cyclic crate dependencies. `smith-core` has no IO dependencies (testable in pure Rust).

## Data flow: a mutation

Example: agent calls `model.createClass`.

1. MCP `tools/call` arrives at axum → rmcp dispatches to `smith-mcp`'s handler.
2. Handler acquires the model write lock, calls `smith-model::create_class(args)`.
3. `smith-model` validates (kind, namespace, name), builds a `Command`, pushes it on the undo stack, applies it to the in-memory projection.
4. The command writes through to `smith-store` (SQLite transaction: insert element row, update closure table, FTS5 index).
5. On commit, `smith-model` emits change events: affected `ElementId`s, diagrams, analyses.
6. `smith-mcp` emits `notifications/resources/updated` on the SSE stream for subscribed URIs.
7. `smith-ui-bridge` receives the change event via a tokio broadcast channel and forwards a Tauri event to the webview.
8. The webview re-renders affected canvas shapes / explorer nodes / inspector fields.

`[REQ-ARCH-007]` Steps 2–5 are one atomic unit (the command). Step 4's SQLite transaction commits or rolls back atomically with the in-memory projection update — they never diverge.

## Data flow: a read

Example: UI requests the current diagram's shapes.

1. Webview calls a Tauri command `get_diagram(diagramId)`.
2. `smith-ui-bridge` reads the diagram + view references from `smith-model` (read lock, snapshot).
3. Returns JSON to the webview, which renders.

`[REQ-ARCH-008]` Reads never block on the writer for long (RwLock; writer holds briefly). Render-critical reads are snapshot-consistent.

## Change propagation

`[REQ-ARCH-009]` `smith-model` emits a **change event** stream (tokio broadcast) with:
- Affected `ElementId`s (field changes).
- Affected relationship IDs.
- Affected diagram IDs (view-reference changes).
- Affected analysis results (defects, coverage, RTM-affected rows).

`[REQ-ARCH-010]` Two subscribers:
- `smith-ui-bridge` → forwards to the webview (Tauri events) for re-render.
- `smith-mcp` → maps to `notifications/resources/updated` URIs for subscribed agents.

`[REQ-ARCH-011]` Change events are coalesced within a single command (one logical mutation = one event bundle), so a batch that touches 100 elements emits one bundle, not 100.

## Startup sequence

`[REQ-ARCH-012]` On launch:
1. Parse CLI args / open the recent project.
2. Open the `.smith` SQLite file (`smith-store`); run pending migrations; `PRAGMA integrity_check`.
3. Load the model into the in-memory projection (`smith-model`) — hydrate `petgraph` from SQLite.
4. Compute initial defect/coverage state (incremental index ready).
5. Start the MCP server (`smith-mcp`) on `127.0.0.1:<port>`; write the discovery file.
6. Launch the Tauri webview (`smith-app`), load the bundled frontend, open the last diagram.

`[REQ-ARCH-013]` Startup MUST complete in < 1 s for a 10⁴-element model on a 2020 laptop (cold cache).

## Shutdown sequence

`[REQ-ARCH-014]` On quit:
1. Flush any pending writes to SQLite (WAL checkpoint).
2. Stop the MCP server (close HTTP listeners; emit nothing special).
3. Remove the discovery file.
4. Exit the tokio runtime; close the webview.

`[REQ-ARCH-015]` Crash recovery: SQLite WAL guarantees durability of committed transactions. On next launch, `PRAGMA integrity_check` + a projection-rehydration sanity check (ownership tree acyclic + connected; every relationship's endpoints exist) surface any corruption as a blocking issue.

## Configuration

`[REQ-ARCH-016]` Smith reads settings from (in priority order): CLI args → project-local `.smith` SQLite file (`ui_state` table — see [architecture/03](./03-persistence.md) §SQLite schema) → user `~/.smith/settings.json` → built-in defaults. Settings: theme, MCP port (or ephemeral), MCP auth token, gizmo gesture threshold, panel layout, autosave interval.

## Error handling

`[REQ-ARCH-017]` `smith-core`/`smith-model`: `thiserror`-typed errors (machine-handleable). `smith-app`/`smith-ui-bridge`: `anyhow` with context. MCP tools map model errors to MCP error codes (see [mcp/01](../mcp/01-server-design.md) §Error model). The UI shows user-facing messages with the offending element + suggested fix.

`[REQ-ARCH-018]` No panic crosses the model API boundary: all fallible operations return `Result`. Panics in analysis (bugs) are caught at the tool/command boundary and returned as internal errors, never crashing the app.

## Logging

`[REQ-ARCH-019]` `tracing` structured logs. Level configurable via MCP `logging/setLevel`. Default INFO; DEBUG for model mutations; TRACE for graph-traversal steps (off by default).

## Open questions (resolve before STABLE)

- [x] **Write-through persistence** (DECIDED): every command commits to SQLite immediately
      (WAL keeps it cheap; synchronous=NORMAL). No separate autosave timer; the Save menu
      item performs a WAL checkpoint for psychological safety. Rationale: write-through
      loses at most one in-flight command on crash, vs. timer-based losing up to the
      interval.
- [x] **SQLite authoritative; projection as cache** (DECIDED per REQ-PERS-001/002). The
      projection rebuilds on load; crash recovery is simpler (no reconciliation between two
      sources of truth).
