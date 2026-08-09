---
status: DRAFT
---

# UML Model 04 — Requirements & Test Cases (Smith extensions)

UML does not have first-class requirements or test cases. Smith extends the metamodel with
**SysML-inspired** `Requirement` and `TestCase` element kinds and ships them in the built-in
`smith::requirements` profile. They live in the ownership tree like any element, participate
in diagrams, and — critically — anchor the traceability chain
([uml-model/05-traceability-chain.md](./05-traceability-chain.md)).

SysML background: see [research/05-traceability-and-analytics.md](../research/05-traceability-and-analytics.md).

## Requirement (extends NamedElement)

A `Requirement` is a named, owned element describing a system requirement. SysML §16.

| Field | Type | Meaning |
|-------|------|---------|
| `id` | `ElementId` | Model identity (immutable). |
| `requirementId` | `String` | **Human requirement identifier**, e.g. `REQ-001`. Unique within the project. Mutable. This is what RTM and tests reference. |
| `text` | `String` | The requirement statement (Markdown allowed). Non-empty. |
| `category` | `RequirementCategory` | `functional` \| `performance` \| `interface` \| `design` \| `constraint` \| `safety` \| `security` \| `other`. |
| `priority` | `Priority` | `must` \| `should` \| `could` \| `wont` (MoSCoW). |
| `status` | `RequirementStatus` | `draft` \| `proposed` \| `approved` \| `implemented` \| `verified` \| `rejected` \| `obsolete`. |
| `verificationMethods` | `Set<VerificationMethod>` | How this requirement will be verified: `analysis` \| `demonstration` \| `inspection` \| `test` (SysML §16). |
| `source` | `Option<String>` | Originating stakeholder or document. |
| `rationale` | `Option<String>` | Why this requirement exists. |
| `customProperties` | `Map<String, Value>` | User-defined typed properties (key/value). |

`[REQ-REQ-001]` Every Requirement MUST have a non-empty `requirementId` unique within the
project. Creating a Requirement with a duplicate `requirementId` MUST be rejected with
error `-32002`.

`[REQ-REQ-002]` `text` MUST be non-empty. An empty-text requirement is invalid.

`[REQ-REQ-003]` `requirementId` is mutable; renaming updates all `«deriveReqt»` and RTM
references. The model `id` is unaffected.

### Requirement visual notation
A Requirement renders as a UML Class-like box with a `«requirement»` stereotype header, the
`requirementId` as the name, and compartments:

```
┌────────────────────────────┐
│      «requirement»         │
│        REQ-001             │
├────────────────────────────┤
│ Text: <requirement text>   │
├────────────────────────────┤
│ Id: REQ-001                │
│ Category: functional       │
│ Priority: must             │
│ Status: approved           │
│ Verify: test, demonstration│
└────────────────────────────┘
```

`[REQ-REQ-004]` Requirements MAY appear on **Use Case diagrams** (as the target of
`«satisfy»`), on dedicated **Requirement diagrams** (a tabular/box layout), and as
annotations on any diagram. They MUST NOT appear as classes on class diagrams (different
semantics); if a user wants to show a requirement on a class diagram, use a `Comment` or a
`«trace»` link shown as a note.

## TestCase (extends Behavior)

A `TestCase` is a behavioral element describing a test. SysML §16.7, §17.3. Smith models
two forms: a **textual** test case and a **behavioral** test case (backed by an Activity or
Interaction).

| Field | Type | Meaning |
|-------|------|---------|
| `id` | `ElementId` | Model identity. |
| `testCaseId` | `String` | Human test identifier, e.g. `TC-001`. Unique within project. |
| `title` | `String` | Short human title. |
| `description` | `Option<String>` | What the test covers. |
| `preconditions` | `Vec<String>` | Preconditions (bullet list). |
| `postconditions` | `Vec<String>` | Expected post-state. |
| `steps` | `Vec<TestStep>` | Ordered steps (for textual tests). |
| `expectedResult` | `String` | Pass criterion. |
| `specification` | `Option<ElementId>` | Optional Activity or Interaction defining the test behavior (behavioral test). |
| `status` | `TestStatus` | `draft` \| `ready` \| `passing` \| `failing` \| `blocked` \| `deprecated`. |

### TestStep
| Field | Type | Meaning |
|-------|------|---------|
| `action` | `String` | What the tester/automation does. |
| `data` | `Option<String>` | Input data. |
| `expected` | `String` | Expected result of this step. |

`[REQ-TC-001]` Every TestCase MUST have a non-empty `testCaseId` unique within the project.

`[REQ-TC-002]` A TestCase with no `steps` AND no `specification` is a draft
(`TestCase.status = 'draft'`); it is NOT a defect. Defect D2 (orphan test) applies only when
the TestCase has zero outgoing `«verify»`, per
[analytics/03](../analytics/03-orphan-and-coverage-analysis.md).

`[REQ-TC-003]` A TestCase MUST have ≥1 outgoing `«verify»` relation to a Requirement.
A TestCase verifying nothing is an orphan test (severity HIGH).

### TestCase visual notation
Renders as a box with `«testCase»` header:

```
┌────────────────────────────┐
│       «testCase»           │
│         TC-001             │
├────────────────────────────┤
│ Title: <title>             │
├────────────────────────────┤
│ Steps:                     │
│  1. <action> → <expected>  │
│  2. ...                    │
├────────────────────────────┤
│ Expected: <result>         │
│ Verifies: REQ-001, REQ-002 │
└────────────────────────────┘
```

TestCases MAY appear on **Requirement diagrams** and as annotations on Use Case / Activity
diagrams (showing what verifies them). They do not appear on class diagrams.

## Requirement package convention

`[REQ-REQ-005]` Smith RECOMMENDS (does not mandate) a top-level `Requirements` package with
sub-packages by category or feature. The UI offers a "New requirement package" command that
creates this structure. Requirements may live anywhere in the ownership tree; the
recommendation is for navigability only.

## Built-in profile: `smith::requirements`

The following stereotypes ship built-in and cannot be removed:

| Stereotype | Applies to | Tags |
|------------|-----------|------|
| (none — `Requirement` is a metaclass) | — | — |

Requirements and TestCases are **metaclasses** (new element kinds), not stereotypes on
Class. This is a deliberate choice: stereotypes are for classifying existing UML elements,
while Requirements/TestCases have distinct fields (requirementId, steps, verification
methods) that don't fit on a Class.

`[REQ-REQ-006]` The model API and MCP tools MUST treat `Requirement` and `TestCase` as
first-class element kinds with dedicated create/read/update operations, not as generic
classes with a stereotype.

## Built-in profile: `smith::tests`

The following stereotypes ship built-in and cannot be removed:

| Stereotype | Applies to | Tags |
|------------|-----------|------|
| (none — `TestCase` is a metaclass) | — | — |

`TestCase` is a **metaclass** (a new element kind extending `Behavior`), not a stereotype on
Class, with distinct fields (`testCaseId`, `status`, `steps`, `specification`) that do not fit
on a Class. This mirrors the `smith::requirements` profile decision (see REQ-REQ-006).

## Relationship to traceability

Requirements and TestCases are the anchors of the traceability chain:

- `Requirement` — target of `«satisfy»`, `«verify»`, `«realize»`; source of `«deriveReqt»`.
- `TestCase` — source of `«verify»`.

See [architecture/04-traceability-relations.md](../architecture/04-traceability-relations.md)
and [uml-model/05-traceability-chain.md](./05-traceability-chain.md).

## Requirement diagram (kind = `requirement`)

The Requirement diagram is the 15th `DiagramKind` (see
[uml-model/06-diagram-interchange.md](./06-diagram-interchange.md) §DiagramKind enumeration).
It is a Smith-specific kind for visualizing Requirements, TestCases, and their traceability.

`[REQ-REQ-007]` Allowed shapes:
- `Requirement` — rendered as the `«requirement»` box (see §Requirement visual notation).
- `TestCase` — rendered as the `«testCase»` box (see §TestCase visual notation).
- `Comment` — rendered as a note.

`[REQ-REQ-008]` Allowed edges — all trace relations (see
[architecture/04-traceability-relations.md](../architecture/04-traceability-relations.md)):
`«satisfy»`, `«verify»`, `«realize»`, `«deriveReqt»`, `«refine»`, `«trace»`, `«copy»`.
UseCases shown alongside Requirements MAY also carry `«include»` / `«extend»` edges.

`[REQ-REQ-009]` Layout conventions:
- **Tabular layout** (default): one row per Requirement, with columns for satisfying
  artifacts and verifying tests — an on-canvas RTM.
- **Box layout**: Requirement and TestCase boxes connected by trace edges in a free-form
  arrangement.

This diagram makes REQ-REQ-004 concrete: the "dedicated Requirement diagrams" referenced
there are instances of this `DiagramKind`.

## Enumeration value sets

`RequirementCategory`, `Priority`, `RequirementStatus`, `VerificationMethod`, `TestStatus`
are closed enumerations. Their values are fixed for v1; user extension is via the
`customProperties` map, not new enum values.

`[REQ-ENUM-001]` Persisting an element with an out-of-range enum value MUST be rejected at
the model API boundary (never silently coerced).

## Open questions (resolve before STABLE)

- [ ] Do we need requirement **versioning** (trace history of text changes)? Current spec:
      no; `status` + `rationale` suffice for v1. Versioning is a v2 candidate.
- [ ] Should `Requirement` support nested sub-requirements via ownership (not just
      `«deriveReqt»`)? Current spec: no — deriveReqt is the only parent-child relation.
      Ownership is purely structural (packages).
