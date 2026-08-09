---
status: DRAFT
---

# UML Model 01 — Metamodel Core

The core of Smith's UML metamodel. This document defines the abstractions every other UML
spec depends on. It mirrors OMG UML 2.5.1 §7 (Common Structure), §12 (Packages/Profiles),
and §9 (Classification). See [research/01-omg-uml-specification.md](../research/01-omg-uml-specification.md) for the spec map.

Every element type Smith implements is a specialization of `NamedElement`. Ownership,
names, visibility, stereotypes, comments, and the relationship base types live here.

## Root abstractions

### Element
The top of the metaclass hierarchy. Every node in the model graph IS-A `Element`.
OMG UML §7.5.

| Field | Type | Meaning |
|-------|------|---------|
| `id` | `ElementId` (opaque, immutable) | Globally unique within the project. Assigned at creation. |
| `owner` | `Option<ElementId>` | The namespace that owns this element. `None` only for the project root. |
| `comments` | `Vec<Comment>` (owned, not referenced) | Comments attached to this element. |
| `stereotypes` | `Vec<AppliedStereotype>` | Stereotype applications. |

`[REQ-MM-001]` Every element MUST have a non-empty `id` that is unique within the project
and immutable for the element's lifetime. IDs are never reused after element deletion.

`[REQ-MM-002]` `owner` forms a tree. Every element (except the project root) has exactly one
owner. The graph of `owner` edges MUST be acyclic and connected (single tree rooted at the
project).

### NamedElement (extends Element)
OMG UML §7.8. Adds naming.

| Field | Type | Meaning |
|-------|------|---------|
| `name` | `Option<String>` | Human-readable, mutable, NOT unique within namespace. |
| `visibility` | `Visibility` | `public` \| `private` \| `protected` \| `package`. Default `public`. |
| `qualifiedName` | derived `String` | `/`-delimited path from root. Derived from `owner` chain + `name`. |

`[REQ-MM-003]` `qualifiedName` MUST be derived on demand from the ownership chain. It MUST
NOT be stored. Renaming or reparenting updates it transparently.

`[REQ-MM-004]` `name` MAY be empty. Two siblings MAY share a name (names are not unique
within a namespace; IDs are the identity).

### Namespace (extends NamedElement)
OMG UML §7.19. An element that owns named members.

| Field | Type | Meaning |
|-------|------|---------|
| `ownedMembers` | derived `Vec<ElementId>` | Elements owned by this namespace. Derived inverse of `owner`. |

`[REQ-MM-005]` Adding an element to a namespace sets its `owner`; removing it unsets the
owner (orphaning it — see [analytics/03](../analytics/03-orphan-and-coverage-analysis.md)).

### Package (extends Namespace, PackageableElement)
OMG UML §12.2. The primary ownership container.

| Field | Type | Meaning |
|-------|------|---------|
| `packagedElements` | `Vec<ElementId>` | Members (packages, classes, diagrams, requirements, etc.). |
| `profileApplications` | `Vec<ProfileApplication>` | Profiles applied to this package (stereotypes available). |
| `isModel` | `bool` | True for the root package (the project). Exactly one per project. |

`[REQ-MM-006]` The project root MUST be a `Package` with `isModel = true`. It owns all
top-level packages and elements.

`[REQ-MM-007]` Packages MAY nest arbitrarily. The ownership tree has no depth limit in the
metamodel; the UI may constrain display.

## Stereotypes & tagged values
OMG UML §12.3 (Profiles), §12.4 (Stereotype/Extension classifier descriptions), §22 (Standard Profile). Smith supports user-defined profiles plus the built-in
`smith::traceability` and `smith::requirements` profiles.

### Profile (extends Package)
Defines stereotypes and their extensions.

| Field | Type | Meaning |
|-------|------|---------|
| `stereotypes` | `Vec<Stereotype>` | Stereotypes defined here. |
| `metamodelExtensions` | `Vec<Extension>` | Which UML metaclasses each stereotype extends. |

### Stereotype (extends Class)
A stereotype is itself a classifier (it can have attributes = tag definitions).

| Field | Type | Meaning |
|-------|------|---------|
| `name` | `String` | Stereotype name (shown in `«guillemets»`). |
| `extendedMetaclasses` | `Vec<MetaclassKind>` | Which element kinds this may apply to. |
| `tagDefinitions` | `Vec<TagDefinition>` | Named typed tags this stereotype adds. |

### AppliedStereotype
An application of a stereotype to a specific element.

| Field | Type | Meaning |
|-------|------|---------|
| `stereotypeId` | `ElementId` | The stereotype being applied. |
| `tagValues` | `Map<TagName, Value>` | Values for the stereotype's tag definitions. |

`[REQ-MM-008]` Applying a stereotype to an element whose metaclass is not in the
stereotype's `extendedMetaclasses` MUST be rejected with error `-32001`.

`[REQ-MM-009]` An element MAY carry multiple stereotypes, from multiple profiles. Tag value
namespaces are per-stereotype (no collision).

`[REQ-MM-010]` The built-in profiles (`smith::traceability`, `smith::requirements`,
`smith::tests`) are present in every project, cannot be deleted, and their stereotypes are
available without explicit `ProfileApplication`. User-defined profiles require explicit
application.

## Comments
OMG UML §7.6.

### Comment (owned by an Element)
| Field | Type | Meaning |
|-------|------|---------|
| `body` | `String` | The comment text (Markdown allowed). |
| `annotatedElements` | `Vec<ElementId>` | Elements this comment annotates (≥1; the owner is implicit). |

`[REQ-MM-011]` A comment MUST have a non-empty `body`. Deleting the element that owns a
comment deletes the comment.

## Relationships — base types
OMG UML §7.9 (Relationship), §19.4 (Dependency), §19.12 (DirectedRelationship).

### Relationship
Base of every edge.

| Field | Type | Meaning |
|-------|------|---------|
| `id` | `ElementId` | Edge identity. |
| `owner` | `ElementId` | Owning namespace. |

### DirectedRelationship (extends Relationship)
| Field | Type | Meaning |
|-------|------|---------|
| `sources` | `Vec<ElementId>` | The "from" elements. |
| `targets` | `Vec<ElementId>` | The "to" elements. |

`[REQ-MM-012]` Most Smith relationships are binary (one source, one target). N-ary is
allowed by the metamodel but the UI shows binary only.

### Dependency (extends DirectedRelationship)
OMG UML §19.4. The base for traceability relations. Trace stereotypes apply to Dependency.

| Field | Type | Meaning |
|-------|------|---------|
| `clients` | derived `Vec<ElementId>` | = `sources`. |
| `suppliers` | derived `Vec<ElementId>` | = `targets`. |

### Relationship subtypes Smith implements
Defined in their own docs:

- **Association** (Class diagram), **Generalization**, **Realization**, **Usage** —
  [uml-model/02-structural-diagrams.md](./02-structural-diagrams.md).
- **Message**, **Lifeline interaction** — [uml-model/03-behavioral-diagrams.md](./03-behavioral-diagrams.md).
- **Trace relations** (`«satisfy»`, `«verify»`, `«realize»`, `«deriveReqt»`, `«trace»`,
  `«refine»`, `«copy»`) — Dependencies with stereotypes, see
  [architecture/04-traceability-relations.md](../architecture/04-traceability-relations.md).

## Visibility
`[REQ-MM-013]` `Visibility` is one of `public` (+), `private` (-), `protected` (#),
`package` (~). The notation prefix is fixed by UML; Smith does not invent new visibilities.

## Multiplicity
OMG UML §7.5. Used on association ends and attributes.

| Field | Type | Meaning |
|-------|------|---------|
| `lower` | `u32` | Inclusive lower bound. |
| `upper` | `Option<u32>` | Inclusive upper bound; `None` = unbounded (`*`). |

`[REQ-MM-014]` Notation: `lower..upper`, abbreviated: `1` = `1..1`, `*` = `0..*`, `0..1`,
`1..5`. `upper < lower` is invalid and MUST be rejected.

## Metaclass kind registry
Smith pre-defines the set of metaclasses (element kinds). Each kind has a stable string
identifier used by stereotypes' `extendedMetaclasses`, MCP tool signatures, and search.

| Kind | Extends | Defined in |
|------|---------|------------|
| `Package` | Namespace | this doc |
| `Class` | Classifier | 02-structural |
| `Interface` | Classifier | 02-structural |
| `DataType` | Classifier | 02-structural |
| `Enumeration` | DataType | 02-structural |
| `Association` | Relationship | 02-structural |
| `Generalization` | DirectedRelationship | 02-structural |
| `Realization` | DirectedRelationship | 02-structural |
| `Dependency` | DirectedRelationship | this doc |
| `UseCase` | BehavioredClassifier | 03-behavioral |
| `Actor` | Classifier | 03-behavioral |
| `Activity` | Behavior | 03-behavioral |
| `Interaction` | Behavior (Sequence) | 03-behavioral |
| `StateMachine` | Behavior | 03-behavioral |
| `Component` | Classifier | 02-structural |
| `Node` | Classifier (Deployment) | 02-structural |
| `Artifact` | Classifier | 02-structural |
| `Requirement` | NamedElement (Smith ext.) | 04-requirements-and-tests |
| `TestCase` | Behavior (Smith ext.) | 04-requirements-and-tests |
| `Profile`, `Stereotype` | — | this doc |
| `Comment` | Element | this doc |

### Sub-element & relationship kinds

The kinds above are the **top-level element kinds** (ownable in the ownership tree). The
following sub-element and relationship kinds are defined across Smith's metamodel; they are
not independently ownable (they belong to a parent element or diagram) but each has a stable
identifier used by view references, search, and constraint checks.

| Kind | Defined in |
|------|------------|
| `PrimitiveType` | 02-structural |
| `AssociationClass` | 02-structural |
| `Signal` | 03-behavioral |
| `InstanceSpecification` | this doc |
| `Slot` | this doc |
| `Link` | 02-structural |
| `Port` | 02-structural (composite structures) |
| `Connector` | 02-structural |
| `Part` | 02-structural (composite structures) |
| `Collaboration` | 03-behavioral |
| `CollaborationUse` | 03-behavioral |
| `Message` | 03-behavioral |
| `Lifeline` | 03-behavioral |
| `ExecutionSpecification` | 03-behavioral |
| `CombinedFragment` | 03-behavioral |
| `InteractionUse` | 03-behavioral |
| `Gate` | 03-behavioral |
| `State` | 03-behavioral |
| `Region` | 03-behavioral |
| `Transition` | 03-behavioral |
| `Pseudostate` (`Initial`/`Choice`/`Junction`/`Fork`/`Join`/`DeepHistory`/`ShallowHistory`/`Terminate`/`EntryPoint`/`ExitPoint`) | 03-behavioral |
| `FinalState` | 03-behavioral |
| `Action` | 03-behavioral |
| `ControlNode` (`Initial`/`ActivityFinal`/`FlowFinal`/`Decision`/`Merge`/`Fork`/`Join`) | 03-behavioral |
| `ObjectNode` | 03-behavioral |
| `ActivityPartition` | 03-behavioral |
| `Include` | 03-behavioral |
| `Extend` | 03-behavioral |
| `ExtensionPoint` | 03-behavioral |
| `Deployment` | 02-structural |
| `Manifestation` | 02-structural |
| `CommunicationPath` | 02-structural |
| `PackageImport` | this doc |
| `PackageMerge` | this doc |
| `ElementImport` | this doc |
| `Metaclass` | this doc |
| `Extension` | this doc |

REQ-MM-015 closure applies to top-level element kinds (the first table). Sub-element kinds
(features, nodes, pseudostates) have their own closed registry in this second table. Stereotype
`extendedMetaclasses` references top-level kinds.

`[REQ-MM-015]` The set of kinds is closed for v1. User extension is via stereotypes, not new
metaclasses.

## Invariants (always checked)

- `[INV-MM-001]` Ownership tree is acyclic and connected (single root).
- `[INV-MM-002]` No two elements share an `id`.
- `[INV-MM-003]` Every stereotype application targets a metaclass in the stereotype's
  `extendedMetaclasses`.
- `[INV-MM-004]` Every relationship's `sources` and `targets` reference existing elements.
- `[INV-MM-005]` `qualifiedName` derivation terminates (no ownership cycle).

## Open questions (resolve before STABLE)

- [ ] Should `Comment` be a first-class element (ownable, referenceable) or strictly
      attached? Current spec: attached (owned by an element), with `annotatedElements` for
      cross-references.
- [ ] Are `AssociationClass` and `Signal` in v1? Tentatively: `Signal` yes (for sequence
      async messages); `AssociationClass` yes (class-with-association, common in domain
      models).
