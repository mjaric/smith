---
status: DRAFT
---

# Analytics 02 — Adjacency Matrix & Graph Analysis

Smith exposes the model's relationship graph (and the trace subgraph) to standard
graph-theoretic analyses. This document specifies **which analyses Smith provides, what each
answers, and the algorithm + reference**. It does NOT derive algorithm math — see
[research/05-traceability-and-analytics.md](../research/05-traceability-and-analytics.md) §3.

Reference implementations: [`petgraph`](https://docs.rs/petgraph/latest/petgraph/) (Rust,
in-process) is the primary engine; algorithms run on the in-memory projection of the model
(see [architecture/03-persistence.md](../architecture/03-persistence.md)). For precedence and
alternative API shapes, see [NetworkX algorithms](https://networkx.org/documentation/stable/reference/algorithms/index.html).

## Two graph inputs

Every analysis operates on one of two derived graphs:

1. **Relationship graph** — all relationship edges (associations, generalizations,
   dependencies incl. trace relations, connectors, messages). Node = element, edge =
   relationship. Directed where the relationship is directed; undirected view available.
2. **Trace graph** — only trace-relation edges (`«satisfy»`, `«realize»`, `«verify»`,
   `«deriveReqt»`, `«refine»`, `«trace»`, `«copy»`). Always directed. Used for RTM, chain
   completeness, circular-trace detection.

`[REQ-GA-001]` Each analysis tool MUST declare which graph it operates on. The default is the
relationship graph unless the analysis is trace-specific (RTM, chain, circular-trace).

## Adjacency matrix

`[REQ-GA-002]` Smith MUST compute and render an **adjacency matrix** view for any selected
subgraph (a package, a diagram's elements, or the whole model):

- Rows and columns = elements (ordered by kind, then qualifiedName).
- Cell `[i][j]` = 1 if an edge from `i` to `j` exists (directed), else 0.
- Cell MAY carry a count (>1 edge) or edge-type label in a denser view.

Reference: [Adjacency matrix (Wikipedia)](https://en.wikipedia.org/wiki/Adjacency_matrix).

`[REQ-GA-003]` The adjacency matrix MUST be available as an MCP tool
(`graph.adjacencyMatrix`) returning a sparse representation (`{nodes: [...], edges: [[i,j], ...]}`)
and as a UI heat-map view (see [ui-ux/04-search-and-analysis-ui.md](../ui-ux/04-search-and-analysis-ui.md)).

## Reachability & transitive closure

`[REQ-GA-004]` Smith MUST compute **reachability** ("is there a path from A to B?") and
**transitive closure** ("all pairs reachable") on demand.

- **Single-source reachability**: BFS from an element. Reference:
  [BFS](https://en.wikipedia.org/wiki/Breadth-first_search).
- **All-pairs transitive closure**: repeated BFS (sparse graphs, typical for UML) preferred
  over Floyd–Warshall. Reference:
  [Transitive closure](https://en.wikipedia.org/wiki/Transitive_closure),
  [Floyd–Warshall](https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm).

`[REQ-GA-005]` Reachability is the engine behind:
- RTM coverage state (requirement → Class/Operation path).
- Impact analysis ("what breaks if this requirement changes" = all elements reachable from
  it on the trace graph).
- Chain completeness.

## Topological sort

`[REQ-GA-006]` Smith MUST compute a **topological ordering** of the trace graph (or any DAG
subgraph), which exists iff the graph is acyclic. Reference:
[Topological sorting](https://en.wikipedia.org/wiki/Topological_sorting).

Use cases:
- Display requirements in implementation order.
- Determine change-propagation order (invalidate downstream artifacts first).
- Validate that an Activity's control flow is acyclic.

`[REQ-GA-007]` If the graph has a cycle, topological sort MUST return the detected cycle as a
defect rather than a partial ordering.

## Cycle detection

`[REQ-GA-008]` Smith MUST detect cycles in the trace graph continuously (incrementally on
each trace-edge mutation) and on demand for the relationship graph. Reference:
[Cycle detection](https://en.wikipedia.org/wiki/Cycle_detection).

A cycle in the trace graph is a HIGH-severity defect (see
[analytics/03-orphan-and-coverage-analysis.md](./03-orphan-and-coverage-analysis.md)).

## Strongly connected components (SCC)

`[REQ-GA-009]` Smith MUST compute SCCs (Tarjan's or Kosaraju's) to identify
mutual-dependency clusters. Reference:
[SCC](https://en.wikipedia.org/wiki/Strongly_connected_component),
[Tarjan](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm).

Any SCC with >1 vertex on the trace graph is a circular-trace defect cluster. On the
relationship graph, large SCCs indicate tightly-coupled design (a code-smell, surfaced as a
LOW-severity "coupling" note, not a defect).

## Shortest path

`[REQ-GA-010]` Smith MUST compute the shortest path between two elements:
- Unweighted (BFS) for "minimum number of hops". Reference:
  [BFS](https://en.wikipedia.org/wiki/Breadth-first_search).
- Weighted (Dijkstra) where edge weights = relationship strength or custom weight.
  Reference: [Dijkstra](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).

Use cases:
- "Show the trace path from requirement REQ-001 to class `Order`" (UI "show chain" feature).
- Impact proximity ("how far is this class from the failing requirement").

## Centrality

`[REQ-GA-011]` Smith MUST compute **betweenness centrality** and **closeness centrality** as
optional analyses (not continuous — on demand). References:
[Betweenness](https://en.wikipedia.org/wiki/Betweenness_centrality),
[Closeness](https://en.wikipedia.org/wiki/Closeness_centrality).

Use cases:
- Identify hub elements (an Activity every trace must pass through — refactor/impact risk).
- Surface in the inspector as a "centrality" badge for the selected element.

`[REQ-GA-012]` Centrality is informational (LOW severity); it is not a defect. The UI shows
it as a non-blocking metric.

## Connected components

`[REQ-GA-013]` Smith MUST compute weakly connected components of the trace graph. Reference:
[Connected component](https://en.wikipedia.org/wiki/Connected_component_(graph_theory)).

A component with no `Requirement` node is a **disconnected subgraph** defect (MEDIUM
severity — orphan cluster; see [analytics/03](./03-orphan-and-coverage-analysis.md)).

## Dominator tree (advanced, optional)

`[REQ-GA-014]` Smith MAY compute a dominator tree for Activity control-flow analysis.
Reference: [Dominator](https://en.wikipedia.org/wiki/Dominator_(graph_theory)).

Use case: identify single points of failure in an Activity (nodes every path must pass
through). Surfaced as an optional Activity-diagram analysis, not globally.

## Performance budget

`[REQ-GA-015]` For models under 10⁵ elements, all on-demand analyses (except full
transitive closure and centrality) MUST complete in under 1 second single-threaded.
Incremental analyses (cycle detection on edge mutation) MUST complete in under 50 ms.

Full transitive closure and centrality MAY be deferred (background job with progress
notification via MCP `notifications/progress`, see
[mcp/01-server-design.md](../mcp/01-server-design.md)).

## MCP tool surface

Each analysis is an MCP tool with `readOnlyHint: true`. See
[mcp/02-tools.md](../mcp/02-tools.md) for the full tool list. Summary:

| Tool | Algorithm | Graph |
|------|-----------|-------|
| `graph.adjacencyMatrix` | adjacency matrix | relationship or trace |
| `graph.reachable` | BFS reachability | specified |
| `graph.transitiveClosure` | repeated BFS / Floyd–Warshall | specified |
| `graph.topoSort` | topological sort | specified |
| `graph.cycles` | DFS cycle detection | trace (continuous) / relationship (on demand) |
| `graph.scc` | Tarjan / Kosaraju | specified |
| `graph.shortestPath` | BFS / Dijkstra | specified |
| `graph.centrality` | betweenness + closeness | relationship (on demand) |
| `graph.connectedComponents` | weakly connected components | trace |

## Open questions (resolve before STABLE)

- [ ] Edge weights for Dijkstra: default all 1 (unweighted)? Or allow user-tagged weights?
      Tentative: default 1; user may tag via stereotype tagged-value `weight`.
- [ ] Should centrality be computed incrementally and cached? Tentative: no — on demand only;
      cache invalidates on any relationship mutation.
