# UML Visual Notation Standards — Research Reference

**Status:** REVIEW

This document is the **notation reference** for Smith's diagram renderer. It captures the
official visual notation of UML 2.5.1 for all 14 diagram families — element shapes, line
styles, arrowheads, label syntax, and compartment rules — plus the places where the OMG
specification deliberately leaves presentation open (where Smith's palette and ergonomics are
product decisions). It is a **reference**, not an implementation spec; the rendering contract
lives in `../uml-model/06-diagram-interchange.md` and the palette in `../ui-ux/01-design-system.md`.

Primary authority: **OMG UML 2.5.1** (formal/17-12-05). Each diagram family section cites the
spec clause(s) that define its notation. Clause numbering verified against the OMG OCUP2
coverage maps and the OMG UML 2.5 issue tracker.

Normative IDs in this document use the prefix `[NOTE-*]`. They record what the UML spec fixes
(→ Smith MUST preserve) versus what it leaves open (→ Smith MAY decide). The UI/UX and renderer
specs restate these as `REQ-*` requirements.

---

## Where notation lives in the UML 2.5.1 specification

UML 2.5.1 is a single document (Infrastructure and Superstructure were merged in 2.5). Every
language-unit clause follows the pattern *Summary → Abstract Syntax → Semantics → **Notation** →
Examples*. The renderer only needs the Notation subsections, but semantics constrain what may be
drawn.

| Diagram family | Spec clause(s) defining notation |
|---|---|
| Class (+ Object) | §7.7 Dependencies; §9.2.4 Classifier notation; §9.8 Instances; §10.4.4 Interfaces; §11.4.4 Classes; §11.5.4 Associations |
| Composite Structure | §11.2.4 StructuredClassifiers; §11.3.4 Ports |
| Component | §11.6.4 Components; §11.3.4 Ports; §10.4.4 Interfaces |
| Package | §12.2.4 Packages |
| Profile | §12.3.4 Profiles; §22 Standard Profile |
| Use Case | §18.1.4 UseCases |
| Activity | §15.2.4 Activities; §15.3.4 Control Nodes; §15.4.4 Object Nodes; §15.6.4 Activity Groups; §16.2.4 Actions & Pins; §16.11 Structured Actions; §16.12 Expansion Regions |
| State Machine | §14.2.4 Behavior StateMachines (14.2.4.1–14.2.4.9) |
| Sequence | §17.2.4 Interactions; §17.3.4 Lifelines; §17.4.4 Messages; §17.6.4 Fragments; §17.7.4 InteractionUses; §17.8 Sequence Diagrams |
| Communication | §17.9 Communication Diagrams |
| Interaction Overview | §17.10 Interaction Overview Diagrams |
| Timing | §17.11 Timing Diagrams (optional for conforming tools; Smith implements it) |
| Deployment | §19.2.4 Deployments; §19.3.4 Artifacts; §19.4.4 Nodes |
| Cross-cutting | §7.4 Namespaces (circle-plus), §7.5 Multiplicity, §7.6 Constraints, §7.8 Comments, §6 BNF conventions |
| Diagram taxonomy & interchange | **Annex A** (the 14 diagram kinds + allowed contents), **Annex B** (UML Diagram Interchange metamodel), Annex C (keywords), Annex D (tabular sequence notation) |

UMLDI history: the standalone **UML Diagram Interchange 1.0** spec (formal/06-04-04) was folded
into UML 2.5 as **Annex B**, with the normative metamodel published as `UMLDI.xmi` (ptc/18-01-04).
UMLDI models diagram *geometry* (shapes, edges, styles) built on OMG Diagram Definition (DD);
its style metamodel carries fill/line/font attributes but fixes no palette — consistent with
§"Notation freedom" below. This supports Smith's diagrams-as-views architecture: DI elements
reference model elements and never own them (see `../uml-model/06-diagram-interchange.md`).

---

## Cross-cutting notation rules (all diagrams)

These apply to every diagram family and are the highest-leverage rules for the renderer.

**[NOTE-CORE-001]** The spec fixes **shape topology, line style (solid/dashed), arrowhead kind,
compartment structure, and label syntax**. Smith MUST preserve these exactly.
**[NOTE-CORE-002]** The spec is monochrome. It fixes **no colors, fonts, sizes, corner radii,
line weights, or spacing**. Palette and typography are Smith product decisions
(`../ui-ux/01-design-system.md`).

### Names, keywords, stereotypes

- Classifier/element name centered in the name compartment; qualified names use `::`.
- Standard keywords and stereotype names are enclosed in **guillemets** `«…»` and placed
  *above* the element name (e.g., `«interface»`, `«component»`, `«actor»`). **[NOTE-CORE-003]**
- Multiple applied keywords may be comma-separated in one guillemet pair, or stacked.
- Profiles may define **icons** that may be shown instead of/above the keyword (§12.3.4).
- Smith's own extensions (`«requirement»`, `«testCase»`, trace relations) render with the same
guillemet convention — see §"Smith extension notation".

### Visibility and feature strings (class-family diagrams)

**[NOTE-CORE-004]** Visibility prefixes on attributes, operations, and association ends:

| Symbol | VisibilityKind |
|---|---|
| `+` | public |
| `-` | private |
| `#` | protected |
| `~` | package |

Feature string syntax (BNF-style, per §9.2.4 classifier notation):

```
<attribute> ::= [<visibility>] <name> ['['<multiplicity>']'] [':' <type>] ['=' <default>] ['{'<prop-modifier>'}']
<operation> ::= [<visibility>] <name> '(' [<parameter> {',' <parameter>}] ')' [':' <return-type>] ['{'<prop-modifier>'}']
<parameter> ::= [<direction>] <name> [':' <type>]
```

**[NOTE-CORE-005]** Multiplicity (§7.5): `n` | `n..m` | `*` | `n..*`; `..` ranges; placed near
association ends / property slots. Unshown multiplicity means *no conclusion* may be drawn.

**[NOTE-CORE-006]** Text-style conventions that carry meaning:

- **Italics** — abstract classifier name; abstract operation.
- **Underline** — static feature; instance/object name (`name : Type` in object diagrams).
- **Leading `/`** — derived feature or derived association (`/total`).

**[NOTE-CORE-007]** Compartment suppression: a conforming tool MAY suppress any compartment or
feature (UML 2.5.1 §9.2.4). Smith's zoom levels and inspectors exploit this legally.

### Constraints and notes

- Constraint: text in braces `{constraint}` attached to the constrained element.
- Comment/Note: rectangle with a **dog-ear (folded top-right corner)**, attached to elements
by a dashed line (§7.8).

### Arrowhead and line-style taxonomy

**[NOTE-CORE-008] [NOTE-CORE-009] [NOTE-CORE-010]** The complete normative graphic vocabulary.
The renderer implements this table once and every diagram family reuses it.

| Graphic | Line | End/head | Used for |
|---|---|---|---|
| Association / link | solid | plain | associations, object links, association paths |
| Generalization | solid | **hollow triangle** at the general (super) end | class, interface, use-case, actor, component generalization |
| Realization / InterfaceRealization | **dashed** | **hollow triangle** at the supplier end | interface realization, component realization, abstraction `«realize»` |
| Dependency family | **dashed** | **open arrowhead** at supplier | `«use»`, `«call»`, `«import»`, `«merge»`, `«access»`, `«apply»`, `«deploy»`, `«manifest»`, `«trace»`, `«include»`, `«extend»` |
| Composition | solid | **filled diamond** at the whole end | composite aggregation |
| Aggregation | solid | **hollow diamond** at the whole end | shared aggregation |
| Navigable end | solid | open arrowhead at end | navigability (may be suppressed) |
| Non-navigable end | solid | small **×** at end | explicit non-navigability |
| Class-owned association end | solid | small **filled dot** (diameter > line width, < aggregation diamond) | end ownership by the classifier (§11.5.4 dot notation; optional, per-diagram consistent) |
| Ordered/read direction | solid | small **filled triangle** pointing read order | ordered association ends |
| Fork / join / synchronization | **thick bar** (solid) | — | activity fork/join; state-machine fork/join pseudostates |
| Synchronous call message | solid | **filled triangle** head | sequence diagram `synchCall` |
| Asynchronous call / signal | solid | **open arrowhead** | sequence diagram `asynchCall`, `asynchSignal` |
| Reply message | **dashed** | open arrowhead | sequence diagram `reply` |
| Create message | **dashed** | open arrowhead | sequence diagram `createMessage` |
| Lost / found message | solid | small filled dot at unknown end | lost (dot at target end) / found (dot at source end) |

---

## 1. Class diagram

*Spec notation:* UML 2.5.1 §9.2.4 (Classifier notation), §10.4.4 (Interfaces), §11.4.4 (Classes),
§11.5.4 (Associations), §7.7 (Dependencies).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Class | Rectangle, name on top | Solid border; fill unspecified | Compartments in order: name, attributes, operations, receptions, internal structure. The classic **3-compartment** (name/attributes/operations) is industry convention; receptions/internal-structure compartments are optional (§11.4.4). Any compartment may be suppressed [NOTE-CORE-007] |
| Abstract class | Same rectangle | — | Name in *italics* [NOTE-CORE-006] |
| Active class (`isActive`) | Rectangle | — | **Double vertical side borders** (§11.4.4, Fig. 11.18) |
| Interface | Classifier rectangle with `«interface»`, or **lollipop** / **socket** icon (§10.4.4) | Solid border | Lollipop = provided (circle on stem); socket = required (half-circle cup). Rectangle form may show operations compartment |
| DataType / Enumeration / PrimitiveType | Classifier rectangle | — | `«dataType»`, `«enumeration»`, `«primitiveType»` keywords; literals compartment for enumerations |
| Association | Solid straight (or bent) line | Solid | May be named; leading `/` = derived. Bent segments carry no meaning [NOTE-CORE-008] |
| Aggregation | Solid line + **hollow diamond** at whole end | Hollow diamond | Diamond sized larger than line width (§11.5.4) |
| Composition | Solid line + **filled diamond** at whole end | Filled diamond | Same rules; merged-tree rendering of parallel aggregations is an allowed presentation option |
| Generalization | Solid line + **hollow triangle** at superclass | Hollow triangle, solid line | Arrow points to the general [NOTE-CORE-009] |
| Interface realization | **Dashed** line + hollow triangle at interface | Hollow triangle, dashed | Also `InterfaceRealization` from class to interface [NOTE-CORE-009] |
| Dependency | **Dashed** line + open arrowhead | Open head, dashed | Optional `«keyword»` label on the line [NOTE-CORE-008] |
| Navigability | Open arrowhead at navigable end; small × at non-navigable end | — | Suppression policy must be diagram-consistent (§11.5.4) |
| End ownership | Small filled dot at end | Filled dot | Dot = classifier-owned end; absence = association-owned end, only when dot notation is used consistently [NOTE-CORE-010] |
| Multiplicity | Text near end | — | `1`, `0..1`, `*`, `1..*`, `n..m` [NOTE-CORE-005] |
| Role name | Text near end | — | Positioned like end adornments |
| Qualified association | Small rectangle on the association path at the qualifying end | Solid | Contains qualifier attributes; part of the line, not of the classifier; MAY NOT be suppressed (§11.5.4) |
| Association class | Class rectangle attached to the association path by a **dashed** line | Dashed connector | Same name on path and class symbol |
| N-ary association | Diamond joining ≥3 solid lines | Solid | Only rendering form for n>2 |
| Read-order (ordered end) | Small filled triangle on the line | Filled | Points in reading direction |
| Line crossing | Optional semicircular jog | — | Like circuit diagrams; jog = no intersection |
| Constraint | `{expression}` | — | Braced text attached to element |
| Note | Dog-eared rectangle + dashed attachment | — | §7.8 |

## 2. Use Case diagram

*Spec notation:* UML 2.5.1 §18.1.4 (UseCases notation); relationships also §9.2.4, §7.7.

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Use case | **Ellipse** (horizontal) | Solid border; fill unspecified | Name inside the ellipse or below it; `«stereotype»` above name; optional `extension points` compartment with heading **extension points**; alternative classifier-rectangle form with ellipse icon in top-right corner (§18.1.4) |
| Actor | **Stick figure** | Solid strokes | Name near icon (usually above/below). Alternatives: classifier rectangle with `«actor»`, or a custom icon (e.g., a computer for non-human actors) |
| Subject (system boundary) | Rectangle enclosing use-case ellipses | Solid border | Name in top-left corner; stereotype keyword in guillemets above the name when the subject has one. Subject rectangle implies *applicability*, **not ownership** — Smith must not treat containment here as model ownership |
| Actor–use-case association | Solid line | Solid | Optional multiplicities |
| `«include»` | **Dashed** arrow, **open arrowhead** | Dashed/open | From base (including) use case **to** included use case; label `«include»` |
| `«extend»` | **Dashed** arrow, **open arrowhead** | Dashed/open | From extending use case **to** extended use case; label `«extend»`; condition/extension points may be shown in an attached comment |
| Generalization (actors or use cases) | Solid line + hollow triangle | Hollow triangle | Points at the general |
| Dependency | Dashed arrow, open head | Dashed/open | E.g., use case → classifier |

## 3. Activity diagram

*Spec notation:* UML 2.5.1 §15.2.4 (Activities), §15.3.4 (Control Nodes), §15.4.4 (Object Nodes),
§15.6.4 (Activity Groups), §16.2.4 (Actions/Pins), §16.11–16.12 (Structured Actions, Expansion).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Activity frame | Rectangle with corner **pentagon** containing keyword `activity` + name | Solid | Same frame convention as interaction diagrams (pentagon descriptor) |
| Action | **Rounded rectangle** | Solid border | Name inside; keyword for special actions (e.g., `«sendSignal»`, `«acceptEvent»` shapes: concave/convex pentagons for accept-time/accept-signal actions) |
| Pin | Small square on action border | Solid | Input/output pins; may be elided (§15.2.4) |
| Initial node | **Filled black circle** | Filled | Single per activity (semantically) |
| Activity final node | Circle containing an **×** | Solid | Stops all flows |
| Flow final node | Circle containing an **×** | Solid | Same shape as activity final; distinction is semantic |
| Decision node | **Diamond** | Solid | Optional name/label |
| Merge node | **Diamond** | Solid | Same shape as decision; combined decision+merge in one diamond allowed (§15.3.4) |
| Fork node | **Thick bar** | Solid, heavy weight | Horizontal or vertical |
| Join node | **Thick bar** | Solid, heavy weight | Combined fork+join single bar allowed |
| Object node | Rectangle | Solid | `«datastore»`, `«centralBuffer»` keywords where applicable |
| Control flow | Solid line + open arrowhead | Solid | Guard in braces on edge |
| Object flow | Solid line + open arrowhead | Solid | Same graphic as control flow; distinguished by context/labels; optional adornments (selection/transformation) |
| Activity partition (swimlane) | Container rectangle subdividing the activity | Solid | Partition name in a header cell (top-left or side edge); partitions may nest (subpartitions); industry layouts are vertical columns or horizontal rows (§15.6.4) |
| Interruptible activity region | Rounded-corner container around protected nodes | Solid | Edges that leave the region carry a **lightning-bolt (zigzag)** adornment at the region boundary (interrupting edges) |
| Expansion region | Rounded rectangle + zigzag **expansion node** adornment on its border | Solid | Mode keyword `«iterative»`, `«parallel»`, or `«stream»` |
| Exception handler | Edge from handler action to protected node boundary | Solid | Lightning-bolt adornment where it exits an interruptible region |
| Variable | Text label attached to activity | — | Shown as labeled element |

## 4. Sequence diagram

*Spec notation:* UML 2.5.1 §17.2.4 (Interaction frames, ExecutionSpecification), §17.3.4
(Lifelines), §17.4.4 (Messages, Gates, Destruction), §17.6.4 (Combined Fragments), §17.7.4
(InteractionUse), §17.8 (Sequence Diagrams).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Interaction frame | Rectangle with corner **pentagon**: `sd <name>` | Solid | Same pentagon convention for all interaction diagram kinds |
| Lifeline | Rectangle "head" + **dashed vertical line** | Dashed | Head usually white rectangle; label `[<name>] [: <Type>]`, decomposition `ref <interaction> [strict]`, or literal `self`. Head may take the shape of the representing classifier (e.g., actor stick figure above the lifeline) |
| Execution specification (activation bar) | Thin rectangle over the lifeline | Spec wording: "thin (**gray or white**) rectangle" — the only color-adjacent hint in the whole notation | Overlapping executions = overlapping rectangles; alternative wide labeled rectangle allowed |
| Synchronous message (`synchCall`) | Horizontal/downward line, **solid**, **filled triangle** head | Solid/filled | Label = operation/args per BNF (§17.4.4) |
| Asynchronous message (`asynchCall` / `asynchSignal`) | Solid line, **open arrowhead** | Solid/open | May overtake other messages |
| Reply message | **Dashed** line, open arrowhead | Dashed/open | Label may include `target = value`; may be omitted when obvious |
| Create message | **Dashed** line, open arrowhead | Dashed/open | Targets the new lifeline head |
| Delete message | Solid line, open arrowhead | — | Ends at a destruction occurrence |
| Destruction occurrence | **×** at the bottom end of the lifeline | Solid × | Lifeline ends at the × |
| Lost message | Solid line ending in a **small filled dot** | Filled dot | Receiver unknown |
| Found message | Solid line starting from a **small filled dot** | Filled dot | Sender unknown |
| Self-message | Arrow looping from the lifeline back to itself | Per message sort | Source and target occurrence on the same lifeline |
| Combined fragment | Rectangle frame with **pentagon operator** (`alt`, `opt`, `break`, `loop`, `par`, `strict`, `seq`, `ignore`, `consider`, `assert`, `neg`) | Solid frame | Operands separated by **dashed horizontal lines**; guard `[condition]` at top-left of each operand; frames nest |
| Interaction use (`ref`) | Rectangle labeled `ref <interaction-name>` | Solid | Actual gates = dots on frame border; return value arrow optional |
| Gate | Small dot on a frame boundary | — | Formal gates (interaction), actual gates (interaction use), inner/outer gates (combined fragments) |
| State invariant | `{constraint}` text on the lifeline | — | May also render as attached note |
| Duration constraint | Braced constraint between two points on a message | — | `{duration …}` |
| Time constraint | Braced constraint from a single point | — | `{time …}` |
| Coregion | Parallel lifeline segment marker | — | Events not locally ordered |
| General ordering | Dashed arrow between occurrences | Dashed | Optional explicit ordering (rarely drawn) |

**Direction rule:** message lines MUST be horizontal or point downward when read from send to
receive (time increases down the page; no global time scale is assumed). **[NOTE-SEQ-001] [NOTE-SEQ-002]**

## 5. State Machine diagram

*Spec notation:* UML 2.5.1 §14.2.4 — 14.2.4.1 (diagram), 14.2.4.3 (Region), 14.2.4.4 (State),
14.2.4.5 (FinalState), 14.2.4.6 (Pseudostates), 14.2.4.7 (ConnectionPointReference), 14.2.4.8
(Transition), 14.2.4.9 (TransitionKind).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| State | **Rounded rectangle** | Solid border | Compartments: name; internal transitions (`entry/`, `exit/`, `do/`); optional behavior/decomposition. State invariant `{…}` may be shown |
| Composite state | Rounded rectangle containing regions | Regions separated by **dashed** lines | Regions side by side or stacked |
| Submachine state | State symbol | — | Name string references the submachine; `ref` syntax |
| Initial pseudostate | **Filled black circle** | Filled | At most one per region; outgoing transition has no trigger/guard |
| Final state | **Circle enclosing a filled black dot** ("bullseye") | Filled dot, ring | Region completion |
| Transition | Solid arrow between vertices | Solid | Label: `<trigger> {',' <trigger>}* ['[' <guard> ']'] ['/' <effect>]`; time events `after (t)` / `when (t)`; internal transitions are listed in the state compartment, **never drawn** (§14.2.4.9) |
| Choice pseudostate | Small **filled circle** | Filled | Dynamic branch (guards evaluated on arrival) |
| Junction pseudostate | Small **filled circle** | Filled | Same shape as choice; static branch; semantic distinction only |
| Fork / join pseudostate | **Thick bar** | Solid, heavy weight | Concurrency entry/sync |
| Entry point | Small circle **on the state/SM boundary** | Solid | Named connection point |
| Exit point | Small circle with **×** inside, on the boundary | Solid | Named connection point |
| Shallow history | Circle with **H** | Solid | |
| Deep history | Circle with **H\*** | Solid | |
| Terminate pseudostate | **×** | Solid | Execution ends immediately |
| Deferred trigger | Trigger name + `/ defer` inside state compartment | — | §14.2.4.8.6 |
| State list notation | Compact list form | — | Graphical shortcut (§14.2.4.4.3) |
| `«statemachine»` classifier | Rectangle with keyword | — | Showing a StateMachine in class-diagram contexts (§14.2.4.2) |
| Action symbols on transitions | Rectangle (action), concave pentagon (signal receipt), convex pentagon (signal send), filled dot (choice), dot (merge) | — | §14.2.4.8.1–5 |

## 6. Communication diagram

*Spec notation:* UML 2.5.1 §17.9 (Communication Diagrams), with message graphics per §17.4.4.

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Lifeline | Classifier symbol: rectangle (`name : Type`), stick figure for actors, or any classifier icon | Solid | **No time axis** (unlike sequence diagrams); `self` allowed |
| Link | Solid line between lifelines | Solid | Instance of an association; one path per communicating pair |
| Message | Arrowhead drawn along the link, labeled with a **sequence expression** | Arrow style per §17.4.4 (filled/open) | Sequence expression: sequence number (`1`, `1.1`, `2a`), letter suffixes denote concurrent messages, iteration `*` or `[n…]`, then `:` message name and arguments (BNF in §17.9) |
| Sequence numbering | Text labels | — | Order, not geometry, carries time — Smith renders numbers, not vertical offsets |

## 7. Component diagram

*Spec notation:* UML 2.5.1 §11.6.4 (Components), §11.3.4 (Ports), §10.4.4 (ball-and-socket),
§22.3 (`«subsystem»`).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Component | Classifier rectangle with `«component»` keyword, optional **component icon** (rectangle with two small protruding tabs) in the top-right corner; or icon-only notation | Solid | May contain compartments (internal structure, operations) |
| Subsystem | Component with `«subsystem»` | Solid | Standard Profile §22.3 |
| Provided interface | **Lollipop**: circle on a stem attached to the component (or to a port) | Solid | Interface name near the circle |
| Required interface | **Socket**: half-circle cup attached to the component/port | Solid | Interface name near the cup |
| Port | Small square on the component boundary | Solid | Name/type nearby; conjugated type prefixed `~` (§11.3.4) |
| Interface dependency | **Dashed** arrow, open head | Dashed/open | Typically from requiring classifier/socket side to providing classifier/lollipop side |
| Component realization | **Dashed** line + hollow triangle | Dashed/hollow | Realizing classifier → component |
| Assembly/delegation connectors | Solid connectors; ball-and-socket join optional | Solid | §11.6.4/§11.2.4 |

## 8. Composite Structure diagram

*Spec notation:* UML 2.5.1 §11.2.4 (StructuredClassifiers — parts, roles, connectors),
§11.3.4 (Ports), §11.7.4 (Collaborations).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Structured classifier frame | Classifier rectangle | Solid | The `internal structure` compartment is required when shown this way (§11.2.4) |
| Part (composite role) | Box nested inside the structure compartment, **solid outline** | Solid | Label `name : Type`; multiplicity may be shown at top-right corner |
| Non-composite role | Box with **dashed outline** | Dashed | Referenced (not owned) participant |
| Port on part/classifier | Small square overlapping the part/classifier boundary | Solid | Lollipop/socket may attach to ports (§11.3.4) |
| Connector | Solid line between parts/roles/ports | Solid | Label `[<name>] [':' <type>]`; end adornments use association-end notation; default multiplicity derives from the role |
| Ball-and-socket join | Lollipop+socket symbol joining ports | Solid | Optional alternative to plain connectors (§11.2.4); channeled form for n-ary |
| Collaboration | Rectangle with `«collaboration»`, or **dashed ellipse** | Dashed (ellipse form) | §11.7.4 |
| Collaboration use | Dashed ellipse labeled with role bindings | Dashed | `use in <collaboration>` style labels |

## 9. Deployment diagram

*Spec notation:* UML 2.5.1 §19.2.4 (Deployments), §19.3.4 (Artifacts), §19.4.4 (Nodes).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Node | **Three-dimensional box** (perspective) | Solid edges | Devices vs execution environments distinguished by `«device»` / `«executionEnvironment»` keywords; nodes nest |
| Artifact | Rectangle with `«artifact»` keyword, or artifact **icon** (rectangle with folded corner) | Solid | May show nested artifacts |
| Deployment relationship | **Dashed** arrow with `«deploy»` stereotype | Dashed/open | From artifact (or component) to deployment target |
| Deployment specification | Dashed arrow `«deploy»` + spec | Dashed | Parameterizes deployment |
| Manifestation | **Dashed** arrow with `«manifest»` | Dashed | Artifact manifests (implements) a component/classifier |
| Communication path | Solid line between nodes | Solid | Association between nodes |
| Deployed component/artifact | Component/artifact symbol drawn inside a node | Solid | Nested containment rendering |

## 10. Package diagram

*Spec notation:* UML 2.5.1 §12.2.4 (Packages), §7.7 (Dependencies), §7.4 (ElementImport).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Package | **Tabbed folder**: large rectangle with a small tab rectangle at the top-left | Solid | Name in the tab or centered in the body |
| Package as container | Large rectangle listing/containing elements | Solid | Name top-left; nested packages drawn inside |
| `«model»` | Package with keyword | Solid | Auxiliary construct (§12.2) |
| Package merge | Arrow with `«merge»` | Dashed/open (dependency) | From receiving to merged package |
| Package import | Arrow with `«import»` | Dashed/open | |
| Element import | Arrow with `«access»` or public/private markers | Dashed/open | §7.4; circle-plus namespace notation for owned members |
| Dependency | Dashed arrow, open head | Dashed/open | Between packages |

## 11. Object diagram

*Spec notation:* UML 2.5.1 §9.8 (Instances; notation §9.8.4), §11.5.4 (links follow association
adornment rules without classifier-level features).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Instance specification | Rectangle | Solid | Name **underlined**: `instanceName : TypeName`; name, type, or both may be omitted [NOTE-CORE-006] |
| Slot | Entry in a `slots`/attributes compartment | — | `attribute = value` with value specification |
| Link (association instance) | Solid line between instances | Solid | Instance of an association; multiplicities may be shown; directed links may add an open arrowhead |
| N-ary link | Diamond form | Solid | Mirrors n-ary association notation |

## 12. Timing diagram

*Spec notation:* UML 2.5.1 §17.11 (Timing Diagrams — 17.11.1 Notation). Optional diagram kind for
conforming tools; Smith implements it.

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Lifeline | Row with the instance name on the left and a **horizontal time axis** | Solid | Time increases left→right (transposed relative to sequence diagrams) |
| State/condition timeline | Stepped horizontal segments, one per lifeline | Solid | Each segment labeled with the state or condition name; vertical steps at state changes |
| Message | Arrow between lifelines, drawn at the appropriate time position | Per §17.4.4 arrowheads | May carry time/duration constraints |
| Time constraint | Brace from a point, labeled | Solid brace | `{time …}` |
| Duration constraint | Brace between two points, labeled | Solid brace | `{duration …}` |
| Destruction event | **×** on the timeline | Solid | Object destruction |

## 13. Interaction Overview diagram

*Spec notation:* UML 2.5.1 §17.10 (Interaction Overview Diagrams); control-node graphics per
§15.3.4; frame graphics per §17.2.4.

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Overview frame | Interaction frame with pentagon `interaction overview`/`sd` | Solid | |
| Interaction occurrence | Rectangle with pentagon `ref <interaction-name>` (InteractionUse notation) | Solid | Reference to another interaction |
| Inline interaction | Frame with pentagon `sd <name>` embedded as a node | Solid | |
| Initial node | Filled black circle | Filled | Activity-diagram convention |
| Final nodes | Circle with × | Solid | Activity-diagram convention |
| Decision / merge | Diamond | Solid | Activity-diagram convention; guards on outgoing edges |
| Fork / join | Thick bar | Solid | Activity-diagram convention |
| Flow edge | Solid arrow | Solid | **Not** an activity diagram: nodes are interactions, not actions — do not reinterpret |
| Time/duration constraints | Braced constraints on nodes/edges | — | Allowed per §17.10 |

## 14. Profile diagram

*Spec notation:* UML 2.5.1 §12.3.4 (Profiles notation), §22.3 (Standard stereotypes).

| Element | Shape | Fill/Stroke convention | Notes |
|---|---|---|---|
| Profile | Tabbed-folder package symbol (may carry `«profile»`) | Solid | Contains stereotypes, metaclass references, nested profiles |
| Stereotype | Classifier rectangle with `«stereotype»` keyword | Solid | Attributes compartment lists tagged values (properties); operations for operations; may generalize other stereotypes |
| Metaclass | Rectangle with `«metaclass»` keyword | Solid | Reference to an extended UML metaclass |
| Extension | Solid line with **filled arrowhead** pointing at the metaclass | Solid/filled head | From stereotype to metaclass; required for stereotypes that add structure |
| Generalization between stereotypes | Solid line + hollow triangle | Hollow triangle | Standard generalization graphic |
| Reference (dependency) | Dashed arrow `«reference»` | Dashed/open | Stereotype referencing other classifiers |
| Profile application | Dashed arrow with `«apply»` | Dashed/open | From applying package to applied profile |
| Standard profile stereotypes | Keywords only | — | `«Create»`, `«Call»`, `«Send»`, `«derive»`, `«trace»`, `«refine»`, `«use»`, `«friend»`, `«instantiate»`, `«responsibility»`, `«implementationClass»`, `«type»`, `«utility»`, `«focus»`, `«framework»`, `«modelLibrary»`, `«process»`, `«thread»`, `«specification»`, `«realization»`, `«auxiliary»`, `«access»`, `«import»`, `«subsystem»`, `«metaclass»` (§22.3) |

---

## Smith extension notation (requirements, test cases, traceability)

Not part of OMG UML — Smith defines these as profile extensions (SysML-inspired), rendered with
the **same graphic grammar** above so they compose with standard notation:

- **`«requirement»`** — classifier rectangle with `«requirement»` keyword and a values compartment
  (`id`, `text`, optional `status`/`priority`), mirroring SysML's requirement box.
- **`«testCase»`** — classifier rectangle with `«testCase»` keyword (steps/expected results in
  compartments).
- **Trace relations** — rendered as the UML dependency graphic (dashed line, open arrowhead)
  with trace-family stereotype labels (`«trace»`, `«derive»`, `«verify»`, `«satisfy»`-family as
  defined in `../architecture/04-traceability-relations.md`), keeping the chain
  Requirement → UseCase → Activity → Sequence → Class visually uniform with UML dependencies.
- Because these are stereotypes, **icon alternatives** (profile-defined icons above the name)
  are legitimate per §12.3.4 — a future product option, not required.

## Notation fixed by the spec vs. open to Smith

**Fixed (MUST preserve — conformance):**

- Element shape topology per the tables above (rectangle vs ellipse vs rounded rectangle vs diamond vs bar vs stick figure vs 3D box vs tabbed folder vs lollipop/socket vs bullseye vs filled dot vs ×).
- Line style semantics: solid vs dashed distinguishes relationship *kinds* (e.g., generalization vs realization, association vs dependency).
- Arrowhead semantics: filled vs open vs hollow-triangle vs diamond vs dot vs × (taxonomy table).
- Compartment order and required compartments (e.g., internal-structure compartment in composite structure notation; class name on top).
- Label syntax: BNF grammar for feature strings, message labels, transition labels, lifeline identifiers, sequence expressions.
- Guillemet keywords `«…»` above names; visibility prefixes; multiplicity syntax; italics = abstract; underline = static / instance names; `/` = derived.
- Message direction rule (horizontal or downward only).
- Subject-rectangle-is-not-ownership rule (use case diagrams).

**Open (product decision — design system / renderer owns it):**

- **All color and fill.** The spec is black-and-white; the only color-adjacent wording is ExecutionSpecification "gray or white" (§17.2.4.4). Smith's palette is unconstrained [NOTE-CORE-002].
- Fonts, font sizes, line weights, dash patterns' exact metrics, corner radius, icon art style, node 3D perspective depth, stick-figure proportions — within the fixed topology.
- Default compartment suppression policy, zoom-dependent detail levels (explicitly permitted, §9.2.4).
- Routing style (straight, orthogonal/elbow, curved), grid, snapping, spacing, margins, diagram frame chrome.
- Presentation options the spec marks optional (dot ownership notation, semicircular crossing jog, merged aggregation trees, ball-and-socket) — if Smith supports them, it MUST apply each consistently within a diagram (spec requirement for dot notation).

**[NOTE-CORE-002]** restated for the design system: because color carries no UML meaning, Smith
may color by *layer* (structure vs behavior), by *traceability chain*, or by *element category*
without any standards conflict — provided meaning never depends on color alone.

## Layout conventions (industry standard, not spec-mandated)

**[NOTE-LAYOUT-001]** None of the following is normative in UML 2.5.1; all are universal
industry practice and SHOULD be Smith's auto-layout and snap defaults:

- **Class diagrams:** inheritance top-down (superclass above, subclass below, arrows pointing up); dependencies flow left→right or top→down; grid-aligned boxes; association labels placed mid-line; related classes clustered; minimize edge crossings.
- **Use case diagrams:** subject boundary centered; primary actors left, secondary/system actors right; actors outside the boundary; included use cases toward one side for readability.
- **Activity diagrams:** main flow top→down (or left→right); swimlanes as **vertical columns** (most common) or horizontal rows; decisions branch horizontally; merges realign.
- **Sequence diagrams:** lifelines ordered left→right in order of first interaction; time strictly downward; activation bars aligned on lifelines; fragments snug around contents.
- **State machines:** initial pseudostate top-left; primary flow left→right / top→down; history/entry/exit points on boundaries facing connected elements.
- **Deployment diagrams:** nodes layered top-down by tier (e.g., client → server → database); communication paths orthogonal.
- **Package diagrams:** ownership hierarchy top-down or nested containment; imports drawn between sibling positions.
- General: orthogonal (elbow) connectors for structure diagrams, straight/curved for interactions; consistent edge routing per diagram kind; collision-free labels.

## Stereotype notation

- Applied stereotype: `«name»` **above the element name**; multiple keywords comma-separated or stacked (§7/§9.2.4 convention, used throughout).
- Defining stereotype (profile diagrams): class symbol with `«stereotype»` keyword; extension arrow to metaclass.
- Profile-defined **icons** may replace the keyword (small graphic above the name) — §12.3.4 permits; Smith can expose per-profile icons later.
- Stereotyped dependency: keyword labels the dashed arrow (`«include»`, `«extend»`, `«deploy»`, `«trace»`, …).
- Standard-profile keywords are reserved spellings (§22.3); Smith extensions use distinct names (`requirement`, `testCase`, trace family).
- On subject rectangles (use case diagrams), the subject's stereotype keyword MUST appear in guillemets above the name (§18.1.4).

## Color guidance for aesthetics (reference only — palette is the design system's job)

Smith's palette is defined in `../ui-ux/01-design-system.md`. This section only names the
sources the design system should harmonize with and the hard constraints:

- **WCAG 2.1** — SC 1.4.3: text contrast ≥ 4.5:1 (≥ 3:1 for large text); SC 1.4.11: non-text
  graphical-object contrast ≥ 3:1 (borders/strokes that convey element identity); SC 1.4.1:
  never encode meaning by color alone — UML already helps here because line style and
  arrowheads are shape-based. [NOTE-A11Y-001]
- **Material 3 color system** — tonal palettes, color roles (surface/container/accent),
  dynamic color, dark-theme derivation: a proven model for a large, calm surface (the canvas)
  with small saturated accents.
- **Apple Human Interface Guidelines (Color)** — semantic colors, automatic light/dark
  appearance, vibrancy/layering: relevant for a native-feeling desktop app.
- **Okabe–Ito color-safe palette** — a categorical palette distinguishable under common
  color-vision deficiencies; a good basis for element-category and traceability-chain hues.
- Dark mode is a first-class canvas variant; notation strokes must keep ≥ 3:1 against both
  canvas variants.

## Sources

- OMG, *Unified Modeling Language*, Version 2.5.1, formal/17-12-05 — https://www.omg.org/spec/UML/2.5.1/PDF — primary notation authority; clause numbering per the table above; Annex A diagram taxonomy, Annex B UMLDI.
- OMG, *UML Diagram Interchange* 1.0, formal/06-04-04 — https://www.omg.org/spec/UMLDI/1.0/PDF — DI metamodel folded into UML 2.5.1 Annex B.
- OMG, UML 2.5.1 Diagram Interchange metamodel XMI, ptc/18-01-04 — https://www.omg.org/spec/UML/20161101/UMLDI.xmi
- OMG OCUP 2 coverage maps (verify UML 2.5.1 clause numbering) — http://www.omg.org/ocup-2/coveragemap-intermed.htm , http://www.omg.org/ocup-2/coveragemap-advanced.htm
- OMG UML issue tracker (clause cross-verification) — https://issues.omg.org/issues/spec/UML
- Kirill Fakhroutdinov, *UML 2.5 Diagrams Overview* — https://www.uml-diagrams.org/uml-25-diagrams.html — secondary community reference with notation summaries.
- W3C, *Web Content Accessibility Guidelines (WCAG) 2.1* — https://www.w3.org/TR/WCAG21/
- Material 3 color system — https://m3.material.io/styles/color/overview
- Apple Human Interface Guidelines, Color — https://developer.apple.com/design/human-interface-guidelines/color
- Okabe, M. & Ito, K., *Color Universal Design* (color-safe categorical palette) — https://jfly.uni-koeln.de/color/
