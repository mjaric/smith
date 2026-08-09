---
status: DRAFT
---

# 02 — Glossary

Single source of truth for vocabulary used across all Smith specifications. Every other
document MUST use these terms with exactly these meanings. If a term is missing, add it
here first; do not coin a local synonym.

## Model & ownership

| Term | Definition |
|------|------------|
| **Model** | The complete, authoritative graph of all elements and relationships in a Smith project. There is exactly one model per project. |
| **Element** | Any node in the model graph. Extends `NamedElement` (has a name, namespace, visibility). See [uml-model/01-metamodel-core.md](./uml-model/01-metamodel-core.md). |
| **Relationship** | Any edge in the model graph connecting two (or more) elements. E.g. `Association`, `Generalization`, `Dependency`. |
| **Namespace** | An element that can own other named elements (e.g. `Package`, `Class`). Each owned element's *qualified name* is derived from its namespace chain. |
| **Package** | The primary namespace and ownership container. Packages nest. The root package is the `Project`. |
| **Ownership tree** | The tree formed by `owner → owned` edges, rooted at the project. This is where elements *live*; diagrams reference elements but do not own them. |
| **QualifiedName** | The `/`-delimited path from the root package to an element, e.g. `/System/Billing/Invoice`. Derived, not stored. |
| **Closure table** | The persistence structure for the ownership tree: a table of all ancestor → descendant ownership pairs (transitive closure), enabling subtree queries without recursive traversal. Backs the SQLite store of record. See [architecture/03-persistence.md](./architecture/03-persistence.md). |

## Diagrams as views

| Term | Definition |
|------|------------|
| **Diagram** | A *view* over the model: a named, typed selection of elements and relationships plus per-diagram layout (position, size) for each. A diagram owns **no** model elements. |
| **Diagram kind** | One of the 14 UML diagram types (e.g. Class, Sequence). Determines which element types the diagram may reference. |
| **View reference** | The link from a diagram to a model element it displays. Carries layout (x, y, w, h) and per-diagram presentation hints (folded compartments, bend points). |
| **Presentation / DI** | Diagram Interchange: the layout + presentation data attached to a view reference. Separated from the model element per OMG UML DI. |
| **Uncovered element** | A model element referenced by **no** diagram. Surfaced as a quality issue (user's explicit ask). |

## Metamodel (UML core)

| Term | Definition |
|------|------------|
| **NamedElement** | Root metaclass: every model element has a name, a namespace, and a visibility. OMG UML §7.8. |
| **Classifier** | An element that classifies instances (Class, Interface, UseCase, Actor, Component, …). |
| **Class** | A classifier with attributes, operations, and (optionally) nesting. |
| **Feature** | A structural or behavioral member of a classifier (Property = attribute/association end; Operation = behavioral signature). |
| **Multiplicity** | The cardinality bound on an association end or attribute (`0..1`, `1`, `*`, `1..n`). |
| **Stereotype** | A user-defined extension of a UML metaclass, applied to elements via `«guillemets»`. Defined in a `Profile`. OMG UML §12.3, §12.4, §22. |
| **Tagged value** | A named value attached to an element via an applied stereotype. |
| **Profile** | A package that defines stereotypes and their extensions. |

## Traceability & extensions

| Term | Definition |
|------|------------|
| **Requirement** | A first-class element (SysML-inspired) with an `id`, `text`, and optional properties. Lives in the ownership tree like any element. |
| **TestCase** | A first-class element representing a test that verifies one or more requirements. |
| **Trace relation** | A typed dependency whose stereotype encodes a traceability semantics (`«satisfy»`, `«verify»`, `«realize»`, `«deriveReqt»`, `«trace»`, `«refine»`, `«copy»`). See [architecture/04-traceability-relations.md](./architecture/04-traceability-relations.md). |
| **Canonical chain** | `Requirement → UseCase → Activity → Sequence → Class/Operation`. A complete model has a path along this chain for each requirement. |
| **Forward trace** | Following trace relations from a requirement outward to design/tests. |
| **Backward trace** | Following trace relations from a test/design element back to a requirement. |
| **Trace graph** | The derived subgraph containing only trace-relation edges (`«satisfy»`, `«realize»`, `«verify»`, `«deriveReqt»`, `«refine»`, `«trace»`, `«copy»`). Used for RTM, reachability, chain completeness, circular-trace detection. See [architecture/04-traceability-relations.md](./architecture/04-traceability-relations.md). |
| **Relationship graph** | The derived subgraph containing ALL relationship edges (associations, generalizations, dependencies, trace relations, connectors, messages). Used for adjacency matrix, SCC, centrality. See [architecture/04-traceability-relations.md](./architecture/04-traceability-relations.md). |

## Analytics

| Term | Definition |
|------|------------|
| **RTM** | Requirements Traceability Matrix: a table mapping each requirement to the artifacts that satisfy/verify it (forward) and each artifact to its requirement(s) (backward). See [analytics/01-rtm.md](./analytics/01-rtm.md). |
| **Adjacency matrix** | A square matrix `A` where `A[i][j] = 1` if an edge from element `i` to element `j` exists. The basis for reachability and graph analysis. |
| **Orphan** | A model element in a defective state: either an orphan *requirement* (no incoming `«satisfy»`/`«verify»`/`«realize»` AND no outgoing `«deriveReqt»`), an orphan *element* (no owner path to root, OR no incoming AND no outgoing relationships), or an *uncovered* element (on no diagram). See [analytics/03-orphan-and-coverage-analysis.md](./analytics/03-orphan-and-coverage-analysis.md). |
| **Disconnected subgraph** | A connected component of the relationship graph that has no trace path to any requirement. |
| **Circular trace** | A cycle in the trace-relation subgraph (a defect; traceability must be a DAG). |
| **VerdictKind** | The per-requirement verdict derived from the `status` (passing / failing / etc.) of the source `TestCase` on each `«verify»` edge: `passing` if all verifying TestCases pass, `failing` if any fails, else `untested`. NOT stored as a per-relation field — derived on demand. See [analytics/01-rtm.md](./analytics/01-rtm.md). |
| **Gold-plating** | A non-requirement element (design artifact) with no trace to any requirement; surfaced as an orphan artifact in the backward RTM. No dedicated defect code in v1 (reported via the RTM orphan-artifact rollup). See [analytics/01-rtm.md](./analytics/01-rtm.md). |

## Agent / MCP

| Term | Definition |
|------|------------|
| **MCP** | Model Context Protocol. JSON-RPC 2.0 protocol over stdio or HTTP/SSE by which an AI client talks to a server that exposes tools, resources, and prompts. See [research/02-mcp-transport.md](./research/02-mcp-transport.md). |
| **Tool** | An MCP-callable operation Smith exposes (e.g. `model.createClass`, `analysis.rtm.forward`). See [mcp/02-tools.md](./mcp/02-tools.md). |
| **Resource** | A readable MCP item Smith exposes (e.g. a model element, a diagram, an analysis result). See [mcp/03-resources.md](./mcp/03-resources.md). |
| **Prompt** | A parameterized MCP prompt template Smith exposes for common modeling tasks. See [mcp/04-prompts.md](./mcp/04-prompts.md). |
| **Agent** | An AI client (Claude Code, Codex, Gemini CLI, web chat) connected to Smith's MCP server. |
| **Session** | A single MCP connection lifecycle, identified by an `Mcp-Session-Id`. |

## UI

| Term | Definition |
|------|------------|
| **Canvas** | The infinite pan/zoom surface on which a diagram's view references are rendered. |
| **Tool** (UI) | An active editing mode (Select, Create-Class, Create-Relation, etc.). Distinct from MCP *Tool*. Context disambiguates. |
| **Gizmo** | The context-sensitive radial/popup control that appears at the click point on empty canvas when the Select tool is active. See [ui-ux/02-canvas-and-interaction.md](./ui-ux/02-canvas-and-interaction.md). |
| **Connection handle** | A small affordance on a selected element's edge from which a new relationship is dragged. |
| **Inspector** | The context-aware properties panel for the current selection. |
| **Model explorer** | The tree view of the ownership tree (packages → elements). |
| **Issue panel** | The list of current model-quality defects (orphans, uncovered, cycles). |
| **Marking menu** | The press-drag-release gesture style for the canvas gizmo: press, drag toward a slot, release to select it (faster for power users). See [ui-ux/02-canvas-and-interaction.md](./ui-ux/02-canvas-and-interaction.md). |

## File & runtime

| Term | Definition |
|------|------------|
| **Project** | A single Smith document = the model + diagrams + presentation data + profile, persisted to a `.smith` file/dir. |
| **Workspace** | The running Smith instance: one open project + the MCP server + the UI. |
