---
status: DRAFT
---

# Architecture 05 — Search

Smith's search subsystem. Full-text search (FTS5 in SQLite) for human queries; structured
filters and graph-traversal queries for the inspector, explorer, and MCP tools.

Backend: SQLite FTS5 ([research/03](../research/03-graph-database-options.md) §3.1) +
`petgraph` for graph queries. UI: [ui-ux/04-search-and-analysis-ui.md](../ui-ux/04-search-and-analysis-ui.md).
MCP tools: [mcp/02-tools.md](../mcp/02-tools.md) `search.*`, `graph.*`.

## Full-text search (FTS5)

`[REQ-SEARCH-001]` Smith maintains an **FTS5 virtual table** (`elements_fts`) indexing, per
element: name, qualified name (derived), kind, requirement_id (if Requirement), test_case_id
(if TestCase), and the bodies of comments attached to the element. See the schema in
[architecture/03-persistence.md](./03-persistence.md).

`[REQ-SEARCH-002]` The FTS index is updated **incrementally** on every element mutation
(create/update/delete), in the same SQLite transaction. It is never rebuilt wholesale except
on migration or corruption recovery.

`[REQ-SEARCH-003]` Tokenizer: `unicode61` (Unicode-aware word splitting; case-insensitive;
accent-sensitive by default). Suitable for English/European text. (CJK segmentation is out of
scope v1 — note as a known limitation; a future tokenizer upgrade is a migration.)

`[REQ-SEARCH-004]` Ranking: FTS5 `bm25` (built-in) for relevance. Results ranked by bm25
score, then by recency (updated_at) as a tiebreak.

`[REQ-SEARCH-005]` Query syntax: FTS5's standard query syntax (quoted phrases, prefix `term*`,
boolean `AND`/`OR`/`NOT`, column-restricted `name:foo`). Smith's search bar exposes a subset
(plain terms + quoted phrases + prefix) and escapes special characters; power users can opt
into raw FTS5 syntax via an advanced toggle.

## Structured filtering

`[REQ-SEARCH-006]` FTS results compose with structured filters (applied in SQL after the FTS
rowid join):

| Filter | Column | Match |
|--------|--------|-------|
| Kind | `elements.kind` | equals (multi-select) |
| Owning package | `ownership_closure.ancestor_id` | equals or subtree |
| Stereotype applied | `applied_stereotypes.stereotype_id` | exists / equals |
| Defect state | derived (D1–D7 from [analytics/03](../analytics/03-orphan-and-coverage-analysis.md)) | equals (multi-select) |
| Coverage | derived (on-no-diagram) | boolean |
| Visibility | `elements.visibility` | equals |
| Created/updated | `elements.created_at`/`updated_at` | range |

`[REQ-SEARCH-007]` A query without a text term returns all elements matching the structured
filters (the "advanced search" mode — see [ui-ux/04](../ui-ux/04-search-and-analysis-ui.md)
REQ-UI-SRCH-005). This is how "all approved Requirements in /System with no «verify»" is
expressed: kind=Requirement, package=/System subtree, status=approved (in `data` JSON),
defect-state=D1.

## Graph-traversal queries

These are NOT FTS — they run on the in-memory `petgraph` projection via
`smith-analysis`. Exposed as MCP `graph.*` tools (see
[mcp/02-tools.md](../mcp/02-tools.md)) and used by the analysis UI.

| Query | Algorithm | Reference |
|-------|-----------|-----------|
| Reachability from X | BFS | [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) |
| All-pairs reachability (transitive closure) | repeated BFS (sparse) / Floyd–Warshall (dense) | [Transitive closure](https://en.wikipedia.org/wiki/Transitive_closure) |
| Topological order | Kahn / DFS topo | [Topological sorting](https://en.wikipedia.org/wiki/Topological_sorting) |
| Cycles | DFS back-edge detection | [Cycle detection](https://en.wikipedia.org/wiki/Cycle_detection) |
| SCC | Tarjan | [Tarjan SCC](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm) |
| Shortest path X→Y | BFS (unweighted) / Dijkstra (weighted) | [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) |
| Connected components | weak/strong | [Connected components](https://en.wikipedia.org/wiki/Connected_component_(graph_theory)) |
| Centrality | betweenness, closeness | [Betweenness](https://en.wikipedia.org/wiki/Betweenness_centrality) |

References are authoritative; Smith does not re-derive (P6). Implementations via `petgraph`
and Smith-specific code where `petgraph` lacks an algorithm (centrality — `petgraph` has
none; implement per NetworkX's reference, [link](https://networkx.org/documentation/stable/reference/algorithms/index.html)).

`[REQ-SEARCH-008]` Graph queries operate on one of two derived graphs (relationship graph or
trace graph — see [architecture/04-traceability-relations.md](./04-traceability-relations.md)
§Trace graph vs. relationship graph). The caller specifies which; default is the relationship
graph.

## Graph scope

`[REQ-SEARCH-009]` Graph queries accept a **scope**: whole model, a package subtree (via the
closure table — `ownership_closure.descendant_id IN (subtree)`), or a single diagram's
elements. Scoped queries run on a subgraph projection (cheaply derived from `petgraph`).

## Index strategy

`[REQ-SEARCH-010]` Smith maintains these derived indexes (all in the in-memory projection,
rebuilt on load, updated incrementally):

| Index | Purpose | Update trigger |
|-------|---------|----------------|
| FTS5 (`elements_fts`) | Text search | SQLite, in-transaction |
| Coverage index (`shape_views.model_element_id`) | Uncovered-element detection | SQLite, in-transaction |
| Trace graph (subgraph of trace edges) | RTM, chain, cycles | projection, on relationship mutation |
| Defect state (per element) | Issue panel | projection, on mutation |
| Adjacency lists (per node, in/out by edge kind) | BFS, SCC, degree queries | projection, on relationship mutation |

`[REQ-SEARCH-011]` Indexes are **derived** — never stored as primary data, always rebuildable
from the model. A corrupt index is rebuilt, not repaired.

## Performance budget

`[REQ-SEARCH-012]` For a 10⁵-element model:
- FTS query (top 50 results): < 50 ms.
- Structured filter query (no FTS, 10⁴ results): < 100 ms.
- Single-source BFS reachability: < 10 ms.
- Full transitive closure: < 500 ms (deferred/progress-notified if larger).
- Tarjan SCC: < 200 ms.

## MCP exposure

`[REQ-SEARCH-013]` Search is exposed as MCP tools (`search.elements`, `search.relations`,
`search.diagrams`) returning `{elementId, score, snippet}` lists — see
[mcp/02-tools.md](../mcp/02-tools.md). Graph queries are the `graph.*` tools. Both are
`readOnlyHint: true`.

## Open questions (resolve before STABLE)

- [ ] CJK tokenizer for FTS5 (out of scope v1) — note as a known limitation; users with CJK
      content get whole-string substring matches, not word matches.
- [ ] Fuzzy matching (Levenshtein) for typo tolerance? FTS5 has no built-in fuzzy; a
      trigram-based approach would help. Tentative: out of scope v1; prefix matching (`term*`)
      covers the common case.
- [ ] Search history / saved queries? Tentative: saved queries (named, in ui_state) in v1;
      history later.
