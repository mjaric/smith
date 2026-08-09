---
status: DRAFT
---

# 03 — Design Principles

Invariants every Smith specification document and every implementation decision MUST honor.
When a spec is silent, resolve the ambiguity toward these principles in the order listed.

## P1 — The model is the single source of truth

There is exactly one model per project, stored as a graph. **Diagrams never store model
facts.** A diagram stores only:

1. Which elements and relationships it *references* (view references).
2. Per-reference presentation data (position, size, bend points, compartment folding).

Changing a class name on one diagram changes it everywhere, because the name lives on the
element in the model, not on the diagram view. Deleting a diagram deletes zero model
elements; it only deletes view references.

**Consequence:** every model fact (name, attribute, operation, relationship, stereotype,
tagged value) is stored exactly once, on the owning element.

## P2 — Ownership lives in the model tree, never on the diagram

Every element has exactly one *owner* (a `Namespace`, usually a `Package`), forming a tree
rooted at the project. This ownership tree is the model's authoritative structure. The model
explorer renders this tree; diagrams render a *flat or re-laid-out subset*.

**Consequence:** moving an element between diagrams does not change its owner. Reparenting
an element (move to another package) is a model operation, reflected on every diagram that
references it.

## P3 — Agents and humans are peers

The model API is the boundary. Both the UI and the MCP server call the **same** underlying
model operations. There is no UI-only capability and no agent-only capability. If the UI can
create a stereotyped class with a tagged value, so can an MCP tool, with identical
validation and identical undo history.

**Consequence:** the UI is a convenience layer; correctness and completeness live in the
model API. Every UI action MUST have an MCP tool equivalent (it may be a composite of
several tools).

## P4 — Make defects visible immediately

Smith continuously runs lightweight quality checks. Orphan requirements, uncovered elements,
trace cycles, and disconnected subgraphs are surfaced in the issue panel the moment they
arise — not as a separate "validation" step the user remembers to run.

Heavy analyses (full RTM, centrality) run on demand, but the cheap structural checks
(orphan, uncovered, cycle) are incremental and live.

**Consequence:** the model carries an always-up-to-date quality state. The UI and MCP both
expose it.

## P5 — Boring foundations, novel UX

Persistence, the graph schema, and the MCP protocol are well-understood problems. Use proven
choices (see tech-stack spec). Reserve novelty and ambition for the **interaction design**:
the context-sensitive gizmo, ownership-aware selection, live analysis — the things that make
Smith pleasant to use.

**Consequence:** do not invent a custom query language if Datalog/SQL suffices. Do not
invent a custom serialization format if an existing one works.

## P6 — Reference, don't re-derive

Algorithms (BFS, topological sort, Tarjan SCC, Floyd-Warshall reachability) are defined once
in authoritative online sources. Smith specs **reference** them by name and link, and specify
only *what Smith computes with them* and *when*. No spec document re-derives algorithm
mathematics.

**Consequence:** see [research/05-traceability-and-analytics.md](./research/05-traceability-and-analytics.md).

## P7 — Restraint in visuals

The UI must be pleasant for multi-hour sessions. Defaults: neutral background, restrained
accent color, high (but not maximum) contrast, predictable and short motion, no gratuitous
animation. Color is used semantically (element type, relationship type, defect severity),
never decoratively. The palette follows a harmony model (see design-system spec).

**Consequence:** no rainbow diagrams, no neon, no ambient animation on idle canvas.

## P8 — Stable identities, derived names

Every element has a stable, opaque, immutable ID (assigned at creation, never reused).
Human-readable names are mutable and non-unique within a namespace. Qualified names are
derived from the ownership tree. Relationships reference elements by ID, never by name.

**Consequence:** rename is cheap and safe. Reparent changes the qualified name but never the
ID or any relationship.

## P9 — Undoable, auditable

Every model mutation is an undoable command on a global command stack. The MCP tools
participate in the same undo stack as UI actions (batched sensibly — see MCP server design).
There is no destructive operation that bypasses undo.

**Consequence:** no `delete everything` footguns; destructive operations are undoable for
the duration of the session.

## P10 — Local-first, single binary

Smith runs as one process on the user's machine: UI + model API + embedded graph DB + MCP
server. No external database process, no separate backend service. The project is a single
`.smith` artifact. The MCP server binds to `127.0.0.1` only.

**Consequence:** distribution is trivial; there is no server to configure.

## Decision ladder

When a spec is ambiguous, climb this ladder:

1. **The spec document for the subsystem** (normative).
2. **This design-principles document.**
3. **The OMG UML 2.5.1 spec** (for UML metamodel questions) — see research/01.
4. **The MCP spec** (for protocol questions) — see research/02.
5. **A recorded ADR** (architecture decision record, if one exists).

If none resolves it, the implementer MUST file the ambiguity as a spec issue and not guess.
