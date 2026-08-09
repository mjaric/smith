---
status: DRAFT
---

# UML Model 05 — Traceability Chain

The traceability chain is Smith's reason for existing as more than a drawing tool. This
document specifies the **canonical chain**, what completeness means, and how every diagram
kind participates.

See also:
- [architecture/04-traceability-relations.md](../architecture/04-traceability-relations.md) — the relation taxonomy.
- [analytics/01-rtm.md](../analytics/01-rtm.md) — the RTM built from the chain.
- [analytics/03-orphan-and-coverage-analysis.md](../analytics/03-orphan-and-coverage-analysis.md) — chain-completeness defects.

## The canonical chain (restated)

```
   Requirement
       │
       │  «satisfy»          (UseCase satisfies Requirement)
       ▼
    UseCase
       │
       │  «realize»          (Activity realizes UseCase)
       ▼
    Activity
       │
       │  «realize»          (Interaction realizes Activity)
       ▼
  Interaction (Sequence)
       │
       │  «realize»          (Class/Operation realizes Interaction)
       ▼
  Class / Operation

  TestCase ──«verify»──> Requirement   (cross-cutting; tests verify any requirement)
```

> **Reading note.** Arrows show reading order (requirement → implementation); the **stored
> trace edges point the opposite direction** — see
> [architecture/04 §Relation taxonomy](../architecture/04-traceability-relations.md#relation-taxonomy)
> for the authoritative source → target direction.

## Per-level semantics

| Level | Element kind | Diagram kind that defines it | What "complete at this level" means |
|-------|--------------|------------------------------|--------------------------------------|
| 0 | `Requirement` | Requirement diagram | Has `text`, `category`, `status`, and ≥1 verification method. |
| 1 | `UseCase` | Use Case diagram | ≥1 outgoing `«satisfy»` to a Requirement; ≥1 incoming `«realize»` from an Activity. |
| 2 | `Activity` | Activity diagram | ≥1 outgoing `«realize»` to a UseCase; ≥1 incoming `«realize»` from an Interaction. |
| 3 | `Interaction` | Sequence diagram | ≥1 outgoing `«realize»` to an Activity; ≥1 incoming `«realize»` from a Class/Operation. |
| 4 | `Class` / `Operation` | Class diagram | Participates in ≥1 outgoing `«realize»` to an Interaction. |
| T | `TestCase` | Requirement diagram (or annotation) | ≥1 outgoing `«verify»` to a Requirement. |

`[REQ-CHAIN-001]` The chain is a **directed acyclic graph** (DAG) over the trace graph.
Cycles are defects (see [analytics/03](../analytics/03-orphan-and-coverage-analysis.md)).

`[REQ-CHAIN-002]` Levels are NOT a hard constraint on element kinds — an Activity may
directly `«realize»` a Requirement, skipping UseCases, for low-level requirements. The
chain is the **preferred** structure, not the only legal one. Completeness checks honor
this (a direct Requirement→Activity→Class path is still "complete").

## Forward and backward trace

`[REQ-CHAIN-003]` **Forward trace** from a Requirement = all elements reachable by following
`«satisfy»`, `«realize»`, and `«verify»` in the **reverse** (target → source) direction, out to
UseCases, Activities, Interactions, Classes/Operations, and TestCases. A Requirement has **no**
outgoing `«satisfy»`/`«realize»`/`«verify»` edges (those point AT it — see
[architecture/04](../architecture/04-traceability-relations.md)). `«deriveReqt»` is followed
forward (source → target, parent → child).

`[REQ-CHAIN-004]` **Backward trace** from a Class/Operation or TestCase = all elements reachable
by following `«realize»` (from a Class/Operation) or `«verify»` (from a TestCase) in the
**source → target** direction, back through Interactions, Activities, and UseCases to
Requirements. This is the mirror of forward trace: stored trace edges point from the
implementing/test element toward the requirement, so backward trace follows them as stored.

Both are BFS over the trace graph (see
[research/05-traceability-and-analytics.md](../research/05-traceability-and-analytics.md)).

## Completeness levels

For a single Requirement, Smith computes a completeness score:

| State | Condition |
|-------|-----------|
| **Untraced** | No incoming `«satisfy»`/`«verify»`/`«realize»`. (orphan requirement) |
| **Satisfied** | ≥1 incoming `«satisfy»` but no path to a Class/Operation. |
| **Realized** | Has a path to a Class/Operation but no incoming `«verify»` from a TestCase. |
| **Verified** | Has a path to a Class/Operation AND ≥1 `«verify»` from a TestCase whose status is `passing`. See [analytics/01-rtm.md](../analytics/01-rtm.md). |
| **Complete** | Verified AND no defects (no cycle, no uncovered element on the path). |

`[REQ-CHAIN-005]` The completeness state of every Requirement MUST be computable on demand
and surfaced in the RTM and in the issue panel.

`[REQ-CHAIN-006]` The transition between states is driven by adding/removing trace
relations; Smith recomputes incrementally (affected Requirements only) on each mutation.

## Diagram participation

Every diagram kind may host trace relations as edges:

| Diagram | Trace edges shown | Typical use |
|---------|-------------------|-------------|
| Requirement | All (`«satisfy»`, `«verify»`, `«realize»`, `«deriveReqt»`) | RTM-like overview. |
| Use Case | `«satisfy»` (UseCase→Requirement), `«include»`/`«extend»` | UC coverage of requirements. |
| Activity | `«realize»` (Activity→UseCase) | UC realization. |
| Sequence | `«realize»` (Interaction→Activity, Class/Operation→Interaction) | Behavioral realization. |
| Class | `«realize»` (Class→Interaction, optional) | Implementation realization. |

`[REQ-CHAIN-007]` Trace relations MAY be shown on any diagram as a dashed dependency arrow
with the stereotype label in `«guillemets»`. Their visual style follows
[research/04-uml-visual-notation.md](../research/04-uml-visual-notation.md) (dependency
notation).

## Navigating the chain in the UI

`[REQ-CHAIN-008]` Selecting any element, the inspector MUST offer:
- "Trace forward" → opens the next-level diagram(s) this element participates in.
- "Trace backward" → opens the previous-level diagram(s).
- "Show full chain" → opens the Requirement diagram filtered to this element's chain.

`[REQ-CHAIN-009]` The MCP `trace.forward` / `trace.backward` tools MUST return the same
information as structured data (element lists per level).

## What breaks completeness (defects)

| Defect | Effect on chain | Severity |
|--------|-----------------|----------|
| Orphan requirement | Chain has no root for this requirement. | HIGH |
| Orphan element (no trace in or out) | Element is disconnected from the chain. | MEDIUM |
| Circular trace | Chain has a cycle; traversal non-terminates. | HIGH |
| Missing link (e.g. UseCase with no Activity) | Chain incomplete at that level. | MEDIUM |
| Orphan test | TestCase verifies nothing. | HIGH |
| Uncovered element on path | Element exists but no diagram shows it. | LOW |

All are defined normatively in [analytics/03-orphan-and-coverage-analysis.md](../analytics/03-orphan-and-coverage-analysis.md).

## Open questions (resolve before STABLE)

- [ ] Do we allow skipping levels (Requirement → Class directly)? Current spec: yes; treated
      as a complete path of length 1.
- [ ] Weighted completeness (a Requirement with 1 of 5 UseCases traced is "20% complete")?
      Tentative: yes in v1 as an optional column in RTM.
