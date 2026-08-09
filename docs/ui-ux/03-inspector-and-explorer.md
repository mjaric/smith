---
status: DRAFT
---

# UI/UX 03 — Inspector, Model Explorer, Properties

The chrome around the canvas. Three panels form the workspace: the **model explorer**
(ownership tree), the **canvas** (center), and the **inspector** (context properties).
Together with the issue panel ([ui-ux/04](./04-search-and-analysis-ui.md)) and the toolbar,
these are the whole UI.

Design tokens from [ui-ux/01-design-system.md](./01-design-system.md). Selection model from
[ui-ux/02-canvas-and-interaction.md](./02-canvas-and-interaction.md).

## Workspace layout

`[REQ-UI-LAY-001]` Default three-pane layout:

```
┌─────────────────────────────────────────────────────────────┐
│ Menu bar · diagram tabs · toolbar                            │
├──────────┬──────────────────────────────────────┬───────────┤
│          │                                      │           │
│  Model   │                                      │ Inspector │
│ explorer │            Canvas                    │           │
│  (tree)  │                                      │ (context) │
│          │                                      │           │
├──────────┴──────────────────────────────────────┴───────────┤
│ Issue panel (toggleable, bottom) · status bar                │
└─────────────────────────────────────────────────────────────┘
```

`[REQ-UI-LAY-002]` Panels are collapsible and resizable (drag the dividers). State
persisted per project. A "focus mode" (`Cmd/Ctrl`+`.`) hides side panels for full-canvas work.

`[REQ-UI-LAY-003]` The layout MUST be responsive to window size: at narrow widths, panels
overlay (drawer style) rather than squeeze the canvas.

## Model explorer (ownership tree)

Renders the model's ownership tree (Packages → elements), the authoritative structure (P1,
P2). NOT a list of diagrams — diagrams are views and live under their owning package in the
tree.

`[REQ-UI-EXP-001]` Tree nodes:
- **Packages** — folder icon; expandable; show child count.
- **Diagrams** — diagram-kind icon; double-click opens the diagram in the canvas.
- **Elements** — kind icon + name; double-click opens the diagram that shows this element
  (or offers to add it to one), per [ui-ux/02](./02-canvas-and-interaction.md) ownership-aware
  selection.

`[REQ-UI-EXP-002]` Tree features:
- Expand/collapse all, per package.
- Drag-and-drop to **reparent** elements between packages (model operation, undoable — see
  [mcp/02-tools.md](../mcp/02-tools.md) `model.reparent`).
- Right-click context menu: new package/element/diagram, rename, delete, apply stereotype,
  trace forward/backward, find on diagrams, copy qualified name.
- **Badges**: defect indicators (colored dot per severity — see
  [ui-ux/01](./01-design-system.md)); "uncovered" indicator (dashed outline) for elements on
  no diagram.
- Search-as-you-type filter (fuzzy) that prunes the tree to matches + ancestors.
- Sort: by name (default), by kind, by defect severity.

`[REQ-UI-EXP-003]` Selection in the explorer syncs with the canvas: selecting an element in
the tree selects it on the active diagram if shown (and vice versa). If the element isn't on
the active diagram, the explorer offers "show on diagram…" listing diagrams that reference it.

`[REQ-UI-EXP-004]` The explorer is DOM-based (React), making it the accessible view of the
model (full keyboard nav, ARIA tree roles) — per
[ui-ux/01](./01-design-system.md) REQ-DS-022.

## Inspector (context properties)

The right panel. Shows properties of the current selection. Context-sensitive: different
content for an element, a relationship, a diagram, or nothing (project info).

`[REQ-UI-INS-001]` When a single element is selected, the inspector shows sections
(collapsible):

| Section | Content |
|---------|---------|
| **Identity** | Name (editable), qualified name (read-only, derived), element kind, stable ID (read-only, copyable). |
| **Ownership** | Owning package (click to navigate), "move to…" (reparent). |
| **Stereotypes** | Applied stereotypes (add/remove), tagged values (edit) per stereotype. |
| **Features** (class-family) | Attributes and operations — add/edit/remove/reorder. Each row: inline editor with visibility prefix, type, multiplicity. |
| **Kind-specific** | E.g. Requirement: requirementId, text, category, priority, status, verification methods, source, rationale. TestCase: testCaseId, title, steps, expected, specification link. |
| **Comments** | List of comments on the element; add/edit. |
| **Relationships** | Incoming and outgoing relationships (clickable → navigate to the other end). |
| **Traceability** | Forward/backward trace summary (counts per chain level), with "show chain" button. |
| **Coverage** | List of diagrams showing this element; "uncovered" badge if none. |
| **Analysis** | Centrality score (if computed), defect count. |

`[REQ-UI-INS-002]` When a **relationship** is selected, the inspector shows: relationship
kind, source (clickable), target (clickable), stereotype (for trace relations),
multiplicities (for associations), bend points count (view), owner package.

`[REQ-UI-INS-003]` When a **diagram** is selected (no element), the inspector shows: diagram
name, kind, owning package, element/edge counts, canvas viewport, and diagram-level settings
(frame on/off, routing kind, layout defaults).

`[REQ-UI-INS-004]` When **nothing** is selected, the inspector shows project info (name,
stats, recent changes) — see `smith://project/info`.

`[REQ-UI-INS-005]` Editing in the inspector goes through the model API (undoable, syncs to
canvas + MCP subscribers). Inline validation: invalid values (bad multiplicity, duplicate
requirementId) show an error and do not commit.

`[REQ-UI-INS-006]` Multi-selection: the inspector shows **common properties** only (kind,
ownership) and indicates "N elements selected"; bulk actions (delete, apply stereotype,
reparent) are available.

## Toolbar

`[REQ-UI-TB-001]` Top toolbar (left to right):
- Diagram tabs (open diagrams; switch; close).
- Tool palette (Select, Create-kind..., Connect, etc. — see
  [ui-ux/02](./02-canvas-and-interaction.md)).
- Layout actions (auto-layout dropdown, align/distribute for multi-select).
- Undo/redo.
- Search trigger (`Cmd/Ctrl`+`F`).
- Issue-panel toggle (with defect count badge).
- Agent status (connected MCP clients, with connection indicators).

## Menu bar

`[REQ-UI-MENU-001]` Standard menus: File (new/open/save/save as/export/recent), Edit
(undo/redo/cut/copy/paste/find/preferences), View (toggle panels, zoom, theme), Model (new
package/element/diagram, validate, run analysis), Diagram (layout, export), Help. Items have
keyboard shortcuts; all actions are reachable by keyboard (a11y).

## Status bar

`[REQ-UI-STATUS-001]` Bottom status bar: current diagram + kind, cursor coordinates (canvas),
zoom %, element/edge counts on the active diagram, save state (saved / unsaved indicator),
MCP server status (running, port, connected clients count).

## Drag-and-drop between panels

`[REQ-UI-DND-001]` Supported drags:
- Explorer element → canvas: adds a view reference (shape) for that element on the target
  diagram (if the element kind is allowed on that diagram kind).
- Explorer element → inspector "stereotype applies to": N/A (use inspector controls).
- Canvas shape → explorer package: reparents the model element (model operation).

## Persistence of UI state

`[REQ-UI-PERSIST-001]` Smith persists per-project UI state: open diagrams, panel
sizes/collapse, last viewport per diagram, expanded explorer nodes, recent element kinds per
diagram (for the gizmo hub — see [ui-ux/02](./02-canvas-and-interaction.md)). Stored in the
`.smith` file alongside the model (UI state section, not model data).

## Open questions (resolve before STABLE)

- [ ] Should the inspector support a "split" mode (show two elements side by side for
      comparison)? Tentative: no in v1; add if users request.
- [ ] Explorer drag-to-reparent confirmation: always ask, or only on cross-package moves?
      Tentative: only on moves that change qualifiedName significantly (different top-level
      package); same-subtree moves are silent.
