---
status: DRAFT
---

# Architecture 06 — Technology Stack

The chosen stack, with rationale. Tied to the research documents; every choice is justified
against Smith's constraints (single binary, embedded graph DB, embedded MCP server, beautiful
interactive canvas, Rust backend).

See:
- [research/03-graph-database-options.md](../research/03-graph-database-options.md)
- [research/02-mcp-transport.md](../research/02-mcp-transport.md) §8 (Rust SDK)
- [research/06-canvas-and-frontend-stack.md](../research/06-canvas-and-frontend-stack.md)

## Summary table

| Layer | Choice | Rationale |
|-------|--------|-----------|
| **Backend language** | Rust (stable) | User requirement; performance, safety, single-binary. |
| **App shell / IPC** | Tauri 2.x | Single binary with embedded frontend assets; OS-supplied webview; MIT; stable (2.9.x). [research/06](../research/06-canvas-and-frontend-stack.md) §2. |
| **Frontend framework** | React 19 + TypeScript (strict) | Largest ecosystem, `react-konva`, design-system maturity. [research/06](../research/06-canvas-and-frontend-stack.md) §5. |
| **Canvas renderer** | Konva.js 10 (Canvas 2D) | Crisp text at all zooms, built-in hit layer, MIT, renderer-only (matches P1). [research/06](../research/06-canvas-and-frontend-stack.md) §3. |
| **Auto-layout** | elkjs (Web Worker) | Best UML-style layered layout; EPL-2.0 weak copyleft, unmodified-use compatible. [research/06](../research/06-canvas-and-frontend-stack.md) §8. |
| **Connector routing** | Wybrow-style orthogonal (in-house TS/Rust), ELK for whole-diagram | Referenced algorithm, not re-derived (P6). [research/06](../research/06-canvas-and-frontend-stack.md) §7. |
| **Persistence (store of record)** | SQLite via `rusqlite` (bundled feature) | Battle-tested ACID single file; recursive CTEs for traversal; FTS5 for search; inspectable by agents. [research/03](../research/03-graph-database-options.md) §4. |
| **In-memory graph engine** | `petgraph` | Algorithms (BFS, DFS, SCC, topo, reachability) run in RAM on a projection. [research/03](../research/03-graph-database-options.md) §4.3. |
| **Ownership-tree encoding** | Closure table (+ adjacency list as source of truth) | Single-join subtree/ancestor queries; depth available. [research/03](../research/03-graph-database-options.md) §5. |
| **Full-text search** | SQLite FTS5 | Ships with SQLite; no extra dependency. |
| **MCP server** | `rmcp` (official Rust SDK) on axum | Schema-faithful, conformance-tested, Streamable HTTP transport. [research/02](../research/02-mcp-transport.md) §8. |
| **MCP transport** | Streamable HTTP (legacy era, 2025-06-18 / 2025-11-25) on `127.0.0.1` | All target clients (Claude Code, Codex, Gemini CLI, web) support it. [research/02](../research/02-mcp-transport.md) §2.5. |
| **Async runtime** | tokio | rmcp + axum + Tauri async tasks all run on tokio. |
| **HTTP server** | axum | rmcp's Streamable HTTP service is a Tower service mounted on axum. |
| **Serialization** | serde + serde_json | Model API and MCP messages are JSON. |
| **Schema validation (MCP)** | schemars (via rmcp) | Generates JSON Schema 2020-12 for tool `inputSchema`. |
| **IDs** | UUID v7 (time-ordered, sortable) | Stable, opaque, globally unique. |
| **Error layer** | `thiserror` (library) / `anyhow` (app) | Per repo conventions. |
| **Logging / tracing** | `tracing` | Structured logs; MCP `logging/setLevel` integration. |
| **Testing** | `cargo test` + `vitest` (frontend) | Per repo conventions. |
| **Lints** | `cargo clippy` (pedantic, `-D warnings`), `oxlint`/`oxfmt` (TS) | Per repo conventions. |
| **Project file** | `.smith` (SQLite single file) | P6 boring choice; agent-inspectable. |

## Process topology

```
┌─────────────────────────────────────────────────────────────┐
│                       Smith process                         │
│                                                             │
│  ┌──────────────────┐   ┌────────────────────────────────┐  │
│  │   Tauri webview  │   │         Rust core              │  │
│  │  (React + Konva) │◄──┤  ┌──────────────────────────┐  │  │
│  │                  │   │  │   Model API (single       │  │  │
│  │  Canvas, panels  │   │  │   writer, command stack)  │  │  │
│  └──────────────────┘   │  └──────────┬───────────────┘  │  │
│          ▲ Tauri cmds   │             │                  │  │
│          │ / events     │  ┌──────────▼───────────────┐  │  │
│          │              │  │  petgraph in-memory      │  │  │
│          │              │  │  projection (analytics)  │  │  │
│          │              │  └──────────┬───────────────┘  │  │
│          │              │             │                  │  │
│          │              │  ┌──────────▼───────────────┐  │  │
│          │              │  │  SQLite (rusqlite,       │  │  │
│          │              │  │  bundled) — .smith file  │  │  │
│          │              │  └──────────────────────────┘  │  │
│          │              └─────────────┬──────────────────┘  │
│          │                            │                     │
│          │              ┌─────────────▼──────────────────┐  │
│          │              │  MCP server (rmcp on axum)      │  │
│          │              │  Streamable HTTP, 127.0.0.1     │  │
│          │              └─────────────┬──────────────────┘  │
│          │                            │                     │
└──────────┼────────────────────────────┼─────────────────────┘
           │                            │
           │ Tauri IPC                  │ HTTP/SSE on loopback
           │                            │
      Human user              ┌─────────┴─────────┐
                              │ MCP clients:      │
                              │ Claude Code,      │
                              │ Codex, Gemini CLI,│
                              │ web chat          │
                              └───────────────────┘
```

`[REQ-STACK-001]` The Rust core, the SQLite store, the MCP server, and the Tauri host MUST
run in a single OS process. The webview is the only out-of-process component (OS-supplied,
not shipped by Smith).

`[REQ-STACK-002]` The model API MUST be the single writer (all mutations serialize through
one async task). The UI (via Tauri commands) and MCP clients (via rmcp tools) both call the
same model API.

`[REQ-STACK-003]` The MCP server MUST bind to `127.0.0.1` only, on a configurable port
(default: ephemeral, announced via discovery file — see
[mcp/01-server-design.md](../mcp/01-server-design.md)).

## Why SQLite (not a graph DB)

Graph-DB-specific products were rejected for concrete reasons — see
[research/03](../research/03-graph-database-options.md) §2 for the full candidate matrix.
The short version:

- **Server-based** (Neo4j, AGE, FalkorDB): violate single-binary (P10).
- **Kùzu**: archived upstream.
- **CozoDB**: feature-ideal but stalled ~20 months; acceptable only as a secondary,
  replaceable query layer, not the store of record.
- **Sled**: self-declared beta, unstable format.
- **Turso/Limbo**: no `WITH RECURSIVE` yet (fatal for graph traversal).

SQLite wins on: 30 years of hardening, single-file ACID, recursive CTEs for traversal, FTS5
for search, single `rusqlite` crate with `bundled` feature (no external C build at user
runtime), and inspectability by any sqlite3 tool (agent-friendly). The graph traversal that
a graph DB would give us is handled by `petgraph` in RAM — at UML scale (≤10⁵ elements) this
is microseconds-to-milliseconds.

## Why Tauri 2 + web (not pure-Rust GUI)

See [research/06](../research/06-canvas-and-frontend-stack.md) §2–4. The deciding factors:

1. **Text crispness at arbitrary zoom** — UML is text-heavy; the browser text stack is the
   best 2D text engine available. Pure-Rust (egui glyph atlas, Vello) is good but bespoke.
2. **"Beautiful, bespoke chrome" (P7)** — web CSS/design-system ecosystem is unmatched; a
   pure-Rust widget set costs person-years for equivalent polish.
3. **Auto-layout ecosystem** — ELK.js, Graphviz WASM run natively in the webview; no
   maintained Rust ports exist.
4. **Single binary still holds** — Tauri embeds frontend assets into the Rust binary; the
   webview itself is OS-supplied (WKWebView on macOS, WebView2 on Windows, WebKitGTK on
   Linux). P10 ("single binary") is satisfied; P10 does not require "no system webview".

The fallback (JointJS core) and contingency (egui) are documented in
[research/06](../research/06-canvas-and-frontend-stack.md) §9 and are NOT the v1 choice.

## Version pins (to be locked at implementation start)

`[REQ-STACK-004]` All dependencies MUST be pinned to exact versions (`=` in Cargo.toml, no
`^`/`~` in package.json) per repo supply-chain rules. The versions cited in the research
docs are the verified-stable baselines as of 2026-08-09; the implementer MUST confirm
current stable versions at lock time.

Baseline crates (verify before pinning):

```toml
# Cargo.toml (illustrative — verify versions at implementation start)
[dependencies]
tauri = "2.9"            # app shell
rusqlite = "0.40"        # bundled SQLite
petgraph = "0.7"         # in-memory graph
rmcp = "0.x"             # MCP server (verify current on crates.io)
axum = "0.8"             # HTTP server for MCP
tokio = "1"              # async runtime
serde = "1"              # serialization
serde_json = "1"
schemars = "0.8"         # JSON Schema for MCP tools
uuid = { version = "1", features = ["v7"] }
thiserror = "2"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Non-choices (explicitly rejected)

- **Custom serialization format** — use JSON (MCP speaks JSON; serde handles it).
- **Custom query language** — SQL (recursive CTEs) + Rust (`petgraph`) suffice.
- **A separate backend service** — everything in-process (P10).
- **Cloud sync / remote backend** — out of scope (non-goal in [01-vision.md](../01-vision.md)).
- **WebGPU / WebGL for v1** — Canvas 2D suffices at Smith's scale; avoid webview
  fragmentation. PixiJS is the documented escape hatch.

## Open questions (resolve before STABLE)

- [ ] Exact `rmcp` version once pinned — verify it supports the legacy-era Streamable HTTP
      shape Smith targets (§2.5 of research/02).
- [ ] Whether to vendor ELK.js or load via npm — npm (with exact pin) is simpler.
- [ ] `.smith` file: single SQLite file, or a directory (for future assets like exported
      images)? Current spec: single file; assets stored as blobs if ever needed.
