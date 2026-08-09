# 03 — Graph Database Options for Smith

**Status:** research complete · **As of:** 2026-08-09 · **Purpose:** select the persistence strategy for Smith's UML model store (nodes = elements, edges = relationships, ownership = Package→Element tree, diagrams = views over the model).

This document is reference-grade: every claim carries a source link; all activity/license data was pulled from the projects' own repositories and release APIs on 2026-08-09.

---

## 1. Evaluation Axes

| # | Axis | Weight |
|---|------|--------|
| A1 | Native Rust or first-class Rust bindings | High |
| A2 | Embedded, in-process (no server) — desktop single-binary distribution | **Required** |
| A3 | Property-graph model (typed nodes + edges, both with properties) | High |
| A4 | Multi-hop traversal performance (RTM, impact analysis, orphan detection) | High |
| A5 | ACID transactions, crash safety for a desktop file (kill -9, power loss) | **Required** |
| A6 | Single-file or directory storage for `.smith` project files | High (single file preferred) |
| A7 | License compatibility (MIT/Apache-2.0 preferred) | High |
| A8 | Active maintenance (2024–2026 commits and releases) | High |

**Workload profile:** one user, one process, models of roughly $10^2$–$10^5$ elements, interactive whole-graph analyses (RTM, adjacency matrix, orphan/coverage detection). The entire graph fits comfortably in RAM; **persistence is the storage problem, traversal is the compute problem.** This decoupling (Section 4.3) drives the recommendation.

---

## 2. Candidate Matrix

| Name | Model | Embedded? | Rust-native? | License | Last commit / release | Maintenance signal | Graph-query capability | Verdict |
|------|-------|-----------|--------------|---------|----------------------|--------------------|------------------------|---------|
| **SQLite** via `rusqlite` (bundled) | Relational | ✅ | Bindings to C (bundled & compiled in) | MIT (rusqlite) + public-domain (SQLite) | rusqlite 0.40.1 (bundles SQLite 3.53.2); repo pushed 2026-08-08 | Excellent; SQLite is a 30-year project with published durability guarantees | Recursive CTEs, triggers, FKs, FTS5 full-text search | **Strong candidate — top pick** |
| **Redb** | KV + multimap tables | ✅ | ✅ pure Rust | MIT OR Apache-2.0 | Stable file format; repo pushed 2026-08-09 | Excellent; single lead maintainer, stable format policy | None built-in; multimap tables map directly to adjacency lists | **Strong candidate — pure-Rust runner-up** |
| **CozoDB** | Relational/Datalog, graph-capable | ✅ | ✅ | MPL-2.0 | v0.7.6 (2023-12-11); last push 2024-12-04 | **Stalled ~20 months**, single maintainer, 49 open issues | Best-in-class: recursive Datalog, built-in SCC / PageRank / Dijkstra / BFS / DFS / topological sort; MVCC; time travel | **Conditional** — feature-ideal, maintenance risk disqualifies it as the primary store |
| **IndraDB** | Property graph | ✅ (library mode) | ✅ | MPL-2.0 | v5.0.0 (2025-08-16); repo pushed 2025-08-16 | Solo maintainer; yearly major releases (v3 2022, v4 2023, v5 2025); real repo is `indradb/indradb` | Typed Rust query API; multi-hop queries with property filters; no text query language; **no transaction API**; persistent datastores (RocksDB/sled) are directory-based, separate crates | **Weak** |
| **Turso / Limbo** (`tursodatabase/turso`) | Relational (SQLite-compatible Rust rewrite) | ✅ | ✅ | MIT | Pre-1.0, very high velocity; 23.8k stars | Excellent funding/activity | **`WITH RECURSIVE` not yet supported** (explicit in COMPAT.md) — cannot do graph traversal | **Rejected (today)** — revisit when recursive CTEs land |
| **PoloDB** | Document (BSON, MongoDB-like) | ✅ | ✅ | Apache-2.0 | v5.3.0; repo pushed 2026-08-09 | Very active | No graph queries; traversal would be hand-written; RocksDB-backed (directory, not single file; C++/libclang build dependency) | **Rejected** — wrong model for the job |
| **Sled** | KV | ✅ | ✅ | Apache-2.0 OR MIT | Last stable release 0.34.7 (2021-09); main branch is a large in-progress rewrite | Effectively stalled; self-declared *beta*; README: "if reliability is your primary constraint, use SQLite" | None (KV only) | **Rejected** — unstable format, beta status |
| **Fjall** | KV (LSM), transactions + savepoints | ✅ | ✅ pure Rust | Apache-2.0 | Repo pushed 2026-07-22 | Active | None (KV only) | **Noted** — viable redb alternative, smaller ecosystem |
| **Oxigraph** | RDF triplestore (SPARQL) | ✅ | ✅ | MIT OR Apache-2.0 | Repo pushed 2026-08-06 | Active, well-run | SPARQL with property paths; powerful but RDF ≠ labeled property graph | **Weak** — model mismatch for UML LPG semantics |
| **SurrealDB** | Document-graph | ✅ (embedded engine) | ✅ | BUSL-1.1 (shown as "Other" on GitHub) | Repo pushed 2026-07-06 | Very active | Graph traversal in SurrealQL | **Rejected** — non-OSI license [INFERENCE on BUSL details; verify before any reconsideration] |
| **Kùzu** | Property graph (Cypher) | ✅ | ❌ (C++) | MIT | **Repository archived** (GitHub `archived: true`, announcement ~2025-10); last release 0.11.3 | Dead | Cypher, columnar, was "the SQLite for graphs" | **Rejected** — archived upstream |
| **Apache AGE** | Property graph (openCypher) | ❌ Postgres extension | ❌ | Apache-2.0 | Active | n/a | openCypher | **Rejected** — requires a running PostgreSQL server; violates A2 |
| **Neo4j** | Property graph (Cypher) | ❌ server | ❌ JVM | GPLv3 (Community Edition) | Active | n/a | Cypher | **Rejected** — server process + GPL incompatible with Smith's distribution |
| **FalkorDB** | Property graph (Cypher; RedisGraph lineage) | ❌ server | ❌ | Server license; moot | Active | n/a | Cypher | **Rejected** — requires a FalkorDB server process; violates A2 |

---

## 3. Candidate Notes

### 3.1 SQLite via `rusqlite` (bundled) — **top pick**
- `rusqlite` 0.40.1 bundles and compiles SQLite 3.53.2 from source (`bundled` feature) → single self-contained desktop binary; MIT licensed.
- Single-file database = natural `.smith` project file; ACID with WAL/journal modes; `PRAGMA integrity_check`.
- Graph traversal: recursive CTEs (`WITH RECURSIVE`), mature since SQLite 3.8.3. Ownership tree via closure table (Section 5).
- FTS5 gives Smith its **search** feature out of the box.
- Foreign keys enforce UML metamodel referential integrity declaratively.
- Agent-native bonus: any MCP client (Claude Code, Codex, Gemini CLI) can inspect a `.smith` file with stock SQLite tooling, read-only.
- Costs: C dependency (mitigated by bundling); verbose SQL for multi-hop pattern queries; no native property-graph query language.

### 3.2 Redb — **pure-Rust runner-up**
- Pure Rust, ACID, MVCC (concurrent readers + writer), crash-safe by default, **savepoints and rollback**, single-file storage, stable file format with an upgrade-path commitment.
- `MultimapTable` (key → many values) is a near-perfect primitive for typed adjacency lists (edge lists per vertex, per relation type).
- No query layer: Smith writes BFS/DFS/toposort itself — at UML scale this is trivially fast in-process (Section 4.3).
- No FTS: search would need tantivy or a manual inverted index.
- Trade vs SQLite: more ownership of the storage schema, zero C dependencies, savepoints usable as coarse batch-undo checkpoints.

### 3.3 CozoDB — feature-ideal, maintenance-stalled
- Embeddable Datalog database in Rust; storage engines in-tree: memory, **SQLite (single file)**, RocksDB, sled, TiKV (`cozo-core/src/storage/`).
- MVCC multi-statement transactions; optional per-relation **time travel** (valid-time history).
- Built-in whole-graph algorithms exactly matching Smith's analytics list: SCC, shortest path (BFS + Dijkstra + A* + Yen), topological sort, PageRank, MST, community detection.
- Performance claims: 2-hop traversal < 1 ms on a 1.6M-vertex / 31M-edge graph (README benchmarks).
- **Risk:** last release v0.7.6 on 2023-12-11; last push 2024-12-04; single maintainer; MPL-2.0. Adopting it pins Smith to a query-language dependency that may never advance. Acceptable only as a secondary, replaceable query engine over the same data — not as the store of record.

### 3.4 IndraDB
- Genuine Rust-native property graph; embeddable via `indradb-lib`; typed query API with multi-hop and indexed property filters.
- In-memory datastore is built-in; persistent stores (RocksDB, sled, Postgres) are separate crates and **directory/server-based**, not single-file.
- No transaction API → Smith would have to implement crash-consistency boundaries itself. Solo maintainer, MPL-2.0.
- Verdict: interesting API design reference, weak foundation for a desktop document store.

### 3.5 Turso / Limbo
- Rust rewrite of SQLite, MIT, extremely active, huge momentum; explicitly guarantees migration back to SQLite.
- **Fatal gap today:** `COMPAT.md` marks the WITH clause as partial and states "WITH RECURSIVE not yet supported." Without recursive CTEs there is no graph traversal. Pre-1.0 with other partial features (WITHOUT ROWID insert-only, VACUUM experimental).
- Action: track the `WITH RECURSIVE` issue; if it ships and stabilizes, Turso becomes a drop-in upgrade of the SQLite option with identical schema.

### 3.6 Sled
- Self-describes as beta; README known-issues: "if reliability is your primary constraint, use SQLite; sled is beta", on-disk format will change before 1.0 requiring manual migration.
- Last stable release 0.34.7 (2021); main branch is a ground-up rewrite tied to the komora project.
- Verdict: do not build a document format on it.

### 3.7 The server-based options (AGE, Neo4j, FalkorDB)
- All require a separate server process, violating the single-binary desktop requirement (A2) outright.
- Neo4j Community Edition is GPLv3 — additionally incompatible with Smith's intended distribution.
- They remain useful as *export targets* (e.g., Cypher/GQL export) rather than backends.

### 3.8 Kùzu (archived)
- Was the closest thing to "SQLite for graphs": embedded, Cypher, MIT. The repository is now **archived** (announcement in README, GitHub archived flag set). Demonstrates the fragility of single-company embedded graph DBs and reinforces choosing boring, broadly maintained infrastructure.

---

## 4. Recommendation

### 4.1 Ranking

| Rank | Option | One-line rationale |
|------|--------|--------------------|
| 🥇 | **SQLite (`rusqlite`, bundled) + adjacency list + closure table + recursive CTEs + FTS5, with `petgraph` as the in-memory analysis engine** | Battle-tested ACID single file; recursive CTEs cover all traversal needs at UML scale; FTS5 covers search; zero exotic dependencies; inspectable by any agent |
| 🥈 | **Redb + hand-rolled graph layer (`petgraph` for algorithms)** | Pure Rust, MVCC + savepoints, single file; chosen only if the team accepts writing all traversal/search infrastructure itself |
| 🥉 | **CozoDB with its SQLite storage engine** | The richest graph query layer (Datalog + built-in algorithms), acceptable only behind an interface boundary, version-pinned, with a documented exit plan — because upstream is stalled |

### 4.2 Explicit tradeoffs

| Dimension | SQLite option | Redb option | CozoDB option |
|---|---|---|---|
| Time-to-first-graph-query | Low (SQL) | High (write traversal code) | Lowest (Datalog + canned algorithms) |
| Query language dependency | SQL (universal) | None (Rust code) | CozoScript (niche, frozen upstream) |
| Crash safety | Proven (WAL + fsync semantics, 20+ years of hardening) | Strong (COW B+tree, fuzzed) | MVCC solid, but unexercised release line |
| Single `.smith` file | ✅ | ✅ | ✅ (SQLite engine) |
| Search feature | FTS5 built-in | Must build (tantivy) | Full-text exists (cangjie tokenizer) |
| External inspectability (MCP/agents) | ✅ any sqlite3 tool | Custom format | Any sqlite3 tool (SQLite engine) |
| Rust purity | C bundled in | ✅ pure Rust | ✅ pure Rust |
| License | MIT / public domain | MIT OR Apache-2.0 | MPL-2.0 (file-level copyleft) |
| 2026 maintenance risk | Minimal | Low | **High** (no release since 2023-12) |
| Undo support | Command layer (Section 7) | Savepoints + command layer | Time travel + command layer |

### 4.3 Architectural consequence (applies to any pick)

Smith should keep an **in-memory `petgraph` projection** of the model for interactive editing and analytics (RTM, adjacency matrix, orphans, impact analysis). The embedded DB is the durable store of record; algorithms run in RAM on load/mutation. At $10^5$ elements this is microseconds-to-milliseconds and makes the persistence choice **reversible** — the graph-layer interface should hide the DB so SQLite↔Redb↔Turso migration stays a bounded refactor.

---

## 5. Ownership-Tree Representation (Package → Element)

Smith needs: subtree enumeration (a package's contents), ancestor path (breadcrumbs), element move between packages, cascade delete of a package, and "elements owned here" for the tree navigator.

| Strategy | Subtree query | Move element | Write cost | Notes |
|---|---|---|---|---|
| Adjacency list (`parent_id`) | Recursive CTE (O(depth × fanout)) | 1 UPDATE | O(1) | Simplest; every deep query pays recursion; FK-enforceable |
| **Closure table** (`ancestor, descendant, depth`) | 1 indexed JOIN | 2 statements: delete old closure rows, insert cross-product of new ancestors × subtree descendants | O(subtree × ancestors) per move | Karwin's recommended pattern; depth carried explicitly; trivial "all descendants" / "all ancestors" / path queries |
| Nested set (`lft, rgt`) | `WHERE lft BETWEEN …` | Rewrite all `lft/rgt` right of insertion point | O(n) per structural change | Fast reads, hostile to an interactive editor where reparenting is frequent |
| Materialized path (`/a/b/c`) | `LIKE '/a/b/%'` prefix scan | Rewrite path prefix of whole subtree | O(subtree) | Good for breadcrumb display; LIKE queries index poorly across collations |

**Recommendation: closure table**, maintained in the same transaction as the ownership edge (SQLite triggers or explicit application writes), with adjacency list kept as the normalized source of truth (`elements.owner_id` + FK) for integrity. Rationale: subtree and ancestor queries are single indexed joins (the RTM and navigator hot paths), moves are two set-based statements — acceptable for an interactive editor — and depth is available for indentation without recursion.

References:
- Stack Overflow canonical discussion — "What is the most efficient/elegant way to parse a flat table into a tree?": <https://stackoverflow.com/questions/192220/>
- Bill Karwin, *SQL Antipatterns Volume 2* (Pragmatic Bookshelf), ch. "Naive Trees" (closure table chapter): <https://pragprog.com/titles/bksap2/sql-antipatterns-volume-2/>

---

## 6. Graph Traversal Algorithms Smith Needs

Named for spec reference; implementations live in `petgraph` (<https://docs.rs/petgraph/latest/petgraph/>) or recursive CTEs. Math intentionally not restated — see linked references.

| Algorithm | Use in Smith | Reference |
|---|---|---|
| BFS | Reachability layers, nearest-neighbor highlighting | <https://en.wikipedia.org/wiki/Breadth-first_search> |
| DFS | Connected components, cycle spotting in dependency views | <https://en.wikipedia.org/wiki/Depth-first_search> |
| Topological sort | Requirement→design→test ordering, activity flow validation | <https://en.wikipedia.org/wiki/Topological_sorting> |
| Transitive closure (Warshall–Floyd) | RTM matrix, full traceability chain materialization | <https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm>, <https://en.wikipedia.org/wiki/Transitive_closure> |
| Strongly connected components (Tarjan) | Circular-dependency detection among requirements/classes | <https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm> |
| Reachability | Impact analysis ("what breaks if this requirement changes") | <https://en.wikipedia.org/wiki/Reachability> |
| Shortest path (BFS for unweighted; Dijkstra for weighted) | Trace chain display between any two elements | <https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm> |
| Textbook treatment | CLRS, *Introduction to Algorithms*, graph-algorithms chapters | <https://mitpress.mit.edu/9780262046305/introduction-to-algorithms/> |
| Worked examples | NetworkX algorithms reference | <https://networkx.org/documentation/stable/reference/algorithms/index.html> |

---

## 7. Versioning / Undo

| Backend | MVCC / history | Consequence for Smith |
|---|---|---|
| SQLite | No MVCC (single-writer locking; WAL gives concurrent readers) | Smith **must** implement its own command/undo layer |
| Redb | MVCC + savepoints/rollback | Savepoints cover transaction-scoped rollback; semantic undo still application-level |
| CozoDB | MVCC + per-relation time travel (valid-time queries) | Historical states queryable, but UX-level undo still application-level |

**Conclusion:** MVCC is concurrency control, not undo. Smith needs a **command pattern with inverse operations** regardless of backend (each user gesture = one command recording `do`/`undo` deltas over the model graph). Choose the backend on storage merits; build undo in the model layer. If CozoDB's time travel is adopted later, it can power *audit/history views*, not Ctrl-Z.

---

## Sources

All repositories and release APIs accessed 2026-08-09.

1. SQLite recursive CTEs — <https://www.sqlite.org/lang_with.html>
2. SQLite FTS5 — <https://www.sqlite.org/fts5.html>
3. SQLite WAL — <https://www.sqlite.org/wal.html>
4. rusqlite — <https://github.com/rusqlite/rusqlite> (pushed 2026-08-08; v0.40.1 bundles SQLite 3.53.2)
5. Redb — <https://github.com/cberner/redb> (pushed 2026-08-09) · design doc: <https://github.com/cberner/redb/blob/master/docs/design.md>
6. Fjall — <https://github.com/fjall-rs/fjall> (pushed 2026-07-22)
7. CozoDB — <https://github.com/cozodb/cozo> (v0.7.6 2023-12-11, last push 2024-12-04) · docs: <https://docs.cozodb.org/> · storage engines source: `cozo-core/src/storage/{mem,rocks,sled,sqlite,tikv}.rs`
8. IndraDB — <https://github.com/indradb/indradb> (v5.0.0 2025-08-16) · <https://indradb.github.io/>
9. Turso/Limbo — <https://github.com/tursodatabase/turso> · compatibility matrix (WITH RECURSIVE gap): <https://github.com/tursodatabase/turso/blob/main/COMPAT.md>
10. PoloDB — <https://github.com/PoloDB/PoloDB> (v5.3.0, pushed 2026-08-09)
11. Sled — <https://github.com/spacejam/sled> (last stable 0.34.7, 2021; beta self-assessment in README)
12. Oxigraph — <https://github.com/oxigraph/oxigraph> (pushed 2026-08-06)
13. SurrealDB — <https://github.com/surrealdb/surrealdb> (license shown "Other"; BUSL-1.1 claim marked [INFERENCE])
14. Kùzu — <https://github.com/kuzudb/kuzu> (GitHub `archived: true`; archive notice in README)
15. Apache AGE — <https://age.apache.org/>
16. Neo4j licensing — <https://neo4j.com/licensing/>
17. FalkorDB — <https://github.com/FalkorDB/FalkorDB>
18. petgraph — <https://docs.rs/petgraph/latest/petgraph/>
19. Ownership trees — <https://stackoverflow.com/questions/192220/> · Karwin, *SQL Antipatterns Volume 2* — <https://pragprog.com/titles/bksap2/sql-antipatterns-volume-2/>
20. Algorithm references — Wikipedia entries and CLRS/NetworkX links as tabulated in Section 6.
