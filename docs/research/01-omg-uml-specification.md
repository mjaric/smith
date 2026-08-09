---
status: REVIEW
---

# Research 01 — OMG UML 2.5.1: Map into the Specification

Reference-grade map of the OMG UML 2.5.1 formal specification (formal/2017-12-05) for Smith
implementers. Dense facts, `§X.Y` citations into the spec, no tutorial prose. For shape/line
notation details see [04-uml-visual-notation.md](./04-uml-visual-notation.md); for Smith's
metamodel contract see [../uml-model/01-metamodel-core.md](../uml-model/01-metamodel-core.md).

## Spec Identification & Clause Map

| Field | Value |
|---|---|
| Title | Unified Modeling Language (UML), Version 2.5.1 |
| OMG document ID | formal/2017-12-05 (also cited formal/17-12-05; changebar: formal/17-12-06). Some citing docs (e.g., SysML 1.6) reference it as formal/2017-12-01 |
| Publication | December 2017, formal status |
| Normative PDF | https://www.omg.org/spec/UML/2.5.1/PDF (~780 pages, single document) |
| Spec landing page | https://www.omg.org/spec/UML/2.5.1/ |
| Supersedes | UML 2.5; merges the former UML Infrastructure + Superstructure into one document (§6.1) |
| Metamodel basis | MOF/CMOF; metamodel partitioned into packages corresponding to clauses, all owned by top-level package `UML` (§6.1) |
| Conformance (§2) | five kinds: abstract syntax, concrete syntax, model interchange, diagram interchange, semantics |

**Terminology corrections (verified):**

- UML 2.5.1 has **no metaclass `MetadataElement`** — that is a UML 2.6/MOF-level addition. The
  root abstraction in 2.5.1 is `Element` (§7.2 Root).
- Profiles are specified in **§12.3** (clause 12 "Packages"), not clause 18 (clause 18 is Use Cases).
- Diagram Interchange is **Annex B** of this document built on DD 1.1; standalone UMLDI 1.0
  (formal/06-04-04, 2006) is the superseded predecessor. OMG document formal/2013-12-01 is
  *Common Terminology Services 2* v1.1 — unrelated to UML DI.

### Clause map

Every metamodel clause follows one internal template (§6.4):
**Summary → Abstract Syntax → Semantics → Notation → Examples → Classifier Descriptions →
Association Descriptions**. The last two are machine-generated from the metamodel; each
metaclass is fully specified exactly once, in its clause's Classifier/Association Descriptions.

| Clause | Title | Metamodel package |
|---|---|---|
| 1 | Scope | — |
| 2 | Conformance | — |
| 3 | Normative References | — |
| 4 | Terms and Definitions | — |
| 5 | Notational Conventions | — |
| 6 | Additional Information (6.1 simplification, 6.2 architectural alignment, 6.3 semantics, 6.4 how to read) | — |
| 7 | Common Structure (7.2 Root, 7.3 Templates, 7.4 Namespaces, 7.5 Types & Multiplicity, 7.6 Constraints, 7.7 Dependencies, 7.8/7.9 generated descriptions) | `UML::CommonStructure` |
| 8 | Values | `UML::Values` |
| 9 | Classification | `UML::Classification` |
| 10 | Simple Classifiers | `UML::SimpleClassifiers` |
| 11 | Structured Classifiers | `UML::StructuredClassifiers` |
| 12 | Packages (12.2 Packages, 12.3 Profiles) | `UML::Packages` |
| 13 | Common Behavior | `UML::CommonBehaviors` |
| 14 | StateMachines | `UML::StateMachines` |
| 15 | Activities | `UML::Activities` |
| 16 | Actions | `UML::Actions` |
| 17 | Interactions | `UML::Interactions` |
| 18 | Use Cases | `UML::UseCases` |
| 19 | Deployments | `UML::Deployments` |
| 20 | Information Flows | `UML::InformationFlows` |
| 21 | Primitive Types | `PrimitiveTypes` model library |
| 22 | Standard Profile | `StandardProfile` model |
| Annex A | Diagrams — 14-kind taxonomy (Figure A.5, PDF p.685) | — |
| Annex B | UML Diagram Interchange (B.1–B.8) | `UMLDI` |
| Annex C | Keywords | — |
| Annex D | Tabular Notation for Sequence Diagrams | — |
| Annex E | XMI Serialization and Schema | — |

## Root Abstractions: Element and NamedElement

> Ticket terminology: UML 2.5.1 has no `MetadataElement`; the root is `Element`. Smith's rule
> "every model element extends NamedElement" maps to the 2.5.1 hierarchy `Element` → `NamedElement`.

### Element (§7.2 Root; description within §7.8 Classifier Descriptions)

Abstract root of the metaclass hierarchy; no superclass.

- Associations: `owner : Element [0..1]` (derived union), `ownedElement : Element [0..*]`
  (derived union, opposite `owner`), `ownedComment : Comment [0..*]` (composite).
- Operations: `allOwnedElements()`, `mustBeOwned() : Boolean`, `destroy()`, `createOwnedComment()`,
  `hasKeyword(keyword)`.
- Constraints: `not_own_self` (an element must not directly or indirectly own itself),
  `has_owner` (must own an owner when `mustBeOwned()` is true).
- No notation of its own; concrete subclasses define notation.

### NamedElement (§7.8.9; prose §7.4 Namespaces)

The naming contract every Smith node implements.

- Attributes: `name : String [0..1]`; `qualifiedName : String [0..1]` (derived);
  `visibility : VisibilityKind [0..1]` ∈ {public, private, protected, package}.
- Associations: `namespace : Namespace [0..1]` (derived); `clientDependency : Dependency [0..*]`
  (made derived in 2.5, §6.1); `nameExpression : StringExpression [0..1]`.
- Constraints:
  - `has_qualified_name` / `has_no_qualified_name` — qualifiedName joins containing-namespace
    names root → element with `::`; it does not exist if the element is unnamed or any
    enclosing namespace is unnamed.
  - `visibility_needs_ownership` — an element not owned by a Namespace has no visibility.
  - `isDistinguishableFrom(n, ns)` — coexistence test inside a namespace.
- Anonymous is legal: absence of name ≠ empty name.

### Adjacent root abstractions

- `PackageableElement` (Common Structure, described in §7.8): NamedElement that may be owned
  directly by a Package; default visibility `public` when owned by a Namespace (§12.2).
- `RedefinableElement`: `isLeaf : Boolean [1..1] = false`; redefinition in specialization contexts.
- `Type` / `TypedElement` (§7.5): value-typing contract used by Property, Parameter, etc.
- `MultiplicityElement` (§7.8.8.1; prose §7.5.3.2): `isOrdered`, `isUnique`, `lower`, `upper`.

**Smith mapping:** node properties `name`, `visibility`; `qualifiedName` derived from the
ownership path; `clientDependency` derived from outgoing Dependency edges.

## Ownership & Namespace Model (the tree Smith replicates)

Ownership in UML is **composition containment**: every owning association end is composite,
a part has at most one composite owner, and deleting the owner cascades to owned parts
(§7.2, §9.5.3.3 aggregation/composition semantics). `Element::owner` is the derived union of
all owning ends; `ownedElement` its inverse. Any element may not own itself, directly or
indirectly (§7.2 constraint `not_own_self`).

### Namespace (§7.8.10; prose §7.4)

Abstract NamedElement that **owns and/or imports** NamedElements identifiable by name (§7.8.10.1).

- `member : NamedElement [0..*]` (derived union) = `ownedMember ∪ importedMember`.
- `ownedMember : NamedElement [0..*]` (derived; subsets `ownedElement`).
- `importedMember : NamedElement [0..*]` (derived), reached via:
  - `elementImport : ElementImport [0..*]` — individual import with optional alias + visibility;
  - `packageImport : PackageImport [0..*]` — import of a Package's public members.
- Operations: `getNamesOfMember()`, `membersAreDistinguishable()`, `excludeCollisions()`,
  `importMembers()`.
- Members must be pairwise distinguishable within a namespace. Known spec defect: the
  `excludeCollisions` OCL self-compares (issues UMLR-847/UMLR-776, open in 2.5.1) —
  implementations must filter `imp1 <> imp2`.

A NamedElement is identifiable in a Namespace by (a) direct ownership, (b) ElementImport,
or (c) PackageImport (§7.4, §7.8.10 Semantics).

### Package (§12.2; backbone of Smith's ownership tree)

`Package` specializes `Namespace`, `PackageableElement`, `TemplateableElement` — it owns
members, can itself be owned by a package, and can be templated.

- `packagedElement : PackageableElement [0..*]` — **composite, subsets `ownedMember`** —
  the primary ownership edge: everything "inside" a package sits here.
- `nestedPackage : Package [0..*]` / `nestingPackage : Package [0..1]` — package nesting is
  ownership.
- `ownedType : Type [0..*]` (subsets `packagedElement`); `ownedStereotype : Stereotype [0..*]` (derived).
- `packageMerge : PackageMerge [0..*]`; `profileApplication : ProfileApplication [0..*]` (§12.3);
  `uri : String [0..1]` (RFC 2396 identifier).
- `Model` specializes `Package` — the root namespace of a model. `Profile` also specializes
  `Package` (§12.3).
- Visibility: owned elements with visibility are public or private (Package constraint);
  `makesVisible()` defines what a Package exposes outside (§12.2).
- Package-like containers: a Component "acts like a Package" for the elements involved in its
  definition, typically owning its realizing Classifiers (§11.6.3.1).

### The ownership tree Smith replicates

```text
Model ─packagedElement─▶ Package ─packagedElement─▶ { Package | Classifier | Behavior | … }
                            │
                            └─nestedPackage─▶ Package   (recursive)
```

- Every composite association end in the metamodel is an ownership edge: Classifier→`attribute`
  Property, BehavioredClassifier→`ownedBehavior` Behavior, Activity→ActivityNode/ActivityEdge,
  Interaction→Lifeline/Message/Fragment, StateMachine→Region→Vertex, UseCase→ExtensionPoint,
  Component→`packagedElement`, Package→`packagedElement`.
- Diagrams are **not** in this tree: a diagram frame's heading denotes the enclosing namespace
  or owning model element (Annex A), and Annex B presentation elements reference model
  elements — the spec basis for Smith's "diagram = view" model (see Diagram Interchange below).
- Deleting a package deletes its subtree; reparenting = moving the composite edge.

## Element Category Taxonomy

Metaclasses Smith implements for full UML. Location = clause where the metaclass is specified;
each appears exactly once in that clause's Classifier Descriptions (§6.4).

| Metaclass | One-line definition | Spec |
|---|---|---|
| `BehavioredClassifier` | Abstract classifier that can own Behaviors (`ownedBehavior`); supertype of Class, UseCase, Actor, Component, Collaboration, Interface | §9.2 (Classification), described in §9.9 |
| `Class` | Classifier of a set of objects sharing features; owns attributes/operations; may own a classifier behavior | §11.4; description §11.8.3.1 |
| `Interface` | Declaration of a coherent set of public features and obligations forming a contract; realized by classifiers | §10.4 (§10.4.3.1) |
| `Association` | Relationship **and** Classifier; typed links between member ends; binary or n-ary (§11.5.3.1); aggregation kinds | §11.5 |
| `UseCase` | BehavioredClassifier specifying complete behavior of a subject yielding observable value; owns ExtensionPoints | §18.1 |
| `Actor` | BehavioredClassifier for a role external to the subject that interacts with its use cases | §18 |
| `Activity` | Behavior as a graph of Actions/ControlNodes/ObjectNodes joined by ControlFlow/ObjectFlow edges; partitions, groups | §15.2 |
| `Interaction` | Behavior expressing message exchange among Lifelines ordered via OccurrenceSpecifications and fragments | §17.2 |
| `StateMachine` | Behavior of Regions of Vertices and Transitions; behavioral (§14.2) and protocol (§14.4) variants | §14 |
| `Component` | Modular replaceable unit with provided/required Interfaces (optionally via Ports); encapsulates realizing classifiers; package-like | §11.6 (§11.6.3) |
| `Node` | DeploymentTarget — computational resource for deploying artifacts; specializations Device, ExecutionEnvironment | §19.4 |
| `Artifact` | Physical information item of a system; specializations DeploymentSpecification, DeployedArtifact | §19.3 |
| `Signal` | Classifier whose instances are communications sent asynchronously and received via SignalEvents | §10.3 |
| `DataType` | Classifier whose instances are distinguished only by value | §10.2 (§10.2.1) |
| `Enumeration` | DataType whose values are individually named, ordered EnumerationLiterals | §10.5 (§10.5.4.1) |

Pervasive supporting metaclasses: `Property` (§9.5 prose, §9.9.17), `Operation` (§9.6.3.1),
`Feature` (§9.4.3.1), `BehavioralFeature` (§9.9.2.1), `InstanceSpecification`/`Slot` (§9.8),
`Constraint` (§7.8.3.1), ValueSpecification family (§8), `TemplateParameter`/`TemplateBinding`
(§7.3), `ExtensionPoint`/`Include`/`Extend` (§18.1), `Port` (§11.3), `Connector` (§11.2),
`Collaboration`/`CollaborationUse` (§11.7), `InformationFlow`/`InformationItem` (§20, §20.2.2),
`Event`/`Trigger` (§13.3, §13.4.11.4).

## Relationship & Dependency Metaclasses (traceability substrate)

### Relationship / DirectedRelationship (§7.8, Common Structure)

- `Relationship` (abstract): `relatedElement : Element [1..*]` (derived union). No notation of its own.
- `DirectedRelationship` (§7.8.5.1): `source : Element [1..*]`, `target : Element [1..*]`
  (both derived, subset `relatedElement`).

### Dependency family (§7.7 Dependencies)

- `Dependency` (§7.7.1; description §7.8.4.3): DirectedRelationship with
  `client : NamedElement [1..*]`, `supplier : NamedElement [1..*]` — "a single model element
  or set of model elements requires other model elements for their specification or
  implementation". Signifies a supplier/client relation; modification of a supplier can impact
  clients.
- Notation §7.7.4: dashed line, open arrowhead client → supplier; may carry name and stereotypes.
- `NamedElement::clientDependency` is derived (§6.1).

Subclasses/specializations (DirectedRelationships; most specialize Dependency):

| Metaclass | Meaning | Spec |
|---|---|---|
| `Usage` | client requires supplier for full implementation/operation | §7.7 |
| `Abstraction` | same concept at different abstraction levels/viewpoints; optional `mapping : OpaqueExpression` | §7.7.3.3 |
| `Realization` | client realizes the supplier's specification | §7.7 |
| `InterfaceRealization` | classifier realizes an Interface | §11.4 |
| `ComponentRealization` | realizing classifiers of a Component | §11.6 |
| `Include` | UseCase mandatorily includes another's behavior (also a Dependency) | §18.1 |
| `Extend` | UseCase conditionally extends another at ExtensionPoints | §18.1 |
| `Deployment` | deployment of an Artifact to a DeploymentTarget | §19.2 |
| `Manifestation` | Artifact manifests (concretely implements) a PackageableElement | §19.3 |
| `InformationFlow` | flow of information items between sources and targets | §20 |
| `PackageMerge` | receiving package incorporates merged package contents | §12.2 |
| `Generalization` | taxonomic specific→general (below) | §9 |
| `Extension` | Stereotype ↔ extended metaclass | §12.4.1.1 |
| `TemplateBinding` | bound element binds a template signature | §7.3 |

### Other core relations

- `Generalization` (§9.9.7; semantics §9.2.3.2, §9.9.7.3): taxonomic DirectedRelationship;
  `general : Classifier [1..1]`, `specific : Classifier [1..1]`; each instance of the specific
  is an instance of the general; specific inherits general's features; `isSubstitutable()`.
  `GeneralizationSet` (§9.7 prose; description §9.9.8, attributes §9.9.8.4): `isDisjoint`,
  `isCovering`, powertypeExtent.
- `Association` (§11.5; §11.5.3.1): `memberEnd : Property [2..*]` (ordered),
  `navigableOwnedEnd : Property [0..*]`, `endType : Type [1..*]`, `isDerived`; ends carry
  multiplicity and aggregation (none/shared/composite — §9.5.3.3); two member ends may share
  the same type (§11.5.3.1). Association Descriptions: §11.9.
- `Connector` (§11.2, within StructuredClassifier): structural wiring between ConnectableElements
  (parts, ports); `end : ConnectorEnd [2..*]`, optional `type : Association`; execution-time
  request routing along connector instances (§11.6.3).
- `Message` (§17.4): NamedElement defining one specific communication between Lifelines of an
  Interaction; `messageSort` (synchCall/asynchCall/asynchSignal/createMessage/deleteMessage/reply);
  `signature : NamedElement [0..1]` (Operation or Signal); `sendEvent`/`receiveEvent` MessageEnds
  (§17.5). Reply notation: dashed with open or filled arrowhead (§17.8).

**Smith mapping:** traceability relations are typed Dependencies (or stereotyped Dependencies);
reachability, RTM, and orphan analyses traverse DirectedRelationship `source`/`target` edges.

## Diagram Kinds (14) — Annex A

Normative diagram taxonomy and contents: **Annex A** (Figure A.5, PDF p.685); per-kind DI
classes: **Annex B** (below). Two spec rules:

1. The kind of a diagram is defined by its **primary graphical symbols**, not its title (Annex A).
2. A diagram frame's heading denotes the **kind, name, and parameters of the enclosing
   namespace or the owning model element** of what the contents area shows (Annex A) —
   Smith diagram frames bind to a namespace/model element and never own contents.

Kinds may be mixed; boundaries are not strictly enforced (Annex A).

### Structure diagrams

| Diagram | Primary elements shown | Ref |
|---|---|---|
| Class | Class, Interface, features, Constraint, Association, Generalization, Dependency | Annex A.3; DI class B.7.6 (p.726) |
| Object | InstanceSpecification (objects), Slot, Link | Annex A; §9.8 |
| Component | Component, provided/required Interface, Port, Artifact, ComponentRealization, Usage | §11.6.4; DI class B.7.10 (p.728) |
| Composite Structure | StructuredClassifier parts (Property), Port, Connector; CollaborationUse | §11.2–§11.3, §11.7 |
| Package | Package, PackageableElement, ElementImport, PackageImport, PackageMerge, Dependency | §12.2 (Notation) |
| Deployment | Node, Device, ExecutionEnvironment, Artifact, Deployment, DeploymentSpecification, CommunicationPath | §19.4 (Notation) |
| Profile | Profile, Stereotype, Extension, metaclass reference, ProfileApplication | §12.3 (Notation) |

### Behavior diagrams

| Diagram | Primary elements shown | Ref |
|---|---|---|
| Use Case | UseCase, Actor, subject, Include, Extend, Association | §18.1 (Notation) |
| Activity | Activity, Action, control/object nodes, ControlFlow/ObjectFlow, partitions, structured nodes | §15.2.4 |
| State Machine | State, Region, Pseudostate, Transition, FinalState, Trigger/Event; protocol variant | §14.2.4; §14.4 |
| Sequence | Lifeline, ExecutionSpecification, Message, CombinedFragment, InteractionUse, StateInvariant | §17.8 |
| Communication | Lifeline, Message with sequence numbering, structural organization | §17.9 |
| Timing | state/condition timelines, TimeConstraint, DurationConstraint | §17.11 |
| Interaction Overview | activity-style flow whose nodes are Interactions / InteractionUses | §17.10 |

## Stereotypes & Profiles (§12.3, §22)

Profiles live in clause 12 "Packages": §12.3 Profiles (prose) + §12.4 Classifier Descriptions
(`Extension` §12.4.1.1, `Stereotype` §12.4.9, plus Profile, ProfileApplication, ExtensionEnd).

- `Profile` specializes `Package`: owns Stereotypes and metaclass references
  (`metaclassReference : ElementImport`); applied to Packages via `ProfileApplication`
  (§12.2 association `profileApplication`).
- `Stereotype` (§12.4.9) specializes `Class`: its properties are **tagged values**; may be
  abstract; may only generalize/specialize other Stereotypes (constraint `generalize`,
  §12.4.9.6); must be directly or indirectly owned by a Profile — `profile()` returns the
  containing profile (§12.4.9.5; `/profile : Profile [1..1]`, §12.4.9.4; packages nested in a
  Profile may own stereotypes, issue UMLR-826).
- `Extension` (§12.4.1.1) specializes `Association`: Stereotype ↔ extended metaclass;
  `ExtensionEnd` multiplicity [0..1] = optional, [1..1] = **required** stereotype.
- Application semantics — §12.3.3.1.3 "MOF-Equivalent Semantics" (PDF pp.297–299): a profile
  maps to a CMOF package; applying a stereotype creates an instance of its CMOF class
  compositionally associated with the extended Element; tagged values map to attributes.
  Composite-aggregation stereotype properties cannot be typed by Stereotypes or metaclasses
  (§12.3.3.4, pp.301–302).
- Notation: `«stereotype»` keyword above the name; Profile shown as `«profile»` package,
  Stereotype as class-like rectangle with `«stereotype»` (§12.3 Notation). Keywords (Annex C)
  share the guillemet notation but are reserved words.
- Standard Profile (§22, PDF p.721): predefined stereotype set; conforming tools should support
  all of it (§22.1). Includes «Subsystem», «Specification», «Realization» (Component),
  «Instantiate», «Create», «Destroy», «script» (Artifact), etc. (§22.3). Known defects:
  UMLR-832 (missing «script» extends arrow), UMLR-833 («Create»/«Instantiate» overlap).
- Machine-readable: StandardProfile.xmi (ptc/18-01-03).

**Smith mapping:** Requirement/TestCase extensions = a Smith-owned profile; traceability
relations = stereotyped Dependencies.

## Comment / Note Mechanism (§7.8.2)

- `Comment` (§7.8.2.1) specializes `Element` (not NamedElement): `body : String [0..1]`;
  `annotatedElement : Element [0..*]`; owned via `owningElement : Element [0..1]` (composition —
  unowned/floating comments are legal; issue UMLR-820).
- Any Element owns `ownedComment : Comment [0..*]` (§7.2 Root).
- A comment annotates but does not alter target semantics; one comment may annotate many
  elements; empty bodies are legal (placeholder / graphics-only notes).
- Notation: rectangle with folded corner; dashed associations to annotated elements (§7.8.2).

**Smith mapping:** comments are nodes with `annotates` edges; `body` is search-indexed.

## Diagram Interchange (Annex B + DD) — the "diagram as a view" basis

UML 2.5.1 separates **abstract syntax** (the model) from **presentation** (diagram geometry).
Presentation is specified by:

1. **Annex B "UML Diagram Interchange"** of the UML 2.5.1 document — B.1 Summary, B.2 Generic,
   B.3 Structure, B.4 Behavior, B.5 Information Flows, B.6 UML Notations ↔ UMLDI representations,
   B.7 Classifier Descriptions, B.8 Association Descriptions (PDF pp.~720–780).
2. **OMG Diagram Definition (DD) 1.1** (formal/15-06-01, https://www.omg.org/spec/DD/1.1/PDF) —
   the generic foundation packages DC (Diagram Common), DG (Diagram Geometry), DI (Diagram
   Interchange); XMI: DC.xmi/DG.xmi/DI.xmi (ptc/14-03-04).
3. **UMLDI metamodel** extending DI with per-diagram-kind classes — normative XMI:
   https://www.omg.org/spec/UML/20161101/UMLDI.xmi (ptc/18-01-04).

Key facts:

- `UMLDiagram` has a `kind`; per-kind DI classes (e.g., `UMLClassDiagram`, `UMLComponentDiagram`)
  specialize `UMLStructureDiagram`/`UMLBehaviorDiagram` (Annex B.3/B.4). Several constrain
  `modelElement->isEmpty()` (UMLClassDiagram B.7.6.3 p.726; UMLComponentDiagram B.7.10.3 p.728) —
  their kind is determined by contents per Annex A, not by a model binding.
- Diagram elements (`UMLShape`, `UMLEdge`) reference the presented model element via
  `modelElement` — **identity lives in the model; geometry lives in DI**. The same model element
  may appear on any number of diagrams. This is exactly Smith's view model: deleting a diagram
  or shape never deletes a model element; deleting a model element cascades to (or invalidates)
  its presentations.
- Shapes carry bounds (position/size); edges carry anchors/waypoints; semantic styles govern
  line/arrowhead rendering (notation per each clause's Notation subsection and Annex C).
- Conformance (§2) defines "diagram interchange" as one of five conformance kinds — compliant
  tools import/export DI.
- Legacy: standalone **UMLDI 1.0** (formal/06-04-04, April 2006,
  https://www.omg.org/spec/UMLDI/1.0/PDF) predates Annex B and is superseded. **Do not cite
  formal/2013-12-01** for DI — that ID is Common Terminology Services 2 v1.1.
- XMI nsPrefix `umldi` (Annex E.5).

**Smith mapping:** DI records reference model nodes by id and never own them; graph schema
keeps presentation as view-only nodes/edges. See [../uml-model/06-diagram-interchange.md](../uml-model/06-diagram-interchange.md).

## Normative Machine-Readable Artifacts

| Artifact | OMG file ID | URL |
|---|---|---|
| UML 2.5.1 abstract syntax metamodel (XMI) | ptc/18-01-01 | https://www.omg.org/spec/UML/20161101/UML.xmi |
| Primitive Types model library | ptc/18-01-02 | https://www.omg.org/spec/UML/20161101/PrimitiveTypes.xmi |
| Standard Profile | ptc/18-01-03 | https://www.omg.org/spec/UML/20161101/StandardProfile.xmi |
| UMLDI metamodel | ptc/18-01-04 | https://www.omg.org/spec/UML/20161101/UMLDI.xmi |

XMI serialization conventions: Annex E (nsPrefixes: `uml`, `primitives`, `StandardProfile`, `umldi`).

Known spec defects relevant to implementers (OMG issue tracker, open as of 2026-08):
UMLR-847/UMLR-776 (`excludeCollisions` OCL), UMLR-771/UMLR-758 (UML.xmi well-formedness /
duplicate xmi:id), UMLR-826 (stereotype ownership diagram), UMLR-848 (initial pseudostate
incoming transitions, §14.2.3.7/§14.5.6.7), UMLR-845 (malformed XMI example pp.298–299).

## Sources

1. OMG UML 2.5.1 spec page (document IDs, machine-readable files): https://www.omg.org/spec/UML/2.5.1/About-UML
2. Normative PDF (structure verified via secondary anchors below): https://www.omg.org/spec/UML/2.5.1/PDF
3. Normative XMI files: https://www.omg.org/spec/UML/20161101/UML.xmi · https://www.omg.org/spec/UML/20161101/PrimitiveTypes.xmi · https://www.omg.org/spec/UML/20161101/StandardProfile.xmi · https://www.omg.org/spec/UML/20161101/UMLDI.xmi
4. UMLDI 1.0 (legacy, formal/06-04-04): https://www.omg.org/spec/UMLDI/1.0/About-UMLDI
5. Diagram Definition 1.1 (formal/15-06-01): https://www.omg.org/spec/DD/1.1/About-DD ; DD 1.0 (formal/12-07-01): https://www.omg.org/spec/DD/1.0/About-DD
6. OMG UML issue tracker — section/page anchors for 2.5.1 (9.9.16.4 p.190, 9.9.2.7–8 p.173, 9.9.8.4 p.181, 12.3.3.4 pp.301–302, 12.4.9.4–6 p.322, 20.2.2 p.716, 13.4.11.4 p.342, 11.6.2 p.210, 14.2.3.7/14.5.6.7, §22 p.721): https://issues.omg.org/issues/spec/UML/2.5.1?view=ALL
7. Issue UMLR-813 (§6.4 document-structure descriptions): https://issues.omg.org/issues/UMLR-813
8. Issue UMLR-793 (Annex references DD 1.1): https://issues.omg.org/issues/UMLR-793
9. ISO 19103:2024 preview (iTech) — exact UML 2.5.1 subsection citations (7.8.2.1, 7.8.3.1, 7.8.4.3, 7.8.5.1, 7.8.8.1, 7.8.9, 7.8.10.1, 7.7.3.3, 9.2.1, 9.2.3.2, 9.4.3.1, 9.5.3.3, 9.6.3.1, 9.9.2.1, 9.9.7.3, 10.2.1, 10.4.3.1, 10.5.4.1, 11.5.3.1, 11.8.3.1, 12.4.1.1, Annex A.3, Annex C, 6.2.3, 6.3.1): https://cdn.standards.iteh.ai/samples/83454/3bc656ab85fc4dcc8652d26884b88c36/ISO-19103-2024.pdf
10. UML 2.5.1 clause-by-clause TOC walkthrough (clauses 1–22, Annexes A–E with subsections): https://showa-yojyo.github.io/notebook/omg15/index.html
11. Ed Seidewitz, "UML 2.5: Specification Simplification" (clause outline, §7 subsections, five conformance kinds): https://www.slideshare.net/slideshow/uml-25-specification-simplification/12210345
12. UML 2.4.1 → 2.5.1 structure changes; 14-diagram list; XMI nsPrefixes; DD-based DI: http://www.uml.org.cn/modeler/202206091.asp?artid=25193
13. Eclipse UML2 5.3 javadoc (implements UML 2.5.x) — NamedElement, Package APIs/constraints: https://download.eclipse.org/modeling/mdt/uml2/javadoc/5.3.0/org/eclipse/uml2/uml/NamedElement.html · https://download.eclipse.org/modeling/mdt/uml2/javadoc/5.3.0/org/eclipse/uml2/uml/Package.html
14. Diagram taxonomy & per-diagram elements: https://www.uml-diagrams.org/uml-25-diagrams.html ; core elements/ownership: https://www.uml-diagrams.org/uml-core.html
15. Annex A p.685 / Annex B B.7.6, B.7.10 pp.726–728 / diagram heading rule: https://softwareengineering.stackexchange.com/questions/445408
16. §6.1 simplification changes; PackageableElement default visibility: https://training-course-material.com/training/What_is_New_in_UML_2.5
17. §22 Standard Profile p.721 anchor: https://www.volcengine.com/article/1176569 ; Comment multiplicities (owningElement/annotatedElement): https://www.volcengine.com/article/1185791
18. Gaphor docs — UML 2.5.1 metamodel package order corroboration: https://docs.gaphor.org/en/latest/models/uml.html
19. Alternate document-ID citations: https://nisp.nw3.dk/standard/omg-formal-17-12-01-uml.html ; SysML 1.6 cites UML 2.5.1 as formal/2017-12-01: https://sysml.org/.res/docs/specs/OMGSysML-v1.6-19-11-01.pdf
20. §11.6 Components verbatim (cross-refs to §7.7.4, §9.2.4, §11.5.4, Clause 19): https://raw.githubusercontent.com/lichenliang666/UML-2.5.1-Specification/master/source/_posts/11-6-1-Components.md
21. formal/2013-12-01 identified as CTS2 v1.1 (not UML DI): https://www.omg.org/spec/CTS2/1.1/PDF
