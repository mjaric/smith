---
status: DRAFT
---

# MCP 02 — Tools

The complete list of MCP tools Smith exposes. Every tool has a typed signature (JSON Schema
input), a defined result shape, error codes, and annotations. The UI calls the same
underlying model operations (design principle P3).

Conventions:
- Tool names: `camelCase` (namespaced by domain) per common MCP convention; SEP-986
  provides name-format guidance (`model.*`, `diagram.*`, `analysis.*`, `project.*`).
- `inputSchema` and `outputSchema` are JSON Schema 2020-12, generated via `schemars` (rmcp).
- `annotations.readOnlyHint` = true for queries; `destructiveHint` = true for deletes /
  structural mutations; `idempotentHint` = true where re-calling is safe.
- All element references use `ElementId` (opaque string, stable). Human-readable names are
  NEVER used as references.
- Every mutating tool pushes one command onto the shared undo stack (see
  [mcp/01-server-design.md](./01-server-design.md) §Undo). Use `batch.*` tools to group
  multiple mutations into one undoable command.

Common error codes: see [mcp/01-server-design.md](./01-server-design.md) §Error model.

## model.* — element & relationship operations

### Structural elements

| Tool | Signature (key args) | Returns | Notes |
|------|----------------------|---------|-------|
| `model.createPackage` | `{ ownerId, name }` | `{ elementId }` | Creates a Package under `ownerId` (a Package or the root). |
| `model.createClass` | `{ ownerId, name, isAbstract?, stereotypes?[] }` | `{ elementId }` | Class in package `ownerId`. |
| `model.createInterface` | `{ ownerId, name, stereotypes?[] }` | `{ elementId }` | |
| `model.createDataType` | `{ ownerId, name }` | `{ elementId }` | |
| `model.createEnumeration` | `{ ownerId, name, literals: string[] }` | `{ elementId }` | |
| `model.createActor` | `{ ownerId, name }` | `{ elementId }` | |
| `model.createUseCase` | `{ ownerId, name }` | `{ elementId }` | |
| `model.createComponent` | `{ ownerId, name }` | `{ elementId }` | |
| `model.createNode` | `{ ownerId, name, kind: "device"\|"executionEnvironment" }` | `{ elementId }` | Deployment node. |
| `model.createArtifact` | `{ ownerId, name, kind? }` | `{ elementId }` | |

### Behavioral elements (see [uml-model/03-behavioral-diagrams.md](../uml-model/03-behavioral-diagrams.md))

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `model.createActivity` | `{ ownerId, name }` | `{ elementId }` | |
| `model.createInteraction` | `{ ownerId, name }` | `{ elementId }` | A Sequence diagram's backing Interaction. |
| `model.createStateMachine` | `{ ownerId, name }` | `{ elementId }` | |

### Requirements & tests (see [uml-model/04-requirements-and-tests.md](../uml-model/04-requirements-and-tests.md))

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `model.createRequirement` | `{ ownerId, requirementId, text, category?, priority?, status?, verificationMethods?[], source?, rationale? }` | `{ elementId }` | `requirementId` unique in project; `text` required. |
| `model.createTestCase` | `{ ownerId, testCaseId, title, description?, steps?[], expectedResult?, specificationId? }` | `{ elementId }` | `testCaseId` unique. |

### Features (attributes, operations)

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `model.addAttribute` | `{ classId, name, typeId?, visibility?, multiplicity? }` | `{ featureId }` | `typeId` = another element (DataType/Class) or a primitive name string. |
| `model.addOperation` | `{ classId, name, parameters?: [{name, typeId, multiplicity?}], returnTypeId?, visibility? }` | `{ featureId }` | |
| `model.addEnumerationLiteral` | `{ enumerationId, name }` | `{ literalId }` | |

### Relationships

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `model.createAssociation` | `{ ownerId, sourceId, targetId, sourceEnd?: {multiplicity, role}, targetEnd?: {multiplicity, role}, kind: "association"\|"aggregation"\|"composition" }` | `{ relationId }` | `ownerId` = the package owning the association (commonly the source's owner). |
| `model.createGeneralization` | `{ ownerId, specificId, generalId }` | `{ relationId }` | specific → general ("is-a"). |
| `model.createRealization` | `{ ownerId, realizingId, contractId }` | `{ relationId }` | |
| `model.createDependency` | `{ ownerId, sourceId, targetId, stereotype? }` | `{ relationId }` | Generic dependency; `stereotype` optional. |
| `model.createTraceRelation` | `{ ownerId, sourceId, targetId, stereotype: "satisfy"\|"verify"\|"realize"\|"deriveReqt"\|"refine"\|"trace"\|"copy" }` | `{ relationId }` | Validates source/target kinds per [architecture/04](../architecture/04-traceability-relations.md). Rejects with `-32001` on kind mismatch. Rejects with `-32003` if the edge would create a trace cycle (unless `acceptDefect: true`). |

### Stereotypes & tagged values

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `model.applyStereotype` | `{ elementId, stereotypeId, tagValues?: map }` | `{ appliedStereotypeId }` | Rejects `-32001` if element metaclass not in stereotype's `extendedMetaclasses`. |
| `model.removeStereotype` | `{ elementId, stereotypeId }` | `{ ok: true }` | |
| `model.setTaggedValue` | `{ elementId, stereotypeId, tagName, value }` | `{ ok: true }` | |
| `model.addComment` | `{ elementId, body, annotatedElementIds?[] }` | `{ commentId }` | `body` non-empty. |

### Profile & stereotype authoring

Profile diagrams are the only place stereotypes are defined (REQ-STRUC-010). These tools let
agents author profiles on equal footing with the UI (design principle P3 — agents and humans
are peers).

| Tool | Signature (key args) | Returns | Notes |
|------|----------------------|---------|-------|
| `model.createProfile` | `{ ownerId, name }` | `{ elementId }` | Creates a `Profile` (a `Package` specialization) under `ownerId`. |
| `model.createStereotype` | `{ profileId, name, extendedMetaclasses: [], tagDefinitions?: [] }` | `{ elementId }` | `extendedMetaclasses` are top-level kinds ([uml-model/01](../uml-model/01-metamodel-core.md) §Metaclass kind registry). |
| `model.addTagDefinition` | `{ stereotypeId, name, typeId }` | `{ featureId }` | Adds a typed tag to a stereotype; `typeId` = element or primitive name. |
| `model.createExtension` | `{ stereotypeId, metaclassKind, required? }` | `{ relationId }` | Records that `stereotypeId` extends `metaclassKind`; `required` marks a required extension. |

### Mutation & lifecycle

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `model.rename` | `{ elementId, name }` | `{ ok: true, qualifiedName }` | Returns new derived qualifiedName. |
| `model.reparent` | `{ elementId, newOwnerId }` | `{ ok: true, qualifiedName }` | Moves element in ownership tree. Rejects if it would create an ownership cycle. |
| `model.setMultiplicity` | `{ featureIdOrEndId, lower, upper? }` | `{ ok: true }` | `upper = null` = unbounded. Rejects `upper < lower`. |
| `model.setVisibility` | `{ elementId, visibility }` | `{ ok: true }` | |
| `model.delete` | `{ elementId, cascade: bool }` | `{ deletedIds: [] }` | `cascade: true` deletes owned members + cascade-deletes view references. `cascade: false` rejects if the element has owned members. Undoable. |
| `model.updateRequirement` | `{ elementId, ...fields }` | `{ ok: true }` | Partial update of Requirement fields (text, category, status, etc.). |
| `model.updateTestCase` | `{ elementId, ...fields }` | `{ ok: true }` | Partial update of TestCase fields. |
| `model.setTestStatus` | `{ testCaseId, status: "passing"\|"failing"\|"blocked"\|... }` | `{ ok: true }` | Sets the TestCase `status` (verdict). The RTM Verdict column derives from this — see [analytics/01](../analytics/01-rtm.md). |

### Queries (read-only)

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `model.get` | `{ elementId }` | `{ element }` | Full element with features, stereotypes, comments. |
| `model.getByName` | `{ qualifiedName }` | `{ element }` | Resolve by `/a/b/c` path. Error `-32000` if not found or ambiguous. |
| `model.listChildren` | `{ ownerId, kind? }` | `{ elements: [] }` | |
| `model.qualifiedName` | `{ elementId }` | `{ qualifiedName }` | |
| `model.ownerChain` | `{ elementId }` | `{ owners: [] }` | Ancestors from element to root. |

## diagram.* — view operations

Diagrams are views (see [uml-model/06-diagram-interchange.md](../uml-model/06-diagram-interchange.md)).
These tools manipulate view references, NOT model elements.

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `diagram.create` | `{ ownerId, name, kind }` | `{ diagramId }` | `kind` from DiagramKind enum. |
| `diagram.addElement` | `{ diagramId, elementId, bounds: {x,y,w,h} }` | `{ shapeId }` | Adds a ShapeView. Rejects `-32001` if element kind not allowed on this diagram kind. |
| `diagram.addRelation` | `{ diagramId, relationId, sourceShapeId, targetShapeId, bendPoints?[] }` | `{ edgeId }` | Adds an EdgeView. |
| `diagram.removeView` | `{ diagramId, viewRefId }` | `{ ok: true }` | Removes a shape/edge view. Does NOT delete the model element. |
| `diagram.moveShape` | `{ diagramId, shapeId, bounds }` | `{ ok: true }` | Update layout. |
| `diagram.setBendPoints` | `{ diagramId, edgeId, bendPoints: [] }` | `{ ok: true }` | |
| `diagram.autoLayout` | `{ diagramId, algorithm: "elk-layered", options? }` | `{ shapeBounds: map }` | Runs layout (ELK in a worker); updates all shape bounds. v1 ships ELK only; additional algorithms (dagre, graphviz-dot) are v2 candidates. |
| `diagram.export` | `{ diagramId, format: "svg"\|"png"\|"pdf" }` | `{ resourceUri, mimeType }` | Returns a resource URI for the rendered export. |
| `diagram.list` | `{ ownerId? }` | `{ diagrams: [] }` | |
| `diagram.get` | `{ diagramId }` | `{ diagram, shapes: [], edges: [] }` | Full view data. |

## analysis.* — analytics (read-only)

All `readOnlyHint: true`. See [analytics/](../analytics/) for the semantics.

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `analysis.rtm.forward` | `{ packageId?, category?, status?, coverageState? }` | `{ rows: [], metrics: {} }` | Forward RTM. See [analytics/01](../analytics/01-rtm.md). |
| `analysis.rtm.backward` | `{ packageId?, kind? }` | `{ rows: [] }` | Backward RTM. |
| `analysis.issues` | `{ severity?, code?, packageId? }` | `{ issues: [] }` | Model defects (D1–D7). See [analytics/03](../analytics/03-orphan-and-coverage-analysis.md). |
| `analysis.uncovered` | `{ packageId? }` | `{ elements: [] }` | Elements on no diagram (D4). |
| `analysis.orphanRequirements` | `{}` | `{ elements: [] }` | D1. |
| `analysis.orphanTests` | `{}` | `{ elements: [] }` | D2. |
| `analysis.circularTraces` | `{}` | `{ cycles: [[]] }` | D5; returns cycles as edge lists. |
| `analysis.incompleteChains` | `{}` | `{ requirements: [] }` | D6. |
| `analysis.disconnectedSubgraphs` | `{}` | `{ components: [[]] }` | D7. |
| `graph.adjacencyMatrix` | `{ packageId?, graph: "relationship"\|"trace" }` | `{ nodes: [], edges: [] }` | Sparse adjacency. |
| `graph.reachable` | `{ fromId, graph }` | `{ reachable: [] }` | BFS reachability. |
| `graph.transitiveClosure` | `{ graph }` | `{ pairs: [] }` | All-pairs reachability. Long-running — progress notified. |
| `graph.topoSort` | `{ graph }` | `{ order: [] }` \| `{ cycle: [] }` | |
| `graph.cycles` | `{ graph }` | `{ cycles: [[]] }` | |
| `graph.scc` | `{ graph }` | `{ components: [[]] }` | Tarjan. |
| `graph.shortestPath` | `{ fromId, toId, weighted? }` | `{ path: [] }` | BFS or Dijkstra. |
| `graph.centrality` | `{ graph, kind: "betweenness"\|"closeness"\|"both" }` | `{ scores: map }` | On demand. |
| `graph.connectedComponents` | `{ graph, mode: "weak"\|"strong" }` | `{ components: [[]] }` | |
| `trace.forward` | `{ elementId }` | `{ levels: map }` | Elements reachable downstream, grouped by chain level. |
| `trace.backward` | `{ elementId }` | `{ levels: map }` | Upstream trace. |

## search.* — search

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `search.elements` | `{ query, kind?, packageId?, limit? }` | `{ results: [{elementId, score, snippet}] }` | FTS5 + kind/package filter. See [architecture/05-search.md](../architecture/05-search.md). |
| `search.relations` | `{ query, kind?, sourceKind?, targetKind? }` | `{ results: [] }` | |
| `search.diagrams` | `{ query }` | `{ results: [] }` | |

## project.* — project lifecycle

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `project.info` | `{}` | `{ name, rootPackageId, stats: {} }` | Project metadata + counts. |
| `project.save` | `{}` | `{ ok: true }` | Flush to `.smith` file. |
| `project.export` | `{ format: "xmi"\|"json"\|"cypher", path? }` | `{ resourceUri }` | Export the model. |

## batch.* — composite operations (single undo unit)

| Tool | Signature | Returns | Notes |
|------|-----------|---------|-------|
| `batch.run` | `{ operations: [{tool, args}] }` | `{ results: [] }` | Runs a sequence of tools as one undoable command. All-or-nothing: if any op fails, the batch rolls back. |
| `batch.undo` | `{}` | `{ ok: true }` | Pop the undo stack. Shared with UI undo. |
| `batch.redo` | `{}` | `{ ok: true }` | |

## Annotations (per tool)

`[REQ-MCP-TOOLS-001]` Every tool MUST declare `annotations`:
- All `model.create*`, `model.add*`, `model.delete`, `model.rename`, `model.reparent`,
  `diagram.*` (mutations), `batch.*`: `destructiveHint: true` (they change the model).
- All `analysis.*`, `graph.*` (read), `search.*`, `model.get*`, `model.list*`,
  `diagram.list`/`get`, `project.info`: `readOnlyHint: true`.
- `model.delete` with `cascade: true`: `destructiveHint: true` AND the tool description MUST
  warn that it cascade-deletes owned members and view references.

## Result shape

`[REQ-MCP-TOOLS-002]` Mutating tools return `{ ok: true, ...ids }` plus the affected
`ElementId`s so the agent can reference the results.

`[REQ-MCP-TOOLS-003]` Every mutating tool MUST emit a `notifications/resources/updated` for
each affected resource URI (element, diagram, analysis) on the SSE stream, so subscribed
agents and the UI re-fetch.

## Example: creating a traced requirement → usecase

```jsonc
// 1. Create requirement
{ "tool": "model.createRequirement",
  "args": { "ownerId": "<req-pkg>", "requirementId": "REQ-001",
            "text": "System shall authenticate users via OAuth2.",
            "category": "security", "priority": "must", "status": "approved",
            "verificationMethods": ["test", "demonstration"] } }
// → { "elementId": "req-abc" }

// 2. Create use case
{ "tool": "model.createUseCase",
  "args": { "ownerId": "<uc-pkg>", "name": "Authenticate" } }
// → { "elementId": "uc-xyz" }

// 3. Trace: usecase satisfies requirement
{ "tool": "model.createTraceRelation",
  "args": { "ownerId": "<uc-pkg>", "sourceId": "uc-xyz", "targetId": "req-abc",
            "stereotype": "satisfy" } }
// → { "relationId": "rel-001" }
```

## Open questions (resolve before STABLE)

- [ ] Should `batch.run` support a declarative DSL or just a list of tool calls? Current:
      list of tool calls (simplest, composable).
- [ ] Pagination for `model.listChildren` and `search.*` via MCP `cursor` — yes for search
      (can be large); no for listChildren (usually small).
- [ ] Should `analysis.*` tools accept a `progressToken` for long computations? Yes —
      per [mcp/01](./01-server-design.md) §Concurrency.
