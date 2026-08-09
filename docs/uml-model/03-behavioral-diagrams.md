---
status: DRAFT
---

# UML Model 03 — Behavioral Diagrams

The behavioral UML diagram kinds Smith supports. Same structure as
[02-structural-diagrams.md](./02-structural-diagrams.md): per-kind element/relationship
contracts. Notation fixed by UML — see
[research/04-uml-visual-notation.md](../research/04-uml-visual-notation.md).

Core metamodel: [uml-model/01-metamodel-core.md](./01-metamodel-core.md). Diagram-as-view:
[uml-model/06-diagram-interchange.md](./06-diagram-interchange.md). Traceability chain:
[uml-model/05-traceability-chain.md](./05-traceability-chain.md) — behavioral diagrams are
the MIDDLE of the canonical chain (UseCase → Activity → Sequence).

`[REQ-BEHAV-001]` Same constraint rule: a diagram's `kind` determines allowed element/edge
kinds (REQ-STRUC-001 applies here too).

## 1. Use Case diagram (`kind = "useCase"`)

OMG UML 2.5.1 §18. Captures system behavior from an actor-goal perspective. This is the
**first behavioral level** in the canonical chain.

**Allowed shapes:** `UseCase`, `Actor`, `Subject` (system boundary rectangle), `Requirement`, `TestCase`, `Comment`.
**Allowed edges:**
- `Association` (Actor — UseCase)
- `«include»` (UseCase → UseCase; dependency stereotype)
- `«extend»` (UseCase → UseCase; dependency stereotype)
- `Generalization` (Actor → Actor, UseCase → UseCase)
- `«satisfy»` (UseCase → Requirement) — trace relation, shown as dashed dependency

Requirements and TestCases appear on Use Case/Activity diagrams to visualize traceability
relations; they are not behavioral elements.

`[REQ-BEHAV-002]` UseCases render as ellipses; actors as stick figures — per
[research/04](../research/04-uml-visual-notation.md) §2. The Subject rectangle is a visual
grouping; it does NOT imply ownership (the model ownership tree owns the use cases).

`[REQ-BEHAV-003]` `«include»`/`«extend»` are modeled as Dependencies with the corresponding
stereotype; their direction follows UML semantics (include: base → included; extend:
extending → extended).

## 2. Activity diagram (`kind = "activity"`)

OMG UML 2.5.1 §15–16. Flow of actions/control/data. Second level of the canonical chain
(realizes UseCases).

**Allowed shapes:**
- `Activity` (frame with corner pentagon `activity <name>`)
- `Action` (rounded rectangle)
- Control nodes: `InitialNode`, `ActivityFinalNode`, `FlowFinalNode`, `DecisionNode`,
  `MergeNode`, `ForkNode`, `JoinNode`
- Object nodes: `ObjectNode`, `DataStore`, `CentralBuffer`
- `ActivityPartition` (swimlane container)
- `InterruptibleActivityRegion`, `ExpansionRegion`
- `Pin` (on actions), `Comment`
- `Requirement`, `TestCase` (traceability annotations — not behavioral elements)

**Allowed edges:**
- `ControlFlow` (Action → Action)
- `ObjectFlow` (Action/ObjectNode → Action/ObjectNode)
- `ExceptionFlow` (with lightning-bolt adornment)
- `«realize»` (Activity → UseCase) — trace relation

`[REQ-BEHAV-004]` Activity shapes follow [research/04](../research/04-uml-visual-notation.md)
§3 exactly: rounded rectangles for actions, diamonds for decision/merge, thick bars for
fork/join, filled circle for initial, bullseye for final.

`[REQ-BEHAV-005]` Swimlanes (ActivityPartitions) are container shapes on the diagram; nodes
inside a lane are visually nested but their model ownership is the Activity (not the
partition). Partition assignment is a model property of each node.

## 3. Sequence diagram (`kind = "sequence"`)

OMG UML 2.5.1 §17.2–17.8. Time-ordered message exchange between lifelines. Third level
(realizes Activities).

**Allowed shapes:**
- `Interaction` (frame with pentagon `sd <name>`)
- `Lifeline` (head + dashed vertical line)
- `ExecutionSpecification` (activation bar)
- `CombinedFragment` (alt/opt/loop/par/etc.)
- `InteractionUse` (`ref`)
- `Gate` (formal/actual)
- `DestructionOccurrence`
- `StateInvariant`, `Comment`

**Allowed edges:**
- `Message` (kinds: `synchCall`, `asynchCall`, `asynchSignal`, `reply`, `createMessage`,
  `deleteMessage`, `lost`, `found`)
- `«realize»` (Interaction → Activity; Interaction → Class/Operation) — trace relation

`[REQ-BEHAV-006]` Message direction MUST be horizontal or downward (time increases down) —
[research/04](../research/04-uml-visual-notation.md) NOTE-SEQ-001. Vertical position encodes
time order; the renderer enforces monotonic time-top-to-bottom.

`[REQ-BEHAV-007]` Message sorts render with distinct arrowheads per
[research/04](../research/04-uml-visual-notation.md) §4: sync = filled triangle, async = open
arrowhead, reply = dashed + open, create = dashed + open (target new lifeline), delete ends
at a destruction ×.

`[REQ-BEHAV-008]` Combined fragment operators (`alt`, `opt`, `loop`, `par`, etc.) are a
property of the CombinedFragment shape; operands separated by dashed horizontal dividers;
guards `[condition]` per operand.

## 4. State Machine diagram (`kind = "stateMachine"`)

OMG UML 2.5.1 §14. State transitions of a classifier.

**Allowed shapes:**
- `StateMachine` (frame with pentagon `stm <name>`)
- `State` (rounded rectangle), `CompositeState`, `SubmachineState`
- `Region` (inside composite states)
- Pseudostates: `Initial`, `Choice`, `Junction`, `Fork`, `Join`, `DeepHistory`, `ShallowHistory`,
  `Terminate`, `EntryPoint`, `ExitPoint`
- `FinalState` (bullseye)
- `ConnectionPointReference`
- `Comment`

**Allowed edges:**
- `Transition` (State/Pseudostate → State/Pseudostate; label
  `<trigger> ['['<guard>']'] ['/'<effect>]`)
- Internal transitions: NOT drawn (listed in the state's internal-transitions compartment)

`[REQ-BEHAV-009]` States render as rounded rectangles; pseudostates per
[research/04](../research/04-uml-visual-notation.md) §5. Initial pseudostate is the only
entry into the machine (single per region).

`[REQ-BEHAV-010]` Composite states contain regions; regions separated by dashed lines. A
submachine state references another StateMachine by name (`ref` notation).

## 5. Communication diagram (`kind = "communication"`)

OMG UML 2.5.1 §17.9. Object interactions with sequence-numbered messages (no time axis).

**Allowed shapes:** `Lifeline` (as object spec, no dashed line), `Comment`.
**Allowed edges:** `Link` (instance of association), `Message` (drawn along the link with a
sequence expression `1.1: message(args)`).

`[REQ-BEHAV-011]` Communication diagrams use sequence numbers (not vertical position) for
ordering — per [research/04](../research/04-uml-visual-notation.md) §6.

## 6. Timing diagram (`kind = "timing"`)

OMG UML 2.5.1 §17.11. State/condition over time, with time on the X-axis.

**Allowed shapes:** `Lifeline` (with horizontal time axis), `State/Condition timeline`,
`DestructionOccurrence`, `Comment`.
**Allowed edges:** `Message` (between lifelines at time positions), time/duration constraints.

`[REQ-BEHAV-012]` Time increases left→right (transposed vs. sequence) —
[research/04](../research/04-uml-visual-notation.md) §12.

## 7. Interaction Overview diagram (`kind = "interactionOverview"`)

OMG UML 2.5.1 §17.10. An activity-diagram-like overview whose nodes are interactions (refs).

**Allowed shapes:** `InteractionUse` (ref nodes), inline `Interaction` frames, control nodes
(initial, final, decision, merge, fork, join) — reused from activity notation.
**Allowed edges:** `ControlFlow` (between interaction nodes; NOT an activity — nodes are
interactions, not actions).

`[REQ-BEHAV-013]` Interaction Overview diagrams MUST visually distinguish themselves from
activity diagrams (per [research/04](../research/04-uml-visual-notation.md) §13): the frame
pentagon reads `interaction overview` / `sd`; nodes are `ref`/`sd` rectangles, not rounded
action rectangles.

## Common constraints (all behavioral diagrams)

`[REQ-BEHAV-014]` Same as REQ-STRUC-011/012/013: elements exist in the model first; shape
deletion removes only the view; bounds are view data, semantics are model data.

`[REQ-BEHAV-015]` Behavioral elements (Activities, Interactions) are model elements with
ownership — they live in packages and appear in the model explorer. A "sequence diagram" in
Smith is a diagram (`kind=sequence`) whose view references an `Interaction` element + its
lifelines/messages.

## Traceability participation

`[REQ-BEHAV-016]` Behavioral diagrams are the **middle of the canonical chain**:
- UseCase diagrams show `«satisfy»` (UseCase → Requirement) — level 1.
- Activity diagrams show `«realize»` (Activity → UseCase) — level 2.
- Sequence diagrams show `«realize»` (Interaction → Activity, Interaction → Class) — level 3.

These trace edges are shown on behavioral diagrams as dashed dependency arrows with stereotype
labels when the "show traceability edges" view setting is on (off by default; see
[uml-model/05-traceability-chain.md](./05-traceability-chain.md) §Diagram participation).

## Open questions (resolve before STABLE)

- [ ] Timing diagram: full support in v1? It's optional for conforming tools. Tentative: yes,
      basic (lifeline + state timeline + message), advanced constraints later.
- [ ] Communication & Interaction Overview diagrams: full v1 or defer? Tentative: v1 supports
      all 14 (per vision G2), but Communication/Interaction Overview get less UX polish than
      the core 5 (class, use case, activity, sequence, state).
- [ ] Activity diagram swimlane orientation: vertical columns default (industry standard per
      [research/04](../research/04-uml-visual-notation.md) §Layout) — confirm with a mock.
