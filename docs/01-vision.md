---
status: DRAFT
---

# 01 — Product Vision

## What Smith is

Smith is a **desktop UML modeling tool** with three distinguishing properties:

1. **The model is a graph.** The authoritative system model lives in an embedded graph
   database. Every element is a node; every relationship is an edge; ownership is a tree of
   `Package` → `Element`. A diagram is a *view* over this graph, not a container.

2. **It is agent-native.** Smith embeds a Model Context Protocol (MCP) server over HTTP/SSE
   on `127.0.0.1`. Any MCP-capable client — Claude Code, OpenAI Codex, Google Gemini CLI,
   or a browser-based chat — connects to the running Smith instance and reads or mutates the
   model through the same primitives a human uses. Humans and AI agents model the **same**
   system at the **same** time.

3. **It models the whole story.** Smith implements full UML 2.5.1 *and* extends it with
   SysML-inspired **Requirements** and **Test Cases**, plus a rich **traceability** relation
   set. The canonical chain
   `Requirement → UseCase → Activity → Sequence → Class` is first-class, and analytical
   tools surface gaps (orphans, uncovered elements, cycles) continuously.

## Who it is for

- **Systems engineers and software architects** who maintain living models of non-trivial
  systems and need traceability from requirements through design to implementation.
- **AI coding agents (and their operators)** that consume or produce design artifacts and
  benefit from a structured, queryable model instead of scraping diagrams as images.

Smith is **not** a toy diagrammer. It is a tool for models that earn their keep by being
queried, traced, and manipulated programmatically.

## Goals

| ID | Goal |
|----|------|
| `G1` | Store a complete UML 2.5.1 model in a graph database with package ownership. |
| `G2` | Render all 14 UML diagram kinds as views over the single model; never duplicate elements. |
| `G3` | Extend UML with `Requirement` and `TestCase` (SysML-inspired) as first-class model elements. |
| `G4` | Provide traceability relations (`«satisfy»`, `«verify»`, `«realize»`, `«deriveReqt»`, `«trace»`, `«refine»`, `«copy»`) and keep the canonical chain queryable. |
| `G5` | Ship analytical tools: RTM, adjacency matrix, orphan/uncovered-element detection, cycle detection, connected components. |
| `G6` | Embed an MCP server (HTTP/SSE, localhost) exposing tools/resources/prompts for AI agents. |
| `G7` | Provide a beautiful, ergonomic, eye-pleasing UI with a context-sensitive gizmo and ownership-aware selection. |
| `G8` | Distribute as a single self-contained desktop binary (no external DB server, no separate backend). |
| `G9` | Support search across the model with ranked results and graph-traversal queries. |
| `G10` | Support undo/redo and version-safe project files. |

## Non-goals (explicitly out of scope for v1)

- **Cloud / multi-user real-time collaboration.** Smith is a single-user desktop tool. The
  MCP server is for local agent collaboration, not remote human collaboration.
- **Code generation to source files.** Smith *models* systems; it does not emit compilable
  code. (A future plugin system may allow this; not in v1.)
- **Reverse engineering from source code.** v1 reads no source. Models are authored by
  humans and agents.
- **Mobile / web client.** Desktop only. The MCP server allows a *browser chat* to talk to
  Smith, but there is no browser-based canvas.
- **Full SysML.** Smith borrows SysML's Requirement/TestCase/trace concepts only. It does
  not implement blocks, parametrics, or allocation.
- **Custom diagram DSLs.** Smith ships the 14 UML diagrams plus the extension elements.

## Success criteria

The specification is "done" when an AI agent can implement any single feature by reading
only the relevant spec documents, without guessing:

1. Every metamodel element has an unambiguous identity, ownership rule, and relationship
   set.
2. Every diagram kind lists exactly which element types it may render and how.
3. Every MCP tool has a typed signature, error model, and example.
4. Every analytical query names the graph algorithm and the reference implementation.
5. Every UI interaction has a trigger, a response, and a fallback.
6. No two documents contradict each other; the glossary is the single vocabulary source.

## Design north stars

When any spec leaves a decision open, resolve it toward these stars (in order):

1. **The model is the truth; the diagram is a view.** Never store model facts on the diagram.
2. **Agents and humans are peers.** Anything a human can do, an MCP tool can do, and vice
   versa. The UI is a convenience layer over the MCP-capable model API.
3. **Boring where it matters.** The persistence layer and core data model use proven,
   well-understood choices. Novelty is reserved for UX.
4. **Silence is a bug.** If an operation would leave the model in a defective state (orphan,
   cycle, uncovered element), the tool surfaces it immediately rather than letting it
   accumulate.
5. **Beauty is a feature.** The UI must be pleasant for long working sessions: restrained
   color, generous whitespace, predictable motion, never visually loud.

## Naming

- **Project name:** Smith.
- **Project file:** `.smith` (a directory or single file — see persistence spec).
- **Model element IDs:** opaque, stable, globally unique within a project (see data model).
- **Code identifiers:** `snake_case` for Rust modules/fields; `PascalCase` for types; the
  MCP layer uses `camelCase` tool names per MCP convention.
