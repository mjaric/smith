# Smith — Specification

**Smith** is a desktop UML modeling tool written in Rust. The model is stored as a property
graph in an embedded graph database. Every diagram is a **view** (a "window") into the single
authoritative model — diagrams never own elements; the model's package ownership tree does.

Smith embeds an **MCP (Model Context Protocol) server over HTTP/SSE on localhost**, so any
AI agent — Claude Code, Codex, Gemini CLI, or a web chat — can read and manipulate the model
alongside a human user. This makes Smith an **agent-native** modeling tool: humans and LLMs
model the same system together.

Smith extends UML with **requirements** and **test cases** (SysML-inspired) and a rich
**traceability** relation set, so the full chain
`Requirement → UseCase → Activity → Sequence → Class` is first-class. Analytical tools
(Requirements Traceability Matrix, Adjacency Matrix, orphan/uncovered-element detection)
are built in.

This directory is the **complete, implementation-ready specification**. Each document is
self-contained and cross-referenced. An AI agent implementing a feature should be able to
read only the relevant spec documents plus this index and proceed without ambiguity.

---

## How to read this specification

1. Start with **[01-vision.md](./01-vision.md)** — product vision, goals, non-goals.
2. Read **[02-glossary.md](./02-glossary.md)** — shared vocabulary; every term is defined once.
3. Read the **research/** documents for authoritative background (OMG UML spec, MCP, graph DB,
   analytics algorithms, visual notation). These are *references*, not implementation specs.
4. Read the **architecture/** documents for the system design.
5. Read the **uml-model/**, **analytics/**, **ui-ux/**, and **mcp/** documents for each
   subsystem's detailed contract.
6. **[99-implementation-guide.md](./99-implementation-guide.md)** ties it together for
   implementers, including a suggested build order.

## Document map

### Foundational
| Doc | Purpose |
|-----|---------|
| [00-README.md](./00-README.md) | This index. |
| [01-vision.md](./01-vision.md) | Product vision, goals, non-goals, success criteria. |
| [02-glossary.md](./02-glossary.md) | Shared vocabulary (single source of truth for terms). |
| [03-design-principles.md](./03-design-principles.md) | Design principles every spec follows. |

### Research (reference material, not implementation specs)
| Doc | Purpose |
|-----|---------|
| [research/01-omg-uml-specification.md](./research/01-omg-uml-specification.md) | Map into the OMG UML 2.5.1 spec. |
| [research/02-mcp-transport.md](./research/02-mcp-transport.md) | MCP HTTP/SSE transport reference. |
| [research/03-graph-database-options.md](./research/03-graph-database-options.md) | Graph DB / persistence evaluation for Rust. |
| [research/04-uml-visual-notation.md](./research/04-uml-visual-notation.md) | UML notation standards per diagram family. |
| [research/05-traceability-and-analytics.md](./research/05-traceability-and-analytics.md) | Traceability relations, RTM, graph algorithms. |
| [research/06-canvas-and-frontend-stack.md](./research/06-canvas-and-frontend-stack.md) | Canvas rendering & frontend stack decision input. |

### Architecture
| Doc | Purpose |
|-----|---------|
| [architecture/01-system-architecture.md](./architecture/01-system-architecture.md) | Component topology, process model, data flow. |
| [architecture/02-data-model.md](./architecture/02-data-model.md) | Graph schema: nodes, edges, ownership tree, DI views. |
| [architecture/03-persistence.md](./architecture/03-persistence.md) | Storage strategy, transactions, undo, file format. |
| [architecture/04-traceability-relations.md](./architecture/04-traceability-relations.md) | Trace stereotype taxonomy + semantics. |
| [architecture/05-search.md](./architecture/05-search.md) | Search indexes, query language, algorithms. |
| [architecture/06-tech-stack.md](./architecture/06-tech-stack.md) | Chosen stack + rationale (Rust backend, frontend, DB). |

### UML Model (the metamodel Smith implements)
| Doc | Purpose |
|-----|---------|
| [uml-model/01-metamodel-core.md](./uml-model/01-metamodel-core.md) | NamedElement, Namespace, ownership, stereotypes, comments. |
| [uml-model/02-structural-diagrams.md](./uml-model/02-structural-diagrams.md) | Class, Object, Component, Composite, Package, Deployment, Profile. |
| [uml-model/03-behavioral-diagrams.md](./uml-model/03-behavioral-diagrams.md) | UseCase, Activity, Sequence, StateMachine, Communication, Timing, Interaction Overview. |
| [uml-model/04-requirements-and-tests.md](./uml-model/04-requirements-and-tests.md) | SysML-inspired Requirement + TestCase extensions. |
| [uml-model/05-traceability-chain.md](./uml-model/05-traceability-chain.md) | Canonical req→uc→activity→seq→class chain. |
| [uml-model/06-diagram-interchange.md](./uml-model/06-diagram-interchange.md) | Diagram-as-view: layout, presentation, view references. |

### Analytics
| Doc | Purpose |
|-----|---------|
| [analytics/01-rtm.md](./analytics/01-rtm.md) | Requirements Traceability Matrix (forward + backward). |
| [analytics/02-adjacency-and-graph-analysis.md](./analytics/02-adjacency-and-graph-analysis.md) | Adjacency matrix, centrality, SCC, cycles, reachability. |
| [analytics/03-orphan-and-coverage-analysis.md](./analytics/03-orphan-and-coverage-analysis.md) | Orphan req/element, uncovered element, disconnected clusters. |
| [analytics/04-quality-rules.md](./analytics/04-quality-rules.md) | Model-quality rule engine + severity model. |

### UI / UX
| Doc | Purpose |
|-----|---------|
| [ui-ux/01-design-system.md](./ui-ux/01-design-system.md) | Palette, type, spacing, motion, contrast, harmony. |
| [ui-ux/02-canvas-and-interaction.md](./ui-ux/02-canvas-and-interaction.md) | Canvas, tools, selection, **context-sensitive gizmo**, connection handles. |
| [ui-ux/03-inspector-and-explorer.md](./ui-ux/03-inspector-and-explorer.md) | Inspector, model explorer tree, properties panel. |
| [ui-ux/04-search-and-analysis-ui.md](./ui-ux/04-search-and-analysis-ui.md) | Search, RTM view, analysis dashboards, issue panel. |

### MCP Server
| Doc | Purpose |
|-----|---------|
| [mcp/01-server-design.md](./mcp/01-server-design.md) | Transport, lifecycle, endpoints, capability negotiation. |
| [mcp/02-tools.md](./mcp/02-tools.md) | Every tool an agent can call (model manipulation + analysis). |
| [mcp/03-resources.md](./mcp/03-resources.md) | Readable model resources exposed to agents. |
| [mcp/04-prompts.md](./mcp/04-prompts.md) | Predefined prompts for common modeling tasks. |
| [mcp/05-client-config.md](./mcp/05-client-config.md) | How to register Smith in Claude Code, Codex, Gemini, web. |

### Implementation
| Doc | Purpose |
|-----|---------|
| [99-implementation-guide.md](./99-implementation-guide.md) | Build order, module boundaries, acceptance criteria template. |

---

## Status legend

Each document carries a status banner:

- `DRAFT` — being written; not yet authoritative.
- `REVIEW` — written; open for cross-review and refinement.
- `STABLE` — reviewed, unambiguous, ready for implementation.

Do not implement against a `DRAFT` document. If a `STABLE` doc has an ambiguity, file it as
an issue against that doc rather than guessing.

## Conventions

- **RFC 2119** keywords (MUST, SHOULD, MAY) are used with their standard meaning.
- Cross-references use relative markdown links.
- External references use full URLs with a one-line citation.
- Every normative requirement is tagged with a stable ID, e.g. `[REQ-DATA-001]`, so
  features and tests can trace back to the spec line that mandates them.
