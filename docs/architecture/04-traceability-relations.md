---
status: DRAFT
---

# Architecture 04 — Traceability Relations

Smith extends UML with a first-class **traceability relation taxonomy**. Every relation is a
`Dependency` (OMG UML §7.8, §19.4) specialized by a **stereotype**. Relations are edges in
the model graph; their stereotype determines traceability semantics and which analytical
queries treat them as part of the trace graph.

This document is the **normative** list of trace relations Smith supports. Background and
external references live in [research/05-traceability-and-analytics.md](../research/05-traceability-and-analytics.md).

## Base metaclass

Every trace relation is a `Dependency` (UML §19.4) with:

- `source` — the **client** element (the depending element), a model element ID.
- `target` — the **supplier** element (the element depended upon), a model element ID.
- `stereotype` — one of the stereotypes below, applied via a Smith built-in profile
  (`smith::traceability`).
- `id` — stable, opaque.
- `owner` — the namespace owning this dependency (typically the source element's package).

Direction convention (storage, normative): **source → target**. Each row's "Source" and
"Target" columns below are authoritative for the stored edge direction. Per SysML §16.3.2.x,
satisfy/verify/realize point FROM the design/test element TO the requirement (or contract);
deriveReqt points FROM the parent requirement TO the derived child. This is the single
source of truth — all chain diagrams elsewhere use "reading order" arrows that may point the
opposite way; when a diagram and this table disagree, **the table wins**.

## Relation taxonomy

Each row: stereotype | source kinds | target kinds | semantics | what it proves.

| Stereotype | Source (client) | Target (supplier) | Semantics | Proves |
|------------|-----------------|-------------------|-----------|-------|
| `«satisfy»` | Design element: `UseCase`, `Class`, `Component`, `Activity`, `Interaction` | `Requirement` | The source element satisfies the target requirement. | Requirement is addressed. |
| `«verify»` | `TestCase` | `Requirement` | The source TestCase verifies the target requirement. | Requirement is tested. |
| `«realize»` | Implementing element: `Activity`→`UseCase`, `Interaction`→`Activity`, `Class`/`Operation`→`Interaction` | The contract element it realizes | The source implements the target's specified behavior/contract. | Implementation completeness. |
| `«deriveReqt»` | `Requirement` (parent) | `Requirement` (derived child) | The target is derived from the source (decomposition into a lower-level requirement). Per SysML §16.3.2.3. | Requirement hierarchy (parent → child). |
| `«refine»` | Any element | Any element | The source refines the target at a more detailed level of abstraction. | Refinement history. |
| `«trace»` | Any element | Any element | Generic trace; no specific contract, just "these are related across levels". | Loose coupling, no semantic commitment. |
| `«copy»` | Any element | Any element | The target is a copy of the source (e.g. reused model fragment). Per SysML §16.3.2.2. | Provenance. Excluded from RTM. |

## Cardinality rules

- Every relation is a single edge: one source, one target. N-ary traceability is modeled as
  N binary relations.
- A `Requirement` MAY have:
  - 0..N outgoing `«deriveReqt»` (parent → derived children).
  - 0..N incoming `«satisfy»`, `«verify»`, `«realize»` (design/test elements that address it).
- A `TestCase` MUST have ≥1 outgoing `«verify»` (a test verifying nothing is an orphan test;
  see analytics).
- A `Requirement` with **zero** incoming `«satisfy»` AND **zero** incoming `«verify»` AND
  **zero** incoming `«realize»` AND **zero** outgoing `«deriveReqt»` is an **orphan
  requirement** (quality defect D1, severity HIGH — see
  [analytics/03-orphan-and-coverage-analysis.md](../analytics/03-orphan-and-coverage-analysis.md)).
  Rationale: a requirement addressed only by derivation is acceptable; one with no link at
  all is unaddressed.

## The canonical chain

A complete model traces every requirement along this chain to implementation:

```
                      (stored edges point downward; arrows show reading order)

   parent Requirement ──«deriveReqt»──> child Requirement(s)     [source → target]

   UseCase ──«satisfy»──> Requirement                              [source → target]
   Activity ──«realize»──> UseCase                                 [source → target]
   Interaction ──«realize»──> Activity                             [source → target]
   Class/Operation ──«realize»──> Interaction                      [source → target]
   TestCase ──«verify»──> Requirement                              [source → target]

   Reading the chain from requirement to implementation follows «satisfy»/«realize» in
   REVERSE (target → source), because those edges point AT the requirement.
```

Reading the chain (from requirement outward to implementation): a **Requirement** is
*satisfied by* one or more **UseCases** (follow incoming `«satisfy»` edges in reverse);
each UseCase is *realized by* one or more **Activities** (follow incoming `«realize»` edges
in reverse); each Activity is *realized by* one or more **Interactions** (same, in reverse);
each Interaction is *realized by* the **Classes and Operations** that collaborate (same, in
reverse). **TestCases** *verify* requirements (follow incoming `«verify»` edges in reverse).
Requirement decomposition follows `«deriveReqt»` forward (parent → children).


This chain is what the RTM and the "incomplete chain" analysis traverse. See
[analytics/01-rtm.md](../analytics/01-rtm.md) and
[analytics/03-orphan-and-coverage-analysis.md](../analytics/03-orphan-and-coverage-analysis.md).

## Trace graph vs. relationship graph

Smith maintains two derived subgraphs over the model's relationship edges:

- **Relationship graph** — ALL relationship edges (associations, generalizations,
  dependencies, trace relations, connectors, messages). Used for adjacency matrix, SCC,
  centrality.
- **Trace graph** — ONLY trace-relation edges (`«satisfy»`, `«realize»`, `«verify»`,
  `«deriveReqt»`, `«refine»`, `«trace»`, `«copy»`). Used for RTM, trace reachability, chain
  completeness, circular-trace detection.

Both are **derived** from the stored model; they are never written directly. Any model
mutation updates them incrementally.

## Quality rules attached to trace relations

Normative references to the analytics specs:

| Defect | Condition | Severity | Spec |
|--------|-----------|----------|------|
| Orphan requirement | Requirement with no incoming `«satisfy»`/`«verify»`/`«realize»` AND no outgoing `«deriveReqt»`. | HIGH | [analytics/03](../analytics/03-orphan-and-coverage-analysis.md) |
| Orphan test | TestCase with no outgoing `«verify»`. | HIGH | [analytics/03](../analytics/03-orphan-and-coverage-analysis.md) |
| Circular trace | Cycle in the trace graph. | HIGH | [analytics/03](../analytics/03-orphan-and-coverage-analysis.md) |
| Incomplete chain | Requirement with no path to a `Class`/`Operation` along the canonical chain. | MEDIUM | [analytics/03](../analytics/03-orphan-and-coverage-analysis.md) |
| Disconnected subgraph | Connected component of the trace graph with no `Requirement` node. | MEDIUM | [analytics/03](../analytics/03-orphan-and-coverage-analysis.md) |

## Stereotype lifecycle

Trace stereotypes are **built-in**: they ship with Smith in the `smith::traceability` profile
and cannot be deleted or redefined by the user. They MAY be composed with user-defined
stereotypes (a relation can carry a trace stereotype AND a user stereotype), but the trace
semantics come only from the built-in stereotype.

## Open questions (resolve before STABLE)

- [ ] Do we allow `«realize»` from a `Class` directly to a `UseCase` (skipping the
      behavioral chain), or strictly Activity → UseCase / Class → Interaction? Current spec:
      the taxonomy permits any implementing → contract pair; the chain is the *preferred*
      shape, not the only legal one. See REQ-CHAIN-002.
- [x] `«copy»` is **provenance only; excluded from RTM and the trace graph's chain
      analysis** (DECIDED). It remains in the relationship graph for adjacency/centrality.
      Including it in RTM would conflate provenance with traceability semantics.
