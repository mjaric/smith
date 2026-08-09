---
status: DRAFT
---

# UML Model 06 — Diagram Interchange (Diagram-as-View)

This is the most important architectural decision in Smith, stated once here and referenced
everywhere: **a diagram is a view over the model, never a container of model elements.**

OMG separates the UML model from the Diagram Interchange (DI) layer for exactly this reason
(see [research/01-omg-uml-specification.md](../research/01-omg-uml-specification.md), §DI).
Smith honors the separation strictly.

## The principle, restated

`[REQ-DI-001]` A diagram owns NO model elements. A diagram owns only **view references**
(DI shapes/edges) that point to model elements by `ElementId`.

`[REQ-DI-002]` Deleting a diagram MUST NOT delete any model element. It deletes only the
view references and their layout.

`[REQ-DI-003]` Reparenting an element (moving it to another package) MUST NOT change its
appearance on any diagram. Only its `qualifiedName` (derived) changes.

`[REQ-DI-004]` Renaming an element MUST update every diagram that references it, because the
name is read live from the model at render time. The diagram stores no name.

## Diagram element (the view container)

### Diagram (extends NamedElement, owned by a Package)
OMG UML DI.

| Field | Type | Meaning |
|-------|------|---------|
| `id` | `ElementId` | Diagram identity. |
| `name` | `String` | Diagram name (e.g. "Login sequence"). Mutable. |
| `kind` | `DiagramKind` | One of the 14 UML kinds + `requirement`. |
| `owner` | `ElementId` | The package owning this diagram. |
| `viewReferences` | `Vec<ViewRefId>` | Shapes and edges shown on this diagram. |
| `canvasState` | `CanvasState` | Last viewport (pan/zoom) for restore. |

`[REQ-DI-005]` A `Diagram` is itself an element in the ownership tree (owned by a Package).
It appears in the model explorer under its owning package.

`[REQ-DI-006]` A diagram has exactly one `kind`. The kind constrains which element kinds
may be referenced (see per-kind specs in 02-structural, 03-behavioral). Adding a view
reference to an element kind not allowed on this diagram kind MUST be rejected with
error `-32001`.

### DiagramKind enumeration
`class` | `object` | `component` | `compositeStructure` | `package` | `deployment` |
`profile` | `useCase` | `activity` | `sequence` | `stateMachine` | `communication` |
`timing` | `interactionOverview` | `requirement`.

`requirement` is a Smith-specific kind for the Requirements diagram (tabular/box layout
of Requirements and their traceability).

## View references

### ShapeView (a view reference to an element)
The DI "shape": how an element appears on one diagram.

| Field | Type | Meaning |
|-------|------|---------|
| `id` | `ViewRefId` | View-reference identity (per-diagram). |
| `diagramId` | `ElementId` | The diagram this view lives on. |
| `modelElementId` | `ElementId` | The model element this view shows. |
| `bounds` | `Bounds` | `{ x, y, w, h }` in canvas coordinates (device-independent units). |
| `presentationHints` | `PresentationHints` | Folded compartments, color override, icon style, etc. |
| `containerShapeId` | `Option<ViewRefId>` | Parent shape (e.g. a class inside a package shape, a lifeline in an interaction). |

`[REQ-DI-007]` `modelElementId` references the model element. The shape stores NO model
data — only geometry and presentation.

`[REQ-DI-008]` The same model element MAY appear as multiple shapes on the same diagram
(legitimate) or on many diagrams (encouraged). Each shape is an independent view reference
with its own bounds.

### Use Case Subject rectangle

The Use Case diagram's **Subject** rectangle is a view-only grouping shape: a `ShapeView`
whose `modelElementId` references the subject `Package` (or a dedicated `Subject` element).
It visually groups use cases but **never owns them** — ownership lives in the model tree (per
REQ-BEHAV-002 in [uml-model/03-behavioral-diagrams.md](./03-behavioral-diagrams.md) and
design-principle P2). A `ShapeView` requires a `modelElementId`; the Subject shape points at
the subject element, not at the use cases it visually contains.

### EdgeView (a view reference to a relationship)
The DI "edge": how a relationship appears on one diagram.

| Field | Type | Meaning |
|-------|------|---------|
| `id` | `ViewRefId` | View-reference identity. |
| `diagramId` | `ElementId` | The diagram. |
| `modelElementId` | `ElementId` | The relationship this view shows (Association, Message, Dependency, …). |
| `sourceShapeId` | `ViewRefId` | The shape the edge starts from. |
| `targetShapeId` | `ViewRefId` | The shape the edge ends at. |
| `bendPoints` | `Vec<Point>` | Routing bend points (canvas coordinates). |
| `labelPositions` | `Map<LabelKey, Point>` | Positions for name/multiplicity/guard labels. |
| `routingKind` | `RoutingKind` | `orthogonal` \| `straight` \| `curved` \| `fixedBendPoints`. |

`[REQ-DI-009]` An edge view references BOTH a relationship (model) and two shapes (view).
If the source or target shape is deleted, the edge view is also deleted (not the model
relationship).

`[REQ-DI-010]` A model relationship MAY be shown on multiple diagrams via multiple edge
views. Each has its own bend points.

## CanvasState (viewport)
| Field | Type | Meaning |
|-------|------|---------|
| `panX`, `panY` | `f64` | Canvas pan offset. |
| `zoom` | `f64` | 1.0 = 100%. Range: `0.1` to `4.0` enforced. |

`[REQ-DI-011]` Canvas state is per-diagram and persisted, so reopening a diagram restores
the user's viewport.

## PresentationHints
Per-shape rendering hints, all optional with defaults.

| Field | Type | Meaning |
|-------|------|---------|
| `compartmentFolding` | `Map<CompartmentKind, bool>` | Which compartments are collapsed. |
| `colorOverride` | `Option<Color>` | Per-shape color override (else from design system by element kind). |
| `iconStyle` | `IconStyle` | `default` \| `minimal` \| `none`. |
| `stereotypeDisplay` | `StereotypeDisplay` | `show` \| `hide` \| `icon`. |
| `shadowStyle` | `ShadowStyle` | `none` \| `subtle` \| `strong`. Default `subtle`. |

`[REQ-DI-012]` Presentation hints are the ONLY place color/style live on a shape. They are
overrides; the default comes from the design system (see
[ui-ux/01-design-system.md](../ui-ux/01-design-system.md)) keyed by element kind and
stereotype.

## Units & coordinate system

`[REQ-DI-013]` Canvas coordinates are in **device-independent units** (1 unit = 1 logical
pixel at 100% zoom). All bounds, bend points, and label positions use this system.

`[REQ-DI-014]` The origin `(0,0)` is the canvas origin; the canvas is infinite in all four
directions (negative coordinates allowed). The model API does not constrain bounds.

## Coverage (uncovered-element detection)

The user's explicit ask: know which elements are not on any diagram.

`[REQ-DI-015]` Smith MUST maintain a derived **coverage index**: for every model element,
the set of diagrams that reference it (via any shape or edge).

`[REQ-DI-016]` An element with an empty coverage set is an **uncovered element** (quality
issue, severity LOW by default — see
[analytics/03-orphan-and-coverage-analysis.md](../analytics/03-orphan-and-coverage-analysis.md)).
The issue panel lists uncovered elements, grouped by owning package.

`[REQ-DI-017]` The coverage index is derived from view references and updated incrementally
on every view-reference add/remove. It is never stored as primary data.

## View-reference integrity

`[REQ-DI-018]` Deleting a model element MUST cascade-delete all view references to it
(orphan shapes/edges are not allowed). The deletion is undoable as a single command.

`[REQ-DI-019]` Reparenting a relationship (changing its `owner`) MUST NOT affect its edge
views (they reference by `ElementId`, which is stable).

## What "diagram owns" summary

| Owned by diagram (view) | Owned by model (truth) |
|-------------------------|------------------------|
| View references (shapes, edges) | Elements (classes, requirements, …) |
| Bounds, bend points, label positions | Names, attributes, operations, stereotypes |
| Compartment folding, color override | Relationship type, multiplicity, direction |
| Canvas viewport state | Trace relations, comments |

## Open questions (resolve before STABLE)

- [ ] Do we support **diagram links** (a shape on one diagram that, when clicked, opens
      another diagram)? Useful for navigation. Tentative: yes, via a special shape kind
      `DiagramLinkShape`.
- [ ] Do we support **diagram frames** (the UML diagram frame border with a heading like
      `sd Login`)? Tentative: yes, optional, on by default for sequence diagrams.
- [ ] Should bend points be **absolute** canvas coords or **relative** to source/target?
      Current spec: absolute (simpler, stable under element move). Reconsider if move
      performance suffers.
