---
status: DRAFT
---

# Architecture 03 — Persistence

How the model is stored durably. Decisions driven by
[research/03-graph-database-options.md](../research/03-graph-database-options.md): **SQLite
(rusqlite, bundled) is the store of record; `petgraph` is the in-memory projection for
analytics.** Ownership tree uses a **closure table** (+ adjacency list as source of truth).

This document specifies the SQLite schema, the projection strategy, transactions, undo, and
the `.smith` file format.

## Store of record vs. projection

`[REQ-PERS-001]` **SQLite is the store of record.** Every committed model mutation is durable
in SQLite (WAL mode). On launch, the in-memory projection is hydrated from SQLite; on
mutation, both update in one transaction.

`[REQ-PERS-002]` The **in-memory projection** (`petgraph` `DiGraph<ElementNode, EdgeNode>`)
is a cache for fast graph algorithms and live UI. It is rebuildable from SQLite at any time.
It is NEVER the source of truth — if it diverges, SQLite wins.

`[REQ-PERS-003]` Rationale (per [research/03](../research/03-graph-database-options.md) §4.3):
at UML scale (≤10⁵ elements) the whole graph fits in RAM; traversal is a compute problem
solved by `petgraph`, persistence is a storage problem solved by SQLite. This decoupling
makes the DB choice reversible.

## SQLite schema

`[REQ-PERS-004]` The schema (illustrative DDL; the implementer finalizes types/constraints):

```sql
-- Pragmas
PRAGMA journal_mode=WAL;
PRAGMA foreign_keys=ON;
PRAGMA synchronous=NORMAL;

-- Metadata
CREATE TABLE smith_meta (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);  -- smith_version, created_at, schema_version, ...

-- Elements (nodes) — single table, kind-discriminated
CREATE TABLE elements (
  id          TEXT PRIMARY KEY,           -- ElementId (UUID v7 string)
  kind        TEXT NOT NULL,              -- metaclass: 'Class', 'Package', 'Requirement', ...
  name        TEXT,                       -- nullable (NamedElement.name)
  owner_id    TEXT,                       -- adjacency-list parent (Namespace); NULL for root
  visibility  TEXT DEFAULT 'public',      -- public|private|protected|package
  data        TEXT NOT NULL DEFAULT '{}', -- JSON blob: kind-specific fields (attributes, operations,
                                         --   requirement text/category, test steps, ...)
  created_at  TEXT NOT NULL,
  updated_at  TEXT NOT NULL,
  FOREIGN KEY (owner_id) REFERENCES elements(id) ON DELETE RESTRICT
);
CREATE INDEX idx_elements_owner ON elements(owner_id);
CREATE INDEX idx_elements_kind  ON elements(kind);

-- Closure table for the ownership tree (Package/Namespace owns members)
CREATE TABLE ownership_closure (
  ancestor_id   TEXT NOT NULL,
  descendant_id TEXT NOT NULL,
  depth         INTEGER NOT NULL,          -- 0 = self, 1 = direct child, ...
  PRIMARY KEY (ancestor_id, descendant_id),
  FOREIGN KEY (ancestor_id)   REFERENCES elements(id) ON DELETE CASCADE,
  FOREIGN KEY (descendant_id) REFERENCES elements(id) ON DELETE CASCADE
);
CREATE INDEX idx_closure_anc ON ownership_closure(ancestor_id, depth);
CREATE INDEX idx_closure_dec ON ownership_closure(descendant_id);

-- Relationships (edges)
CREATE TABLE relationships (
  id          TEXT PRIMARY KEY,            -- RelationshipId
  kind        TEXT NOT NULL,               -- 'Association', 'Generalization', 'Dependency', ...
  stereotype  TEXT,                        -- for trace relations: 'satisfy', 'verify', ...
  source_id   TEXT NOT NULL,
  target_id   TEXT NOT NULL,
  owner_id    TEXT NOT NULL,               -- owning namespace
  data        TEXT NOT NULL DEFAULT '{}',  -- JSON: multiplicity ends, association kind, ...
  created_at  TEXT NOT NULL,
  updated_at  TEXT NOT NULL,
  FOREIGN KEY (source_id) REFERENCES elements(id) ON DELETE RESTRICT,
  FOREIGN KEY (target_id) REFERENCES elements(id) ON DELETE RESTRICT,
  FOREIGN KEY (owner_id)  REFERENCES elements(id) ON DELETE RESTRICT
);
CREATE INDEX idx_rel_source ON relationships(source_id);
CREATE INDEX idx_rel_target ON relationships(target_id);
CREATE INDEX idx_rel_kind   ON relationships(kind);

-- Stereotype applications
CREATE TABLE applied_stereotypes (
  id            TEXT PRIMARY KEY,
  element_id    TEXT NOT NULL,
  stereotype_id TEXT NOT NULL,             -- references the Stereotype element
  tag_values    TEXT NOT NULL DEFAULT '{}',-- JSON map
  FOREIGN KEY (element_id) REFERENCES elements(id) ON DELETE CASCADE
);
CREATE INDEX idx_stereo_element ON applied_stereotypes(element_id);

-- Comments (owned by an element)
CREATE TABLE comments (
  id           TEXT PRIMARY KEY,
  owner_id     TEXT NOT NULL,              -- the element owning this comment
  body         TEXT NOT NULL,
  annotated    TEXT NOT NULL DEFAULT '[]', -- JSON array of ElementIds
  FOREIGN KEY (owner_id) REFERENCES elements(id) ON DELETE CASCADE
);
CREATE INDEX idx_comments_owner ON comments(owner_id);

-- Diagrams (views) and view references (DI shapes/edges)
CREATE TABLE diagrams (
  id           TEXT PRIMARY KEY,
  name         TEXT NOT NULL,
  kind         TEXT NOT NULL,              -- DiagramKind
  owner_id     TEXT NOT NULL,              -- owning package
  canvas_state TEXT NOT NULL DEFAULT '{}', -- JSON: panX, panY, zoom
  FOREIGN KEY (owner_id) REFERENCES elements(id) ON DELETE CASCADE
);

CREATE TABLE shape_views (
  id               TEXT PRIMARY KEY,       -- ViewRefId (per-diagram)
  diagram_id       TEXT NOT NULL,
  model_element_id TEXT NOT NULL,
  bounds           TEXT NOT NULL,          -- JSON: {x,y,w,h}
  hints            TEXT NOT NULL DEFAULT '{}', -- PresentationHints JSON
  container_shape  TEXT,                   -- parent shape (nested)
  FOREIGN KEY (diagram_id)       REFERENCES diagrams(id) ON DELETE CASCADE,
  FOREIGN KEY (model_element_id) REFERENCES elements(id) ON DELETE CASCADE
);
CREATE INDEX idx_shape_diagram ON shape_views(diagram_id);
CREATE INDEX idx_shape_element ON shape_views(model_element_id);  -- coverage index

CREATE TABLE edge_views (
  id                TEXT PRIMARY KEY,
  diagram_id        TEXT NOT NULL,
  model_relation_id TEXT NOT NULL,
  source_shape_id   TEXT NOT NULL,
  target_shape_id   TEXT NOT NULL,
  bend_points       TEXT NOT NULL DEFAULT '[]', -- JSON array of {x,y}
  label_positions   TEXT NOT NULL DEFAULT '{}',
  routing_kind      TEXT NOT NULL DEFAULT 'orthogonal',
  FOREIGN KEY (diagram_id)          REFERENCES diagrams(id)     ON DELETE CASCADE,
  FOREIGN KEY (model_relation_id)   REFERENCES relationships(id) ON DELETE CASCADE,
  FOREIGN KEY (source_shape_id)     REFERENCES shape_views(id)  ON DELETE CASCADE,
  FOREIGN KEY (target_shape_id)     REFERENCES shape_views(id)  ON DELETE CASCADE
);
CREATE INDEX idx_edge_diagram ON edge_views(diagram_id);

-- FTS5 full-text search index
CREATE VIRTUAL TABLE elements_fts USING fts5(
  element_id UNINDEXED,
  name, qualified_name, kind, requirement_id, test_case_id, comment_body,
  tokenize = 'unicode61'
);

-- UI state (per-project panel layout, expanded nodes, etc.)
CREATE TABLE ui_state (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
```

`[REQ-PERS-005]` The `data` JSON blobs per element/relationship hold kind-specific fields. This avoids a wide sparse table; the `smith-model` crate owns the (de)serialization into typed Rust structs per kind. Blobs are validated on write (serde), never trusted raw.

`[REQ-PERS-006]` The `shape_views.model_element_id` index IS the coverage index (REQ-DI-015): `SELECT model_element_id FROM shape_views` gives all covered elements; set difference with `elements` yields uncovered.

## Closure-table maintenance

`[REQ-PERS-007]` The `ownership_closure` table is maintained in the same transaction as the owning edge. On:
- **Element creation** (with owner O): insert rows `(anc, new, depth+1)` for each `(anc, O, depth)` in O's ancestor chain, plus `(new, new, 0)`.
- **Reparent** (move from old owner A to new owner B): delete rows where `descendant = X` or any descendant of X has ancestor in A's chain; insert cross-product of B's ancestors × X's subtree. (Two set-based statements — see [research/03](../research/03-graph-database-options.md) §5.)
- **Delete** (cascade): `ON DELETE CASCADE` removes the element's closure rows.

`[REQ-PERS-008]` Closure-table maintenance MUST be atomic with the element/owner mutation (same SQLite transaction). A failure rolls back both.

## Migrations

`[REQ-PERS-009]` Smith ships a `smith_meta.schema_version` and a forward-only migration runner. On open: check version, run pending migrations in a transaction, bump version. Migrations are tested; a failed migration aborts open with a clear message.

`[REQ-PERS-010]` Migrations are append-only SQL/Rust files; no down-migrations (Smith is local-first; a broken migration is a bug to fix forward, not a rollback).

## Transactions & concurrency

`[REQ-PERS-011]` SQLite is opened in **WAL mode** (`PRAGMA journal_mode=WAL`), `synchronous=NORMAL`. This gives concurrent readers + one writer, crash-safe commits.

`[REQ-PERS-012]` Smith holds a single write connection (the model API's writer). Read connections are pooled for UI/analysis/MCP queries. No long-running write transactions (each command commits promptly).

`[REQ-PERS-013]` Every model command = one SQLite transaction. If the transaction fails, the in-memory projection is NOT updated (they update together or not at all — REQ-ARCH-007).

## Undo / redo (command stack)

`[REQ-PERS-014]` Undo is a **command pattern** in `smith-model` (NOT SQLite-level — see [research/03](../research/03-graph-database-options.md) §7: MVCC is concurrency, not undo). Each command records `do` and `undo` deltas.

`[REQ-PERS-015]` Command stack:
- Bounded depth (default 200); oldest evicted.
- Project-wide (not per-diagram).
- Shared by UI (Cmd/Ctrl+Z) and MCP (`batch.undo`).
- Each command tagged with `actor` (`ui` or `session:<id>`).
- A `batch` command wraps N sub-commands so they undo as one unit.

`[REQ-PERS-016]` Undo/redo replays the inverse/forward delta through the same model API (re-validating invariants). An undo that would violate an invariant is rejected with a clear error (rare — usually only happens if the model changed underneath; in single-user Smith, this shouldn't occur).

## `.smith` file format

`[REQ-PERS-017]` A `.smith` "file" is a **single SQLite database file** (with `-wal` and `-shm` sidecars during use, checkpointed on close). Extension: `.smith` (the file is a valid SQLite DB regardless of extension).

`[REQ-PERS-018]` The file is inspectable by any sqlite3 tool — agents and humans can run `sqlite3 project.smith ".tables"` to understand the model directly (agent-friendly property — see [research/03](../research/03-graph-database-options.md) §3.1).

`[REQ-PERS-019]` Smith MUST set a SQLite `PRAGMA application_id` to a Smith-specific magic number and verify it on open (reject non-Smith SQLite files with a clear error).

`[REQ-PERS-020]` Backup: Smith writes `.smith.bak` (previous version) on each successful open, rotated to keep the last 3. Users can also export to XMI/JSON (see [mcp/02-tools.md](../mcp/02-tools.md) `project.export`).

## Performance budget

`[REQ-PERS-021]` For a 10⁵-element model:
- Cold load (hydrate projection): < 1 s.
- Single-element mutation (create/rename/delete): < 10 ms end-to-end (including closure-table + FTS5 + projection + notification).
- Full RTM computation (in-memory): < 500 ms (per [analytics/01](../analytics/01-rtm.md) REQ-RTM-005).
- WAL checkpoint on close: < 500 ms.

## Open questions (resolve before STABLE)

- [x] `.smith` is a **single SQLite file** (DECIDED — per REQ-PERS-017/018/019; assets as
      blobs if ever needed; a directory layout would complicate single-binary distribution
      and agent inspectability). Cross-references: architecture/06, mcp/01 §Discovery.
- [x] v1 supports **one project per Smith process** (DECIDED). Multiple windows open the
      same project (shared model API). Multi-project is a v2 candidate.
- [x] WAL `synchronous=NORMAL` (DECIDED — good-enough durability for a desktop tool; FULL
      halves write throughput and the cost/benefit doesn't justify it for local-first use).
