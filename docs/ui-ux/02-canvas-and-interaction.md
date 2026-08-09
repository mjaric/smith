---
status: DRAFT
---

# UI/UX 02 — Canvas & Interaction

The canvas is where Smith lives or dies as a tool. This document specifies the interaction
model: tools, selection, the **context-sensitive gizmo** (the user's signature ask),
connection creation, editing, navigation. Notation (shapes/lines/arrowheads) is fixed by UML
— see [research/04-uml-visual-notation.md](../research/04-uml-visual-notation.md). Design
tokens from [ui-ux/01-design-system.md](./01-design-system.md).

Rendering stack: Konva.js on Canvas 2D, inside Tauri + React — see
[research/06-canvas-and-frontend-stack.md](../research/06-canvas-and-frontend-stack.md).

## Tool model

`[REQ-UI-001]` Smith has a **tool palette** with these tools (keyboard shortcuts in parens):

| Tool | Shortcut | Purpose |
|------|----------|---------|
| **Select** (`V`) | default | Select, move, resize, open gizmo. |
| **Marquee** (`M`) | | Drag a rectangle to select multiple. (Sub-mode of Select via drag on empty canvas.) |
| **Create [kind]** (`C`, `U`, `A`, `S`, `R`, `T`...) | | Create a specific element kind on click. One per element kind; the palette groups them by the active diagram kind. |
| **Connect** (`L`) | | Drag from one element to another to create a relationship; the kind is chosen on drop. |
| **Pan** (`Space`-hold or middle-mouse) | | Pan the canvas. |
| **Zoom** (`Z` + scroll, or pinch) | | Zoom. |

`[REQ-UI-002]` The active tool is a single global state. Pressing `Esc` returns to Select.
Pressing a one-key shortcut switches tools; pressing it again also returns to Select (toggle).

`[REQ-UI-003]` The palette shows ONLY the element kinds valid for the current diagram kind
(see [uml-model/02-structural-diagrams.md](../uml-model/02-structural-diagrams.md),
[03-behavioral-diagrams.md](../uml-model/03-behavioral-diagrams.md)). E.g. on a Sequence
diagram, the palette offers Lifeline, Message, Combined Fragment, Interaction Use — not
Class.

## Selection model (ownership-aware)

`[REQ-UI-004]` Selection is a **set** of view references (shapes/edges) on the current
diagram, plus optionally the underlying model elements (for the inspector).

`[REQ-UI-005]` Click on a shape → selects that shape (single selection; replaces previous).
Shift-click → adds/toggles. Click on empty canvas → clears selection AND opens the gizmo
(§Gizmo). Drag on empty canvas → marquee select.

`[REQ-UI-006]` **Ownership-aware selection** (user's explicit concern — elements belong to
the model tree, not the diagram): double-clicking a shape that represents a composite
element (a Package, a Component, a composite State, an Activity) navigates INTO it:
- Package/Component → opens the first diagram inside it, or offers to create one.
- Composite State → focuses the sub-state-machine.
- Activity → opens the activity's diagram.
This navigates the **ownership tree**, not the diagram — reinforcing that the model is the
truth and the diagram is a view.

`[REQ-UI-007]` Right-click on a selection opens a **context menu** with actions valid for the
selection kind (rename, delete, apply stereotype, trace forward/backward, show in explorer,
hide on this diagram). Actions go through the model API (P3).

## The context-sensitive gizmo (signature feature)

This is the user's core ask: *"if the select tool is active, clicking empty canvas opens a
gizmo at the click point offering context-appropriate options."* Smith makes this the primary
creation affordance.

### Trigger
`[REQ-UI-GIZMO-001]` When the **Select tool** is active and the user **clicks empty canvas**
(no shape under the cursor), a **gizmo** appears centered at the click point (in canvas
coordinates, so it stays anchored during pan/zoom until dismissed).

`[REQ-UI-GIZMO-002]` The gizmo does NOT appear if:
- A drag occurs (that's a marquee).
- Another tool is active (creation tools place their element directly).
- The click is on a shape/edge (that's selection).

### Layout — radial menu
`[REQ-UI-GIZMO-003]` The gizmo is a **radial (pie) menu** with a center hub and 4–8 slots
around it. Rationale: radial menus minimize cursor travel (Fitts's law), are faster than
linear menus for ≤8 items, and feel natural for spatial "place something here" intent.

```
        ┌─────┐
        │  A  │
   ┌────┼─────┼────┐
   │ B  │  ◉  │ C  │      ◉ = center (dismiss / recent)
   └────┼─────┼────┘
        │  D  │
        └─────┘
   (4-slot example; expands to 6–8 with submenus)
```

`[REQ-UI-GIZMO-004]` Slots are **context-determined** by the current diagram kind and the
click location (e.g. clicking inside a swimlane on an activity diagram offers actions scoped
to that lane). The slot set is the **most likely next action** given context.

### Slot content per diagram kind
`[REQ-UI-GIZMO-005]` Default slot allocation (the top actions for each diagram kind). Each
slot creates an element at the click location AND enters inline rename. One slot is reserved
for "More…" → expands a secondary ring or a small linear menu with the remaining kinds.

| Diagram kind | Slots (radial) |
|--------------|----------------|
| **Class** | Class · Interface · Enumeration · Association (then drag) · Note · More… |
| **Use Case** | Use Case · Actor · Subject boundary · «include» · «extend» · More… |
| **Activity** | Action · Decision · Fork/Join · Initial · Final · Swimlane · More… |
| **Sequence** | Lifeline · Message (sync) · Combined Fragment · Interaction Use · Note · More… |
| **State Machine** | State · Initial · Final · Transition · History · More… |
| **Component** | Component · Interface (lollipop) · Port · Dependency · More… |
| **Package** | Package · Class · Diagram (new) · Import · More… |
| **Requirement** | Requirement · TestCase · «satisfy» · «verify» · «deriveReqt» · More… |

`[REQ-UI-GIZMO-006]` The "More…" slot reveals the full element-kind list for the diagram,
plus structural actions (add comment, add constraint, paste).

### Center hub
`[REQ-UI-GIZMO-007]` The center hub shows the **most recent element kind created** on this
diagram (one-tap repeat). If none, it shows a dismiss icon. Clicking the hub either repeats
the last creation or dismisses the gizmo.

### Interaction
`[REQ-UI-GIZMO-008]` The gizmo accepts both **click** (click a slot) and **gesture**
(press-drag toward a slot, release — "marking menu" style, faster for power users). Drag
distance ≥ 8px toward a slot selects it on release.

`[REQ-UI-GIZMO-009]` Keyboard: while the gizmo is open, number keys `1`–`8` select slots
radially (clockwise from top). `Esc` or clicking the hub dismisses without action.

`[REQ-UI-GIZMO-010]` The gizmo auto-dismisses on: selection of a slot (after action), `Esc`,
click elsewhere, pan/zoom beyond a threshold, or 4 seconds of inactivity (configurable).

### Visual design
`[REQ-UI-GIZMO-011]` Gizmo visual:
- Translucent dark chip on the canvas (works in both light/dark themes), backdrop-blur.
- Slot labels appear on hover; otherwise show icons (per [ui-ux/01](./01-design-system.md)
  iconography).
- Subtle scale-in on open (120ms, `motion.fast`); scale-out on dismiss.
- Does NOT occlude the click point — offset slightly so the created element lands at the
  original click.

`[REQ-UI-GIZMO-012]` The gizmo is rendered ABOVE all canvas elements (top z-layer, per
[ui-ux/01](./01-design-system.md) z-order).

### Accessibility
`[REQ-UI-GIZMO-013]` The gizmo MUST be keyboard-operable: when it opens, focus moves to the
hub; Tab/arrows move between slots; Enter activates; Esc dismisses. The radial layout is
visual only; the keyboard order is deterministic (clockwise from top).

## Connection creation

`[REQ-UI-CONN-001]` Two ways to create a relationship:
1. **Connection handle** — when a shape is selected, a small handle appears on each edge
   (midpoint). Drag from a handle to another element; on drop, a menu offers the valid
   relationship kinds for the source/target element kinds (filtered by diagram kind). This is
   the primary method.
2. **Connect tool (`L`)** — press L, drag from source to target, choose kind on drop. Useful
   when creating many connections in sequence.

`[REQ-UI-CONN-002]` On drop, Smith validates the relationship against the metamodel (source
and target kinds must be legal for the relationship kind; trace relations enforce the
direction and reject cycles — see
[architecture/04-traceability-relations.md](../architecture/04-traceability-relations.md)).
Invalid drops show a brief inline message ("Cannot: A `«verify»` must come from a TestCase")
and do not create the edge.

`[REQ-UI-CONN-003]` New edges get **orthogonal routing** by default (see
[research/06](../research/06-canvas-and-frontend-stack.md) §7). The user may drag bend points
to override; user-placed bends win over auto-routing (P1).

## Inline editing

`[REQ-UI-EDIT-001]` Double-click a shape → inline edit the element name (Enter commits, Esc
cancels). For class compartments, double-click a feature line → edit that feature's
signature in place.

`[REQ-UI-EDIT-002]` For richer editing (multi-field, stereotypes, tagged values), use the
inspector (see [ui-ux/03-inspector-and-explorer.md](./03-inspector-and-explorer.md)).

## Move, resize, snap

`[REQ-UI-MOVE-001]` Dragging a shape moves its view-reference bounds (NOT the model element
— the element has no position; the view does). Multi-select drag moves all selected shapes,
preserving relative offsets.

`[REQ-UI-MOVE-002]` Resize handles (8, on corners/edges of a selected shape) resize the
shape's view bounds. Resizing does NOT change the model (compartments auto-size to content;
the shape width is a view concern).

`[REQ-UI-MOVE-003]` Snapping: shapes snap to the 8px grid by default; connectors snap to
shape edges and ports. Snapping toggled with `Shift` (hold to disable) or a setting. Edge
bend points snap to orthogonal angles.

## Pan & zoom

`[REQ-UI-NAV-001]` Pan: `Space`-drag, middle-mouse drag, or two-finger trackpad scroll.
Zoom: `Cmd/Ctrl` + scroll, pinch, or `+`/`-` keys. `Cmd/Ctrl` + `0` resets to 100% /
fit-to-content; `Shift+0` fits selection.

`[REQ-UI-NAV-002]` Zoom range: 10%–400% (per
[uml-model/06-diagram-interchange.md](../uml-model/06-diagram-interchange.md) REQ-DI-011).
Beyond range, clamp. Text re-rasterizes at every zoom (crisp — see
[ui-ux/01](./01-design-system.md) REQ-DS-010).

`[REQ-UI-NAV-003]` A **minimap** (toggleable, corner of canvas) shows the whole diagram with
the current viewport rectangle, for navigation on large diagrams. Clicking the minimap pans.

## Undo/redo

`[REQ-UI-UNDO-001]` `Cmd/Ctrl`+`Z` / `Shift+Z` undo/redo through the shared command stack
(same stack MCP `batch.undo` uses — P3). Every UI action (create, move, resize, connect,
rename, delete) is one undoable command; multi-shape drags are one command.

`[REQ-UI-UNDO-002]` The undo stack persists across diagram switches (project-wide), not per
diagram. Stack depth: configurable, default 200.

## Clipboard

`[REQ-UI-CLIP-001]` Copy/paste (`Cmd/Ctrl`+`C`/`V`) copies selected elements (deep — with
their features) into the clipboard and pastes as new elements with new IDs, offset on the
diagram. Cross-diagram paste is allowed (pastes into the target diagram's owning package).
Relationships among pasted elements are duplicated; relationships to non-pasted elements are
NOT (they'd dangle) — instead a note is offered.

`[REQ-UI-CLIP-002]` Cut (`Cmd/Ctrl`+`X`) = copy + delete. Delete (`Del`) removes selected
elements (model delete, undoable). Cascade-delete prompt if elements have owned members.

## Diagram frame & header

`[REQ-UI-FRAME-001]` Each diagram MAY render a UML frame (per
[research/04](../research/04-uml-visual-notation.md)): a rectangle with a corner pentagon
containing the diagram kind keyword + name (e.g. `sd Login`, `activity Process order`).
Frames are optional, on by default for Sequence and Activity diagrams, off for Class
(configurable).

## Performance target

`[REQ-UI-PERF-001]` At ≤ 500 shapes + ≤ 1000 edges on one diagram, pan/zoom/edit MUST stay
at ≥ 60 fps on a 2020-era laptop. Auto-layout (ELK) runs in a Web Worker to avoid jank.

## Open questions (resolve before STABLE)

- [ ] Gizmo gesture threshold (8px) — confirm empirically with a prototype.
- [ ] Should the gizmo remember per-diagram-kind recent items across sessions? Tentative:
      yes, persisted in settings.
- [ ] Multi-touch (pinch) on trackpad vs. tablet — confirm Tauri webview gesture support.
- [ ] Right-drag vs. middle-mouse for pan — offer both; middle-mouse default on macOS (where
      right-drag is uncommon).
