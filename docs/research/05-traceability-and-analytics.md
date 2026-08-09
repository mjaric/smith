# Smith — Traceability & Analytics Research

Status: `DRAFT` · Research document (reference material, not an implementation spec)

Research inputs for `analytics/01-rtm.md`, `analytics/02-adjacency-and-graph-analysis.md`,
`analytics/03-orphan-and-coverage-analysis.md`, `architecture/04-traceability-relations.md`.

Scope: **name** each traceability relation, matrix, and graph algorithm; state **what it
answers** for Smith; **link** to the authoritative reference. No derivations.

Conventions:
- SysML 1.7 = OMG Systems Modeling Language v1.7, formal/24-01-07
  (https://sysml.org/.res/docs/specs/OMGSysML-v1.7-24-01-07.pdf). Section numbers cite the
  printed table of contents of that PDF.
- UML 2.5 = OMG Unified Modeling Language v2.5.1, formal/17-12-05
  (https://www.omg.org/spec/UML/2.5.1/PDF). UML 2.5.1 §11.8.16 defines `Abstraction` (the
  metaclass all trace relations stereotype) and `Realization`.
- CLRS = Cormen, Leiserson, Rivest, Stein, *Introduction to Algorithms* — chapters on
  Graph Algorithms (22), Shortest Paths (24); companion site https://mitpress.mit.edu/books/introduction-algorithms-third-edition.
- networkx = NetworkX algorithm reference (graph-backend reference implementation, cited as
  the practical API precedent):
  https://networkx.org/documentation/stable/reference/algorithms/index.html

## 1. Traceability relations (UML + SysML-inspired)

Smith extends UML 2.5 with SysML-style traceability stereotypes. All trace relations are
**directed**: *source → target*. In SysML they are stereotypes of UML `Dependency` or
`Abstraction` (UML 2.5.1 §11.8.16 Abstraction; §7.8.6 Dependency).

### 1.1 `«requirement»` — how SysML models requirements

- **Stereotype:** `«requirement»` (SysML 1.7 §16.3.2.6 *Requirement*, stereotype of UML
  `Class`).
- **What it answers:** what shall the system do/be — a capability or condition that must be
  satisfied. Carries `id`, `text`, and optional properties (priority, rationale, owner).
- **Requirement properties:** `id` (§16.3.1.2 Requirement Notation: name + id + text
  compartments), `text`, `priority`, `method`, `category`, `rationale` (§16.3.1.4 Requirements
  on Other Diagrams, §16.3.3 Verdicts for verification status).
- **Spec sections:** §16 Requirements (overview §16.1), §16.3.2.6 Requirement, §16.3.2.1
  AbstractRequirement, §16.2.1 Requirement Diagram.
- **Spec link:** https://www.omg.org/spec/SysML/1.7/ (PDF:
  https://sysml.org/.res/docs/specs/OMGSysML-v1.7-24-01-07.pdf)
- **Smith mapping:** Smith's `Requirement` element type (see
  `uml-model/04-requirements-and-tests.md`) mirrors this stereotype: name + `id` + `text`
  required; priority/category/method optional.

### 1.2 Trace stereotypes (source → target, semantics, SysML section)

All seven are stereotypes of `Abstraction`/`Dependency`; SysML 1.7 sections below.

| Stereotype | Source element type | Target element type | Semantics | SysML 1.7 § |
|---|---|---|---|---|
| `«satisfy»` | any design model element (Block, UseCase, Activity, Interaction…) | Requirement | source **fulfills** the requirement (design satisfies spec) | §16.3.2.7 |
| `«verify»` | `«testCase»` element (or test procedure / analysis) | Requirement | source **proves** the requirement is met | §16.3.2.9 |
| `«deriveReqt»` | Requirement | Requirement | target is **derived from** source (decomposition / refinement into lower-level requirements) | §16.3.2.3 |
| `«refine»` | any model element | Requirement | source **elaborates** the requirement with greater precision (e.g., a constraint or use case clarifying it) | §16.3.2.5 |
| `«copy»` | Requirement | Requirement | target is a **duplicate** of source, maintained separately | §16.3.2.2 |
| `«trace»` | any model element | any model element | **generic** traceability between artifacts with no specific semantics | §16.3.2.8 |
| `«realize»` | behavioral/structural element | element it realizes | UML `Realization` (UML 2.5.1 §11.8.16, specialization of Abstraction): source is the implementation of target | not a SysML req stereotype; UML 2.5.1 §11.8.16 |

Notes for Smith:
- SysML 1.7 §16.3.2.7/§16.3.2.9/§16.3.2.3/§16.3.2.5/§16.3.2.2/§16.3.2.8 all inherit from
  `Abstraction` (UML 2.5.1 §11.8.16) with `supplier` = requirement side for satisfy/verify.
- `«realize»` is **not** a SysML requirement stereotype; it is UML `Realization`. Smith uses
  it as the *implementation-trace* link in the canonical chain (§1.3) per the Smith README
  (Requirement → UseCase → Activity → Sequence → Class).
- `«trace»` is the catch-all; analytics SHOULD treat it as a weaker link type than
  `satisfy`/`verify` when computing chain completeness.

### 1.3 Canonical Smith chain

`Requirement --«satisfy»--> UseCase --«realize»--> Activity --«realize»--> Interaction(Sequence) --«realize»--> Class/Operation`

- Each hop is a directed edge in Smith's trace graph.
- Chain completeness = directed reachability from a Requirement to at least one
  Class/Operation (Part 3, §3.2; Part 4, query Q6).
- SysML precedent for the full req→design→verify workflow: SysML 1.7 §16.4 Usage Examples —
  §16.4.1 Requirement Decomposition and Traceability, §16.4.2 Requirements and Design
  Elements, §16.4.4 Verification Procedure – Test Case.
- Related MBSE process framing: V-model (requirement decomposition ↔ verification levels):
  https://en.wikipedia.org/wiki/V-model_(software_development)

### 1.4 Requirements-engineering standards (traceability concepts)

- **ISO/IEC/IEEE 29148:2018** — *Systems and software engineering — Life cycle processes —
  Requirements engineering*. Defines traceability as a required characteristic of requirement
  records (bidirectional: forward to design/implementation/test, backward to source/stakeholder
  needs). https://www.iso.org/standard/72089.html
- **ISO/IEC TR 24766:2009** — *Guide for requirements traceability* (companion technical
  report). https://www.iso.org/standard/45172.html
- **IEEE 830-1998** — *Recommended Practice for Software Requirements Specifications*
  (superseded by 29148; still the classic SRS outline incl. traceability appendix guidance).
  https://standards.ieee.org/standard/830-1998.html
- **Wikipedia — Requirements traceability** (good overview + bibliography incl. Gotel &
  Finkelstein 1994): https://en.wikipedia.org/wiki/Requirements_traceability

## 2. Requirements Traceability Matrix (RTM)

### 2.1 Definition

- **RTM** — bidirectional grid mapping each requirement to the artifacts that implement and
  verify it, and each artifact back to the requirement(s) it serves.
- **Forward traceability** — requirement → downstream artifacts (design elements, test cases).
  Answers: "is every requirement satisfied and verified?"
- **Backward traceability** — artifact → upstream requirement(s). Answers: "why does this
  artifact exist? which requirement justifies it?"
- References:
  - Wikipedia — Traceability matrix: https://en.wikipedia.org/wiki/Traceability_matrix
  - Wikipedia — Requirements traceability: https://en.wikipedia.org/wiki/Requirements_traceability
  - PMI — *Requirement traceability, a tool for quality results*:
    https://www.pmi.org/learning/library/requirement-traceability-tool-quality-results-8873
  - ISO/IEC TR 24766:2009: https://www.iso.org/standard/45172.html
- **Note on "IBR — Institute for Building Research RTM template":** no authoritative online
  publication by that organization could be verified (searches resolve only to Shenzhen
  Institute of Building Research, unrelated to RTM templates). The canonical column layout
  below follows the PMI article and IEEE/ISO practice above. `[INFERENCE]`

### 2.2 Standard RTM columns

One row per requirement (forward view) or per artifact (backward view); Smith renders both
from the same graph. Typical columns:

1. Requirement ID
2. Requirement name / text
3. Source (stakeholder need, parent requirement via `«deriveReqt»`)
4. Satisfying design artifacts (targets of incoming `«satisfy»`/`«realize»`)
5. Verification artifacts (incoming `«verify»` — test cases), verdict/status (SysML 1.7
   §16.3.3.1 VerdictKind: pass / fail / untested)
6. Status / coverage flags (satisfied? verified? orphaned?)

### 2.3 What an RTM tool must compute

- For each requirement: set of satisfying artifacts, set of verifying artifacts, verdict
  rollup, coverage status (none / satisfied / verified).
- For each artifact: set of traced requirements (backward).
- Derived counts: total requirements, % satisfied, % verified, orphan counts (below).
- All of these are **directed reachability / degree queries** on the trace graph
  (Part 3 algorithms).

### 2.4 Orphans (quality defects)

- **Orphan requirement** — requirement with no outgoing `«satisfy»`/`«deriveReqt»` (nothing
  claims to satisfy it). Defect: unsatisfied requirement.
- **Orphan design** — design artifact with no incoming trace from any requirement. Defect:
  gold-plating / scope creep; nothing justifies the artifact.
- Both are standard RTM completeness checks (PMI article; ISO/IEC TR 24766). Smith encodes
  them as queries Q1/Q2 in Part 4.

## 3. Adjacency matrix and graph algorithms (reference index)

Name → what it answers for Smith → reference. No derivations.

| # | Algorithm | Answers for Smith | Reference |
|---|---|---|---|
| 1 | **Adjacency matrix** | flat V×V representation of the trace/relationship graph; basis of RTM-style matrix views and boolean-matrix reachability | https://en.wikipedia.org/wiki/Adjacency_matrix |
| 2 | **Reachability / transitive closure** (Warshall / Floyd–Warshall; repeated BFS for sparse graphs) | "can this requirement reach this class?"; closure answers all RTM cells, all chain-completeness checks | https://en.wikipedia.org/wiki/Transitive_closure · https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm · https://en.wikipedia.org/wiki/Reachability |
| 3 | **Topological sort** (Kahn / DFS-based) | linear ordering of req → design → impl dependency DAG; existence iff acyclic; invalidation order for change impact | https://en.wikipedia.org/wiki/Topological_sorting |
| 4 | **Cycle detection** (DFS back-edge / coloring) | traceability cycles = defects (a requirement chain that depends on itself) | https://en.wikipedia.org/wiki/Cycle_detection · https://en.wikipedia.org/wiki/Directed_acyclic_graph |
| 5 | **Strongly connected components** — Tarjan's / Kosaraju's | mutual-dependency clusters; any SCC with >1 vertex on the trace graph is a circular-trace defect cluster | https://en.wikipedia.org/wiki/Strongly_connected_component · https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm · https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm |
| 6 | **Shortest path** — BFS (unweighted) / Dijkstra (weighted) | "distance from requirement to implementation"; minimal trace path, change-impact proximity | https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm · https://en.wikipedia.org/wiki/Breadth-first_search |
| 7 | **Betweenness / closeness centrality** | identify hub elements (an Activity/Class every trace must pass through — refactor / impact risk) | https://en.wikipedia.org/wiki/Betweenness_centrality · https://en.wikipedia.org/wiki/Closeness_centrality |
| 8 | **Connected components** (weak, on directed trace graph) | isolated subgraphs = orphan clusters with no trace to any requirement | https://en.wikipedia.org/wiki/Connected_component_(graph_theory) |
| 9 | **Dominator tree** (advanced) | for activity control flow: nodes that every path to a node must pass through — single points of failure in an Activity's flow | https://en.wikipedia.org/wiki/Dominator_(graph_theory) |

Reference implementations / further reading:
- NetworkX algorithms reference (DAG, components, centrality, shortest paths):
  https://networkx.org/documentation/stable/reference/algorithms/index.html
- CLRS — *Introduction to Algorithms*: https://mitpress.mit.edu/books/introduction-algorithms-third-edition

## 4. Quality queries Smith must support

| # | Query | Detects | Algorithm(s) that answer it |
|---|---|---|---|
| Q1 | **Orphan requirement** | requirement with no outgoing `«satisfy»`/`«deriveReqt»` — spec never implemented | out-degree test on requirement-typed vertices in the trace subgraph (O(1) per vertex) |
| Q2 | **Orphan element** | element not owned by any package reachable from the model root, or with no incoming relationship at all — dead/degenerate model content | ownership-tree reachability (BFS/DFS from root over `owns` edges); in-degree test |
| Q3 | **Uncovered element** (user's explicit ask) | element that exists in the model but appears on **no diagram** | set difference: model elements − elements referenced by any diagram view; no graph algorithm needed — view membership index |
| Q4 | **Disconnected subgraph** | cluster of elements with no trace path to any requirement | weakly connected components (algorithm 8); components containing zero Requirement vertices are orphan clusters |
| Q5 | **Circular trace** | cycle in the trace graph — traceability must be acyclic | cycle detection (algorithm 4) or SCC with >1 vertex (algorithm 5) |
| Q6 | **Incomplete chain** | requirement with no directed path to any Class/Operation — spec that never reached implementation | reachability from requirement-typed vertices to Class/Operation-typed vertices (algorithm 2; BFS per requirement, or transitive closure batch) |

Notes:
- Q2's ownership half is the "element not owned by a package reachable from root" check —
  in Smith's property graph every element MUST have a unique owner; an unreachable element
  means a corrupt/dangling owner edge.
- Q3 is computed from the **DI / view layer**, not the trace graph (diagrams reference
  elements; an element with zero references is uncovered). See
  `uml-model/06-diagram-interchange.md`.
- All six are candidate MCP analysis tools (`mcp/02-tools.md`) and issue-panel entries
  (`ui-ux/04-search-and-analysis-ui.md`).

## 5. SysML requirement & test modeling reference

Smith extends UML with requirements + test cases; SysML is the precedent.

- **Requirement stereotype** — `«requirement»` with `id`, `text`, optional properties
  (SysML 1.7 §16.3.2.6; notation §16.3.1.2; tables §16.3.1.5 Requirements Table).
  https://www.omg.org/spec/SysML/1.7/
- **`«verify»` relation** — from a `«testCase»` (or other verification artifact) to a
  Requirement, asserting the test demonstrates the requirement (SysML 1.7 §16.3.2.9).
- **`«testCase»` stereotype** — stereotype of UML `UseCase`/Operation representing a test
  procedure; its relationship to the requirement is `«verify»` (SysML 1.7 §16.3.2.4 TestCase,
  §16.4.4 Verification Procedure – Test Case).
- **Verdicts** — `VerdictKind` (pass/fail/untested) recorded on verification
  (SysML 1.7 §16.3.3.1); Smith's RTM verdict column mirrors this.
- **Requirement decomposition & traceability worked example** — SysML 1.7 §16.4.1, §16.4.2.
- **SysML v2** (successor language; requirements as first-class `RequirementDefinition` /
  `RequirementUsage`, satisfaction and verification via typed links): https://www.omg.org/spec/SysML/2.0/
- **MBSE verification context (V-model)** — requirement levels ↔ verification levels:
  https://en.wikipedia.org/wiki/V-model_(software_development)

Smith's test-case extension (`uml-model/04-requirements-and-tests.md`) follows §16.3.2.4 +
§16.3.2.9 semantics: a TestCase is a stereotyped UseCase/Operation whose only trace edge
kind is `«verify»` targeting one or more Requirements.

## Sources

### OMG specifications
- OMG SysML 1.7 spec landing: https://www.omg.org/spec/SysML/1.7/ · PDF formal/24-01-07:
  https://sysml.org/.res/docs/specs/OMGSysML-v1.7-24-01-07.pdf (§16 Requirements: §16.3.2.1–.9
  stereotypes, §16.3.3 Verdicts, §16.4 usage examples)
- OMG SysML 2.0 spec landing: https://www.omg.org/spec/SysML/2.0/ · overview:
  https://www.omg.org/sysml/SysML-2.htm
- OMG UML 2.5.1 landing: https://www.omg.org/spec/UML/2.5.1/ · PDF formal/17-12-05:
  https://www.omg.org/spec/UML/2.5.1/PDF (Abstraction §11.8.16, Realization; Dependency §7.8.6)

### IEEE / ISO standards
- IEEE 830-1998 (SRS): https://standards.ieee.org/standard/830-1998.html
- ISO/IEC/IEEE 29148:2018 (Requirements engineering): https://www.iso.org/standard/72089.html
- ISO/IEC TR 24766:2009 (Traceability guide): https://www.iso.org/standard/45172.html

### RTM practice
- PMI — *Requirement traceability, a tool for quality results*:
  https://www.pmi.org/learning/library/requirement-traceability-tool-quality-results-8873
- Wikipedia — Traceability matrix: https://en.wikipedia.org/wiki/Traceability_matrix
- Wikipedia — Requirements traceability: https://en.wikipedia.org/wiki/Requirements_traceability

### Graph algorithms
- Adjacency matrix: https://en.wikipedia.org/wiki/Adjacency_matrix
- Transitive closure: https://en.wikipedia.org/wiki/Transitive_closure · Reachability:
  https://en.wikipedia.org/wiki/Reachability · Floyd–Warshall:
  https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
- Topological sorting: https://en.wikipedia.org/wiki/Topological_sorting · DAG:
  https://en.wikipedia.org/wiki/Directed_acyclic_graph
- Cycle detection: https://en.wikipedia.org/wiki/Cycle_detection
- SCC: https://en.wikipedia.org/wiki/Strongly_connected_component · Tarjan:
  https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm ·
  Kosaraju: https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm
- Dijkstra: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm · BFS:
  https://en.wikipedia.org/wiki/Breadth-first_search
- Betweenness: https://en.wikipedia.org/wiki/Betweenness_centrality · Closeness:
  https://en.wikipedia.org/wiki/Closeness_centrality
- Connected component: https://en.wikipedia.org/wiki/Connected_component_(graph_theory)
- Dominator: https://en.wikipedia.org/wiki/Dominator_(graph_theory)
- NetworkX algorithms reference: https://networkx.org/documentation/stable/reference/algorithms/index.html
- CLRS *Introduction to Algorithms*: https://mitpress.mit.edu/books/introduction-algorithms-third-edition

### Process context
- V-model (software development): https://en.wikipedia.org/wiki/V-model_(software_development)
