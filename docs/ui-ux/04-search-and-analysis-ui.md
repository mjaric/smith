---
status: DRAFT
---

# UI/UX 04 — Search & Analysis UI

The surfaces for finding things and understanding model health. Search, the RTM view,
analysis dashboards, and the issue panel. These are the analytical face of Smith.

Backend: see [architecture/05-search.md](../architecture/05-search.md),
[analytics/01-rtm.md](../analytics/01-rtm.md),
[analytics/02-adjacency-and-graph-analysis.md](../analytics/02-adjacency-and-graph-analysis.md),
[analytics/03-orphan-and-coverage-analysis.md](../analytics/03-orphan-and-coverage-analysis.md).
Design tokens from [ui-ux/01-design-system.md](./01-design-system.md).

## Search

`[REQ-UI-SRCH-001]` Search is triggered by `Cmd/Ctrl`+`F` (or the toolbar search). A
search bar appears (top-center overlay or a dedicated panel); results stream in as you type
(debounced 150ms).

`[REQ-UI-SRCH-002]` Search matches:
- Element names (FTS5, ranked).
- Element qualified names (prefix match).
- Requirement IDs and test-case IDs (exact + prefix).
- Feature names (attributes, operations) — shown with their owning class.
- Comments body text.
- Diagram names.

`[REQ-UI-SRCH-003]` Result rows show: kind icon, name, qualified name (muted), matched
snippet (highlighted). Clicking a result navigates: opens the element's owning diagram (or
offers to) and selects the element.

`[REQ-UI-SRCH-004]` Filters (chips above results): element kind, owning package, stereotype
applied, defect state. These compose with the query.

`[REQ-UI-SRCH-005]` An **advanced search** mode (toggle) exposes structured queries: "all
Requirements in /System with status=approved and no «verify»" — composed via UI chips, backed
by the search/graph tools (see [mcp/02-tools.md](../mcp/02-tools.md) `search.*`,
`analysis.*`).

`[REQ-UI-SRCH-006]` Search is also exposed as the MCP tool `search.elements` for agents
(see [mcp/02-tools.md](../mcp/02-tools.md)).

## Issue panel

The always-on quality surface. Bottom panel, toggleable; shows a badge count in the toolbar.

`[REQ-UI-ISSUE-001]` The issue panel lists all current defects (D1–D7, see
[analytics/03](../analytics/03-orphan-and-coverage-analysis.md)), each row:
- Severity icon (colored dot: red HIGH, amber MEDIUM, blue LOW).
- Defect code + short message ("D1: Requirement REQ-099 has no satisfying relation").
- Offending element (click → select + navigate).
- "Fix" affordance where an auto-suggestion exists (e.g. "create UseCase"), linking to the
  relevant tool/prompt.
- Suppress checkbox (per-element-per-code; suppressed issues hide behind a filter toggle).

`[REQ-UI-ISSUE-002]` Issue panel features:
- Sort by severity (default), code, or element.
- Filter by severity, code, owning package.
- Group by code (all D1 together) or by package.
- "Show suppressed" toggle (off by default).
- Click-to-select syncs with the explorer and canvas.

`[REQ-UI-ISSUE-003]` The defect list updates **live** as the user edits (incremental — see
[analytics/03](../analytics/03-orphan-and-coverage-analysis.md) REQ-ORPH-INC-001). New defects
flash briefly (subtle highlight, 400ms) to draw attention without being jarring.

## RTM view

A dedicated view (opened from the menu or a tool button) showing the Requirements
Traceability Matrix — see [analytics/01-rtm.md](../analytics/01-rtm.md).

`[REQ-UI-RTM-001]` The RTM view has two tabs:
- **Forward** (requirement-centric): one row per Requirement; columns per
  [analytics/01](../analytics/01-rtm.md) §Forward RTM.
- **Backward** (artifact-centric): one row per non-requirement element with a trace;
  columns per §Backward RTM.

`[REQ-UI-RTM-002]` Aggregate metrics header: total requirements, % satisfied, % verified, %
chain complete, orphan counts (per [analytics/01](../analytics/01-rtm.md) §Aggregate metrics).

`[REQ-UI-RTM-003]` Filters: package subtree, category, priority, status, coverage state,
verdict. Column show/hide. Sort by any column.

`[REQ-UI-RTM-004]` Cells are interactive: clicking a satisfying-artifact cell navigates to
that element; empty cells for an orphan requirement are highlighted (severity tint) with a
"trace…" affordance.

`[REQ-UI-RTM-005]` Export: CSV, and as a model resource (`smith://analysis/rtm`,
`smith://analysis/rtm/backward`) for MCP consumption.

## Adjacency matrix view

`[REQ-UI-ADJ-001]` A heat-map view of the adjacency matrix (see
[analytics/02](../analytics/02-adjacency-and-graph-analysis.md)). Scope selector: current
diagram, a package subtree, or the whole model. Graph selector: relationship or trace.

`[REQ-UI-ADJ-002]` Cells colored by edge presence/count (monochrome intensity scale — NOT
categorical color, to avoid rainbow). Row/column headers are element names (sortable,
groupable by kind). Clicking a cell selects both elements and shows their relationships.

`[REQ-UI-ADJ-003]` For large scopes (>200 elements), the view paginates or offers to filter
to a package subtree (full-model adjacency is large; render is O(n²) cells).

## Graph analysis views

`[REQ-UI-GA-001]` On-demand analysis dialogs (opened from the inspector or menu), each
showing results + a "render on a copy of the current diagram" option:

| Analysis | View |
|----------|------|
| Reachability from element X | Highlighted subgraph (X + all reachable) on the diagram. |
| Shortest path X → Y | Highlighted path with hop count. |
| Connected components | List of components; click to highlight one on the diagram. |
| SCC (coupling clusters) | List of SCCs >1; highlight on diagram. |
| Topological order | Linear list (requirements/design/impl order). |
| Cycle report | List of cycles (edges); highlight one at a time. |
| Centrality | Sortable list of elements by betweenness/closeness; top elements highlighted. |

`[REQ-UI-GA-002]` Long-running analyses (full transitive closure, centrality) show a progress
indicator and are cancellable. They run off the writer thread (see
[mcp/01-server-design.md](../mcp/01-server-design.md) §Concurrency).

## Trace visualization

`[REQ-UI-TRACE-001]` "Show full chain" (from an element, per
[uml-model/05-traceability-chain.md](../uml-model/05-traceability-chain.md) REQ-CHAIN-008):
opens a **trace overview diagram** — a generated diagram showing the element's forward and
backward trace chain, laid out by chain level (Requirement → UseCase → Activity → Sequence →
Class). This is a transient, read-only view (not a stored diagram); it can be saved as a new
diagram if the user wants to keep it.

`[REQ-UI-TRACE-002]` The trace overview uses level-based layout (columns or rows per chain
level) and colors edges by trace stereotype (semantic tints, per
[ui-ux/01](./01-design-system.md)).

## Dashboard / overview

`[REQ-UI-DASH-001]` An optional project dashboard (menu → View → Dashboard): model stats
(element counts by kind, diagram counts, requirements by status, defect counts by severity),
rendered as simple stat cards + a couple of small charts (bar: requirements by status; donut:
defects by severity). Not a heavy BI surface — a quick health snapshot.

## Open questions (resolve before STABLE)

- [ ] Should the trace overview be interactive (click an element to recenter the chain on it)?
      Tentative: yes.
- [ ] Dashboard scope: project-wide only, or per-package? Tentative: project-wide in v1.
- [ ] Heat-map cell click behavior: select both, or open a relationships inspector? Tentative:
      select both + show a small popover listing the edges.
