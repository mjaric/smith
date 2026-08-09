---
status: DRAFT
---

# Analytics 03 — Orphan & Coverage Analysis

Smith continuously detects **model-quality defects** and surfaces them in the issue panel.
This document defines each defect, its detection condition, severity, and the algorithm that
detects it. References: [research/05-traceability-and-analytics.md](../research/05-traceability-and-analytics.md) §4.

## Defect taxonomy

| ID | Defect | Severity | Continuous? |
|----|--------|----------|-------------|
| D1 | Orphan requirement | HIGH | yes |
| D2 | Orphan test (TestCase verifies nothing) | HIGH | yes |
| D3 | Orphan element (no owner path to root, or no relationships at all) | MEDIUM | yes |
| D4 | Uncovered element (on no diagram) | LOW | yes |
| D5 | Circular trace (cycle in trace graph) | HIGH | yes |
| D6 | Incomplete chain (requirement with no path to Class/Operation) | MEDIUM | yes |
| D7 | Disconnected subgraph (trace component with no Requirement) | MEDIUM | on demand |

`[REQ-ORPH-001]` Smith MUST detect D1–D6 continuously (incrementally on each model mutation)
and surface them in the issue panel immediately. D7 is computed on demand (it's a
graph-wide query; cheap enough but not needed on every keystroke).

## D1 — Orphan requirement

**Condition:** A `Requirement` with zero incoming `«satisfy»`, `«verify»`, AND `«realize»`
relations AND zero outgoing `«deriveReqt»`.

Rationale: a requirement that nothing satisfies, verifies, or realizes, and that derives no
sub-requirements, is an unaddressed spec — either delete it or link it.

`[REQ-ORPH-D1-001]` Detection: in-degree test on the requirement node in the trace subgraph
(edge kinds: satisfy, verify, realize) AND out-degree test for deriveReqt. O(1) per node.

`[REQ-ORPH-D1-002]` Severity: HIGH. The issue panel lists orphan requirements first.

## D2 — Orphan test

**Condition:** A `TestCase` with zero outgoing `«verify»` relations.

Rationale: a test that verifies no requirement is dead test coverage.

`[REQ-ORPH-D2-001]` Detection: out-degree test on the TestCase node for `«verify»` edges. O(1).

`[REQ-ORPH-D2-002]` Severity: HIGH.

## D3 — Orphan element

**Condition:** An element that is either:
(a) not reachable from the model root via the ownership tree (corrupt/dangling owner edge), OR
(b) has zero incoming AND zero outgoing relationships of any kind (isolated node).

Rationale: (a) indicates model corruption; (b) indicates an element that was created but
never connected — likely a mistake.

`[REQ-ORPH-D3-001]` Detection (a): ownership-tree reachability — every element MUST be
reachable from the root Package via `owner` edges. This is an invariant
([uml-model/01-metamodel-core.md](../uml-model/01-metamodel-core.md) INV-MM-001); a violation
is model corruption, not just a defect.

`[REQ-ORPH-D3-002]` Detection (b): total-degree test (in + out) across ALL relationship kinds
on the relationship graph. O(1) per node.

`[REQ-ORPH-D3-003]` Severity: MEDIUM for isolated nodes (case b); model corruption (case a)
is a fatal error and MUST be surfaced as a blocking issue.

`[REQ-ORPH-D3-004]` Exclusions from the isolated-node check: `Package`, `Profile`,
`Stereotype`, `Comment`, `Diagram`, and the project root. These are legitimately
relationship-free. The check applies only to modelable content elements (classes, use cases,
requirements, etc.).

## D4 — Uncovered element

**Condition:** A modelable element (see D3 exclusions) referenced by NO diagram (zero view
references across all diagrams).

This is the user's explicit ask: "know which element is not shown on any diagram."

Rationale: an element exists in the model but is invisible — either delete it or put it on a
diagram.

`[REQ-ORPH-D4-001]` Detection: set difference — `modelableElements − elementsReferencedByAnyDiagram`.
Computed from the coverage index (see
[uml-model/06-diagram-interchange.md](../uml-model/06-diagram-interchange.md) §Coverage),
which is maintained incrementally on view-reference add/remove.

`[REQ-ORPH-D4-002]` Severity: LOW (informational). The issue panel groups uncovered elements
by owning package for easy triage.

`[REQ-ORPH-D4-003]` The uncovered-element check MUST be available as a filter in the model
explorer (dim or badge uncovered elements) and as an MCP tool (`analysis.uncovered`).

## D5 — Circular trace

**Condition:** A cycle in the trace graph (subgraph of trace-relation edges).

Rationale: traceability must be acyclic (DAG). A cycle means a requirement (transitively)
depends on its own satisfaction — logically incoherent.

`[REQ-ORPH-D5-001]` Detection: DFS cycle detection (or equivalently, any SCC with >1 vertex
in the trace graph). Reference:
[Cycle detection](https://en.wikipedia.org/wiki/Cycle_detection).

`[REQ-ORPH-D5-002]` The detection MUST identify the exact cycle (list of edges) so the user
can break it.

`[REQ-ORPH-D5-003]` Severity: HIGH. Adding a trace edge that would create a cycle is rejected
by default unless the caller passes `acceptDefect: true` (MCP error `-32003`; UI shows a
confirmation dialog).

## D6 — Incomplete chain

**Condition:** A `Requirement` with no directed path (along trace edges) to any
`Class`/`Operation`.

Rationale: the canonical chain
`Requirement → UseCase → Activity → Sequence → Class` should terminate at implementation. A
requirement that never reaches a Class is specified-but-not-implemented.

`[REQ-ORPH-D6-001]` Detection: reachability (BFS) from the requirement following
`«satisfy»`/`«realize»` in **reverse** (target → source), because those stored edges point AT
the requirement; check if any reached node is a Class/Operation. Reference:
[Reachability](https://en.wikipedia.org/wiki/Reachability).

`[REQ-ORPH-D6-002]` Severity: MEDIUM. Surfaced in the RTM as the coverage state
`satisfied`/`realized` (if reached an Activity/Interaction but not a Class) vs `untraced`.

`[REQ-ORPH-D6-003]` Note: a direct `Requirement → Class` path (skipping levels) is valid and
counts as complete (see [uml-model/05-traceability-chain.md](../uml-model/05-traceability-chain.md)
REQ-CHAIN-002).

## D7 — Disconnected subgraph

**Condition:** A weakly connected component of the trace graph that contains zero
`Requirement` nodes.

Rationale: a cluster of elements traced among themselves but to no requirement is orphaned
design — either link it to a requirement or delete it.

`[REQ-ORPH-D7-001]` Detection: weakly connected components on the trace graph. Reference:
[Connected component](https://en.wikipedia.org/wiki/Connected_component_(graph_theory)).

`[REQ-ORPH-D7-002]` Severity: MEDIUM. Computed on demand (not continuous — graph-wide query).

## Severity model

`[REQ-SEV-001]` Severities: `HIGH` (blocking — model is wrong or dangerously incomplete),
`MEDIUM` (should fix — model quality issue), `LOW` (informational — nice to fix).

`[REQ-SEV-002]` The issue panel MUST sort by severity (HIGH first) and MUST allow filtering.
HIGH defects MAY optionally block project export (configurable).

`[REQ-SEV-003]` Each defect carries a stable `defectCode` (D1–D7) for programmatic reference
(testing, MCP consumers, suppression rules).

## Suppression

`[REQ-ORPH-SUP-001]` A user MAY suppress a specific defect instance (e.g. "this uncovered
element is intentional"). Suppression is recorded on the element as a tagged value
(`smith::suppressDefect` with the defect code) and persists with the model. Suppressed
defects do not appear in the default issue-panel view but are visible in a "show suppressed"
filter.

`[REQ-ORPH-SUP-002]` Suppression is per-element-per-defect-code, not global. Suppressing D4
on element X does not suppress D1 on X.

## Incremental maintenance

`[REQ-ORPH-INC-001]` D1, D2, D3(b), D4, D5, D6 MUST be maintained incrementally:
- On element creation: evaluate defects for the new element.
- On relationship add/remove: re-evaluate defects for affected endpoints.
- On view-reference add/remove: re-evaluate D4 for the referenced element.

`[REQ-ORPH-INC-002]` D3(a) (ownership reachability) is an invariant enforced at the model
API; it cannot arise from a legal mutation. D7 is computed on demand.

## MCP surface

`[REQ-ORPH-MCP-001]` These defects MUST be exposed via MCP:
- `analysis.issues` tool — returns current defects (filterable by severity/code/package).
- `smith://analysis/issues` resource — same data, readable.
- Each defect references the offending element(s) by `ElementId` and the relevant relation(s).

## Open questions (resolve before STABLE)

- [ ] Should D6 distinguish "reached Activity but not Class" from "reached nothing"? Current
      spec: yes, via the coverage state in RTM (satisfied vs realized vs untraced), but D6
      itself is a single MEDIUM defect covering all non-complete cases.
- [ ] Auto-fix suggestions (e.g. "create a UseCase for this orphan requirement")? Out of
      scope v1; the issue panel links to the relevant create-tool instead.
