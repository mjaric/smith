---
status: DRAFT
---

# MCP 03 — Resources

Readable resources Smith exposes to MCP clients. Resources let an agent **read** model state
without mutating it (the `model.get*` tools also read, but resources are the idiomatic MCP
way to expose addressable, subscribable, cacheable data).

See [research/02-mcp-transport.md](../research/02-mcp-transport.md) §4.2 for the resource
protocol primitives.

## URI scheme

`[REQ-MCP-RES-001]` Smith defines these URI schemes:

| Scheme | Example | Meaning |
|--------|---------|---------|
| `smith://elements/{elementId}` | `smith://elements/req-abc` | A single element (full representation). |
| `smith://elements/{elementId}/features` | `smith://elements/class-1/features` | The features (attributes, operations) of a classifier. |
| `smith://packages/{packageId}/children` | `smith://packages/pkg-1/children` | Direct children of a package. |
| `smith://packages/{packageId}/tree` | `smith://packages/pkg-1/tree` | Full ownership subtree (recursive). |
| `smith://diagrams` | `smith://diagrams` | List of all diagrams. |
| `smith://diagrams/{diagramId}` | `smith://diagrams/diag-1` | A diagram's full view data (shapes, edges). |
| `smith://diagrams/{diagramId}/export?format=svg` | | Rendered export of a diagram. |
| `smith://analysis/rtm` | `smith://analysis/rtm` | Forward RTM (full). |
| `smith://analysis/rtm/backward` | | Backward RTM. |
| `smith://analysis/issues` | `smith://analysis/issues` | Current model defects. |
| `smith://analysis/uncovered` | | Uncovered elements. |
| `smith://analysis/adjacency?graph=trace` | | Adjacency matrix. |
| `smith://project/info` | `smith://project/info` | Project metadata + stats. |
| `smith://project/profiles` | | Built-in + applied profiles. |

`[REQ-MCP-RES-002]` Element IDs in URIs are the opaque stable `ElementId` strings. Agents
discover IDs via tools (`model.create*` returns IDs; `search.elements` returns IDs;
`model.listChildren` returns IDs) and then read resources by ID.

`[REQ-MCP-RES-003]` Unknown-element URIs return error `-32602` (Invalid Params) with a clear
message; unknown URI paths return `-32601` (method not found).

## Resource list

`[REQ-MCP-RES-004]` `resources/list` returns all listable resources. For element-scoped
resources (which are open-ended), Smith returns:
- All packages (`smith://packages/{id}/tree` for each top-level package).
- All diagrams (`smith://diagrams/{id}`).
- The analysis resources.
- The project info resource.

`[REQ-MCP-RES-005]` Smith MUST also expose `resources/templates/list` with RFC 6570 URI
templates so clients can construct element-scoped URIs and autocomplete arguments:

```
smith://elements/{elementId}
smith://packages/{packageId}/children
smith://diagrams/{diagramId}
```

## Resource representation

`[REQ-MCP-RES-006]` Each resource's `resources/read` returns JSON (`mimeType:
application/json`) with a stable, documented shape:

### `smith://elements/{elementId}` — Element
```jsonc
{
  "element": {
    "id": "req-abc",
    "kind": "Requirement",
    "name": "OAuth2 authentication",
    "ownerId": "pkg-req",
    "qualifiedName": "/System/Requirements/OAuth2 authentication",
    "visibility": "public",
    "stereotypes": [{ "stereotypeId": "...", "tagValues": {} }],
    "comments": [{ "id": "...", "body": "..." }],
    // kind-specific fields:
    "requirementId": "REQ-001",
    "text": "System shall authenticate users via OAuth2.",
    "category": "security",
    "priority": "must",
    "status": "approved",
    "verificationMethods": ["test", "demonstration"]
  }
}
```

`[REQ-MCP-RES-007]` The `kind` field names the metaclass; the remaining fields are
kind-specific and documented per element type in [uml-model/](../uml-model/).

### `smith://diagrams/{diagramId}` — Diagram view
```jsonc
{
  "diagram": { "id": "diag-1", "name": "Login sequence", "kind": "sequence",
               "ownerId": "pkg-seq", "canvasState": {"panX":0,"panY":0,"zoom":1} },
  "shapes": [ { "id": "shape-1", "modelElementId": "lifeline-1",
                "bounds": {"x":100,"y":80,"w":120,"h":400},
                "presentationHints": {} } ],
  "edges":  [ { "id": "edge-1", "modelElementId": "msg-1",
                "sourceShapeId": "shape-1", "targetShapeId": "shape-2",
                "bendPoints": [], "routingKind": "orthogonal" } ]
}
```

### `smith://analysis/issues` — Issues
```jsonc
{ "issues": [
    { "code": "D1", "severity": "HIGH", "elementId": "req-orphan",
      "message": "Requirement REQ-099 has no satisfying/verifying relation.",
      "suppressed": false },
    ...
  ],
  "counts": { "HIGH": 1, "MEDIUM": 0, "LOW": 3 } }
```

### `smith://project/info` — Project
```jsonc
{ "name": "E-commerce v2", "rootPackageId": "pkg-root",
  "smithVersion": "1.0.0",
  "stats": { "elements": 482, "relationships": 631, "diagrams": 24,
             "requirements": 57, "testCases": 43 } }
```

## Subscriptions & change notifications

`[REQ-MCP-RES-008]` Agents MAY subscribe to any resource via `resources/subscribe`. On
mutation, Smith emits `notifications/resources/updated {uri}` for every affected resource.
Affected-resource computation:

| Mutation | Resources updated |
|----------|-------------------|
| Element field change | `smith://elements/{id}`, plus `smith://elements/{id}/features` if a feature changed, plus any `smith://diagrams/{diagId}` showing it, plus `smith://analysis/*` that depend on it. |
| Relationship add/remove | Both endpoint `smith://elements/{id}`, all showing diagrams, trace-sensitive analyses (RTM, issues, adjacency?graph=trace). |
| View-reference change | The containing `smith://diagrams/{diagId}`, and `smith://analysis/uncovered` (if an element's coverage changed). |
| Package reparent | The reparented subtree's `smith://packages/{id}/tree`, the old and new owner's `children`. |

`[REQ-MCP-RES-009]` Smith MUST NOT over-notify: if a single batch command touches 100
elements, it emits at most 100 `elements/{id}` updates + a bounded number of analysis
updates, deduplicated.

## Resource size limits

`[REQ-MCP-RES-010]` The `resources/read` response for a single resource MUST stay under
1 MiB. Resources that can exceed this (full RTM on a 10k-requirement project, full
adjacency matrix) MUST paginate via query params (`?offset=0&limit=500`) or return a
reference to a `smith://analysis/...` resource that the agent reads in pages.

## Open questions (resolve before STABLE)

- [ ] Should Smith expose a `smith://search?q=...` resource (readable search results), or is
      the `search.elements` tool sufficient? Current: tool only; resources are for stable
      addresses, not query results.
- [ ] Versioned resources (`smith://elements/{id}?at=<commit>`)? Out of scope v1 (no
      versioning).
