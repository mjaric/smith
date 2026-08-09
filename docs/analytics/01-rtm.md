---
status: DRAFT
---

# Analytics 01 — Requirements Traceability Matrix (RTM)

The RTM is the primary analytical artifact Smith produces from the trace graph. It answers
two questions, bidirectionally:

- **Forward:** Is every requirement satisfied and verified?
- **Backward:** What requirement justifies this design artifact / test case?

References: [research/05-traceability-and-analytics.md](../research/05-traceability-and-analytics.md) §2.
Standards: ISO/IEC/IEEE 29148:2018, ISO/IEC TR 24766:2009, IEEE 830-1998, PMI RTM practice.

## Inputs

The RTM is a **derived view** computed from the trace graph (a subgraph of the model
containing only trace-relation edges — see
[architecture/04-traceability-relations.md](../architecture/04-traceability-relations.md)).
It is NEVER stored as primary data; it is recomputed on demand or incrementally on trace
mutations.

`[REQ-RTM-001]` The RTM MUST be computable from:
- The set of `Requirement` elements.
- The set of trace relations (`«satisfy»`, `«verify»`, `«realize»`, `«deriveReqt»`, `«refine»`,
  `«trace»`, `«copy»`) — see
  [architecture/04-traceability-relations.md](../architecture/04-traceability-relations.md).
- The `TestCase.status` (passing / failing / blocked / not-run / etc.) of each `TestCase` that
  is the **source** of a `«verify»` edge. There is **no** per-relation verdict field — the
  verdict for a requirement is *derived* from the statuses of its verifying TestCases.

## Two views, one graph

Smith renders two views from the same underlying graph:

### Forward RTM (requirement-centric)
One row per `Requirement`. Columns:

| Column | Source | Meaning |
|--------|--------|---------|
| Requirement ID | `Requirement.requirementId` | Stable human ID. |
| Name / Text | `Requirement.name`, `Requirement.text` | Human description. |
| Source | `requirementId` of the source (parent) `Requirement` on each incoming `«deriveReqt»` edge | Parent requirement(s). |
| Satisfying artifacts | incoming `«satisfy»` + `«realize»` sources | Design elements that satisfy this requirement. |
| Verifying tests | incoming `«verify»` sources | TestCases that verify this requirement. |
| Verdict | aggregate of `TestCase.status` for TestCases with an outgoing `«verify»` to this requirement | `passing` if all verifying TestCases are `passing`; `failing` if any is `failing`; else `untested`. |
| Coverage state | derived (see [uml-model/05-traceability-chain.md](../uml-model/05-traceability-chain.md)) | `untraced` \| `satisfied` \| `realized` \| `verified` \| `complete`. |
| % chain complete | derived | Fraction of canonical-chain levels that have ≥1 element. |

### Backward RTM (artifact-centric)
One row per non-requirement element that has ≥1 trace relation. Columns:

| Column | Source | Meaning |
|--------|--------|---------|
| Artifact kind | element metaclass | Class / UseCase / Activity / Interaction / TestCase / etc. |
| Artifact name/ID | element `name`, `id` | Identity. |
| Owner | qualifiedName of owning package | Where it lives in the model tree. |
| Traced requirements | targets of outgoing `«satisfy»`/`«verify»`/`«realize»`/`«trace»` | Requirements this artifact serves. |
| Orphan flag | derived | `true` if no traced requirement (gold-plating). |

`[REQ-RTM-002]` Both views MUST be available as MCP analysis tools
(`analysis.rtm.forward`, `analysis.rtm.backward`) returning structured results, and as UI
panels (see [ui-ux/04-search-and-analysis-ui.md](../ui-ux/04-search-and-analysis-ui.md)).

## Coverage state machine

Derived from [uml-model/05-traceability-chain.md](../uml-model/05-traceability-chain.md):

```
untraced ──(+satisfy/verify/realize)──> satisfied ──(path to Class)──> realized ──(+verify pass)──> verified ──(no defects)──> complete
```

"+verify pass" = a `TestCase` whose `status` is `passing` has an outgoing `«verify»` to this
requirement (no per-relation verdict field — see REQ-RTM-001).

`[REQ-RTM-003]` The RTM MUST show the coverage state per requirement and MUST allow filtering
by state (e.g. "show only untraced requirements").

## Aggregate metrics

`[REQ-RTM-004]` The RTM view MUST show these rollups at the top:

| Metric | Definition |
|--------|------------|
| Total requirements | Count of `Requirement` elements. |
| % satisfied | Requirements with ≥1 incoming `«satisfy»` or `«realize»` / total. |
| % verified | Requirements with ≥1 `«verify»` from a `TestCase` whose `status` is `passing` / total. |
| % chain complete | Requirements with a full path to Class/Operation / total. |
| Orphan requirements | Count (see [analytics/03](./03-orphan-and-coverage-analysis.md)). |
| Orphan artifacts (gold-plating) | Non-requirement elements with no trace to any requirement. |

## Algorithms

All RTM computations are degree and reachability queries on the trace graph
([research/05](../research/05-traceability-and-analytics.md) §3, algorithms 1–2):

- **Coverage state per requirement**: BFS/reachability from the requirement following
  `«satisfy»`/`«realize»` in **reverse** (target → source), since those stored edges point AT
  the requirement, to check for a path to a Class/Operation. `«deriveReqt»` is followed forward.
- **Verdict rollup**: aggregation over the `TestCase.status` of each TestCase with an outgoing
  `«verify»` to the requirement (no per-relation field).
- **Aggregate metrics**: count queries over element kinds and edge degrees.

`[REQ-RTM-005]` For models under 10⁵ elements, the full RTM MUST compute in under 500 ms
(in-memory `petgraph`, single-threaded). Incremental updates on a single trace-edge mutation
MUST recompute only the affected requirements' rows (under 10 ms).

## Filtering and export

`[REQ-RTM-006]` The RTM MUST support filtering by:
- Requirement package (ownership subtree).
- Requirement category / priority / status.
- Coverage state.
- Verdict.

`[REQ-RTM-007]` The RTM MUST be exportable as CSV and as a model resource
(`smith://analysis/rtm`) readable via MCP `resources/read`.

## Defects surfaced by RTM

The RTM is the primary surface for these defects (defined in
[analytics/03-orphan-and-coverage-analysis.md](./03-orphan-and-coverage-analysis.md)):

- Orphan requirement (no satisfying artifact).
- Orphan artifact / gold-plating (no traced requirement).
- Incomplete chain (requirement with no path to Class/Operation).
- Circular trace (cycle in trace graph — RTM computation must detect and flag, not infinite-loop).

`[REQ-RTM-008]` RTM computation MUST be cycle-safe: if the trace graph has a cycle, the RTM
MUST mark affected requirements as `circular-trace` defect rather than hanging.

## Open questions (resolve before STABLE)

- [ ] Weighted coverage (a requirement with 3 of 5 UseCases traced): show as 60%? Tentative:
      yes, as an optional column, not replacing the discrete coverage state.
- [ ] Should RTM support **time-travel** (show the RTM as of a past commit)? Out of scope v1;
      requires versioning. Tentative: no.
