---
status: DRAFT
---

# UML Model 02 — Structural Diagrams

The structural UML diagram kinds Smith supports, the element types each renders, and the
constraints on what may appear. Notation (shapes/lines/arrowheads) is fixed by UML — see
[research/04-uml-visual-notation.md](../research/04-uml-visual-notation.md). This document
specifies the **metamodel contract** for each diagram kind (which elements, which
relationships, what's prohibited).

Core metamodel: [uml-model/01-metamodel-core.md](./01-metamodel-core.md). Diagram-as-view:
[uml-model/06-diagram-interchange.md](./06-diagram-interchange.md).

## Principle: a diagram kind constrains view references

`[REQ-STRUC-001]` A diagram's `kind` determines which element kinds may appear on it as
shapes and which relationship kinds may appear as edges. Adding a view reference to a
disallowed kind is rejected (`-32001`) — see
[uml-model/06-diagram-interchange.md](./06-diagram-interchange.md) REQ-DI-006.

`[REQ-STRUC-002]` Element kinds NOT listed for a diagram kind are prohibited on it, even
though they exist in the model. (They may appear on other diagram kinds.)

## 1. Class diagram (`kind = "class"`)

OMG UML 2.5.1 §11. Class diagrams show the static structure: classifiers and their
relationships.

**Allowed element kinds (shapes):**
- `Class`, `Interface`, `DataType`, `Enumeration`, `PrimitiveType`
- `AssociationClass` (class attached to an association)
- `Signal` (for receptions)
- `Package` (as a nested container / package-shape)
- `Comment`, `Constraint`

**Allowed relationship kinds (edges):**
- `Association` (with `aggregation`/`composition` variants)
- `Generalization`
- `Realization` (interface realization)
- `Dependency` (incl. stereotyped: `«use»`, `«call»`, `«import»`)
- Trace relations `«realize»`, `«trace»`, `«refine»` (class → interaction, optional)

**Prohibited:** UseCases, Actors, Activities, Interactions, Requirements, TestCases do NOT
appear as shapes (different semantics). Trace relations to them appear only as dashed
dependencies with stereotype labels.

`[REQ-STRUC-003]` A Class shape renders the class's compartments per
[research/04](../research/04-uml-visual-notation.md) §1: name (italic if abstract), attributes,
operations, receptions (optional). Compartment folding is a view concern
([uml-model/06](./06-diagram-interchange.md) PresentationHints).

`[REQ-STRUC-004]` Associations on a class diagram show role names, multiplicities, and
navigability arrows per the element's association definition (model data), positioned as view
labels.

## 2. Object diagram (`kind = "object"`)

OMG UML 2.5.1 §9.8. Instances and links — snapshots of class diagrams.

**Allowed shapes:** `InstanceSpecification` (with slots), `Comment`.
**Allowed edges:** `Link` (instance of an association), `Dependency`.

`[REQ-STRUC-005]` Instance names render **underlined** (`instanceName : TypeName`); slots
show `attribute = value`. This is the UML convention — see
[research/04](../research/04-uml-visual-notation.md) §11.

## 3. Component diagram (`kind = "component"`)

OMG UML 2.5.1 §11.6. Components, provided/required interfaces (ball-and-socket), ports.

**Allowed shapes:** `Component`, `Interface` (as lollipop/socket or rectangle), `Port`,
`Comment`.
**Allowed edges:** `InterfaceRealization`, `Usage` (dependency on interface), `ComponentRealization`,
`Dependency`, `Assembly`/`Delegation` connectors.

`[REQ-STRUC-006]` Components render with the `«component»` keyword and optional component
icon (two-tab rectangle). Provided interfaces = lollipops; required = sockets — per
[research/04](../research/04-uml-visual-notation.md) §7.

## 4. Composite Structure diagram (`kind = "compositeStructure"`)

OMG UML 2.5.1 §11.2. Internal structure of structured classifiers: parts, connectors, ports.

**Allowed shapes:** `StructuredClassifier` (frame), `Part` (role, owned/referenced), `Port`,
`Collaboration`, `Comment`.
**Allowed edges:** `Connector` (between parts/ports), `Dependency`.

`[REQ-STRUC-007]` Parts render as boxes inside the classifier's internal-structure
compartment; owned parts have solid outlines, referenced parts dashed — per
[research/04](../research/04-uml-visual-notation.md) §8.

## 5. Package diagram (`kind = "package"`)

OMG UML 2.5.1 §12.2. Package ownership and dependencies between packages.

**Allowed shapes:** `Package` (tabbed-folder), `Profile` (stereotyped package), `Model`
(stereotyped package), `Comment`.
**Allowed edges:** `PackageImport` (`«import»`), `PackageMerge` (`«merge»`),
`ElementImport` (`«access»`), `Dependency`.

`[REQ-STRUC-008]` A package diagram MAY show elements inside packages (nested shapes), but
the ownership is the model's ownership tree (P1, P2). The diagram is a view; moving a class
between packages via this diagram is a model reparent operation.

## 6. Deployment diagram (`kind = "deployment"`)

OMG UML 2.5.1 §19. Physical deployment: nodes, artifacts, communication paths.

**Allowed shapes:** `Node` (`«device»` / `«executionEnvironment»`, 3D box), `Artifact`
(`«artifact»`), `Component` (deployed), `Comment`.
**Allowed edges:** `Deployment` (`«deploy»`), `Manifestation` (`«manifest»`),
`CommunicationPath` (association between nodes), `Dependency`.

`[REQ-STRUC-009]` Nodes render as 3D perspective boxes (per
[research/04](../research/04-uml-visual-notation.md) §9); artifacts as folded-corner
rectangles.

## 7. Profile diagram (`kind = "profile"`)

OMG UML 2.5.1 §12.3, §18. Defines stereotypes and their metaclass extensions.

**Allowed shapes:** `Profile`, `Stereotype`, `Metaclass` (reference), `Comment`.
**Allowed edges:** `Extension` (stereotype → metaclass, filled arrowhead),
`Generalization` (stereotype → stereotype), `Dependency` (`«reference»`, `«apply»`).

`[REQ-STRUC-010]` Profile diagrams are the only place Smith users DEFINE stereotypes. On
other diagram kinds, stereotypes are merely applied (via the inspector / `model.applyStereotype`).

## Common constraints (all structural diagrams)

`[REQ-STRUC-011]` All element kinds must already exist in the model (be owned by a package)
before appearing on a diagram. The "create element on diagram" workflow creates the element
in the model first (in a chosen/default package), then adds the view reference — one combined
command, undoable.

`[REQ-STRUC-012]` Deleting a shape from a structural diagram removes ONLY the view reference
(NOT the model element). Confirm dialogs make this distinction clear ("Hide on this diagram"
vs "Delete from model").

`[REQ-STRUC-013]` A shape's bounds, bend points, and label positions are view data
([uml-model/06](./06-diagram-interchange.md)); the element's name/attributes/operations are
model data, read live at render.

## Traceability participation

`[REQ-STRUC-014]` Structural diagrams participate in the traceability chain at the
**Class/Operation level** (the terminal end of the canonical chain — see
[uml-model/05-traceability-chain.md](./05-traceability-chain.md)). Classes may carry incoming
`«realize»` from Interactions; this is shown as a dashed dependency arrow with the stereotype
label when the user chooses to display traces on the diagram (off by default — controlled by a
diagram view setting "show traceability edges").

## Open questions (resolve before STABLE)

- [ ] Should class diagrams support nested-package rendering (show packages as containers
      with classes inside)? Tentative: yes (an optional layout mode); the default is
      flat-with-qualified-names.
- [ ] AssociationClass support in v1? Tentative: yes (common in domain models).
