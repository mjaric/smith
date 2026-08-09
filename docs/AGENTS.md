---
status: REVIEW
auto-generated: true
generator: "Regenerate by scanning headings + bracketed IDs in docs/. (A docs/scripts/build_index.py is a TODO; until then, re-run the extraction.)"
---

# AGENTS.md — Spec Navigation Index

How to use this file: read it once at session start. It maps every cross-reference in the
Smith spec so you can jump straight to the right document/section instead of grepping blindly.

**Source:** all 36 markdown documents under `docs/` (~418 KB). **Scope:** every `## H2`
section, every `[REQ-*-NNN]` / `[INV-*-NNN]` ID, defect codes `D1`–`D7`, principles
`P1`–`P10`, goals `G1`–`G10`, the 15 `DiagramKind`s, trace stereotypes, MCP tools, and error
codes.

> **ID convention.** Normative requirement IDs are written in backticks at line start,
> e.g. `` `[REQ-MM-001]` Every element MUST … ``. Cross-references to an ID elsewhere
> usually drop the brackets (e.g. `REQ-MM-001`); the **Defined in** column below is always
> the single document carrying the normative backtick statement. There are 333 unique REQ
> IDs; 332 have a normative definition and 1 (`REQ-DATA-001`) is an example-only placeholder
> in `00-README.md` that is never normatively defined — see §2.

## 1. Document outline map

Each document's `## H2` sections (H3+ omitted as too noisy). Line numbers are current but
drift on edit — trust the section title. `architecture/02-data-model.md` is a DRAFT pointer
stub with no H2 sections.

### Foundational

#### `docs/00-README.md` — This index + how to read the spec.

- §1 How to read this specification  *(L24)*
- §2 Document map  *(L36)*
- §3 Status legend  *(L108)*
- §4 Conventions  *(L119)*

#### `docs/01-vision.md` — Product vision, goals, non-goals, success criteria.

- §1 What Smith is  *(L7)*
- §2 Who it is for  *(L27)*
- §3 Goals  *(L37)*
- §4 Non-goals (explicitly out of scope for v1)  *(L52)*
- §5 Success criteria  *(L66)*
- §6 Design north stars  *(L79)*
- §7 Naming  *(L94)*

#### `docs/02-glossary.md` — Shared vocabulary (single source of truth for terms).

- §1 Model & ownership  *(L11)*
- §2 Diagrams as views  *(L24)*
- §3 Metamodel (UML core)  *(L34)*
- §4 Traceability & extensions  *(L47)*
- §5 Analytics  *(L60)*
- §6 Agent / MCP  *(L72)*
- §7 UI  *(L83)*
- §8 File & runtime  *(L96)*

#### `docs/03-design-principles.md` — Design principles every spec follows (P1–P10).

- §1 P1 — The model is the single source of truth  *(L10)*
- §2 P2 — Ownership lives in the model tree, never on the diagram  *(L25)*
- §3 P3 — Agents and humans are peers  *(L35)*
- §4 P4 — Make defects visible immediately  *(L46)*
- §5 P5 — Boring foundations, novel UX  *(L58)*
- §6 P6 — Reference, don't re-derive  *(L68)*
- §7 P7 — Restraint in visuals  *(L77)*
- §8 P8 — Stable identities, derived names  *(L86)*
- §9 P9 — Undoable, auditable  *(L95)*
- §10 P10 — Local-first, single binary  *(L104)*
- §11 Decision ladder  *(L112)*


### Research (reference, not implementation spec)

#### `docs/research/01-omg-uml-specification.md` — Map into the OMG UML 2.5.1 spec.

- §1 Spec Identification & Clause Map  *(L12)*
- §2 Root Abstractions: Element and NamedElement  *(L71)*
- §3 Ownership & Namespace Model (the tree Smith replicates)  *(L115)*
- §4 Element Category Taxonomy  *(L177)*
- §5 Relationship & Dependency Metaclasses (traceability substrate)  *(L207)*
- §6 Diagram Kinds (14) — Annex A  *(L266)*
- §7 Stereotypes & Profiles (§12.3, §22)  *(L302)*
- §8 Comment / Note Mechanism (§7.8.2)  *(L334)*
- §9 Diagram Interchange (Annex B + DD) — the "diagram as a view" basis  *(L346)*
- §10 Normative Machine-Readable Artifacts  *(L383)*
- §11 Sources  *(L399)*

#### `docs/research/02-mcp-transport.md` — MCP HTTP/SSE transport reference.

- §1 1. Specification versions  *(L16)*
- §2 2. Transport options  *(L38)*
- §3 3. Server lifecycle & endpoints (legacy-era Streamable HTTP)  *(L139)*
- §4 4. Protocol primitives Smith must implement  *(L219)*
- §5 5. Capability negotiation — Smith's initialize response  *(L317)*
- §6 6. Client configuration formats  *(L351)*
- §7 7. Auth & security on localhost  *(L461)*
- §8 8. Rust SDK status  *(L495)*
- §9 Sources  *(L543)*

#### `docs/research/03-graph-database-options.md` — Graph DB / persistence evaluation for Rust.

- §1 1. Evaluation Axes  *(L9)*
- §2 2. Candidate Matrix  *(L26)*
- §3 3. Candidate Notes  *(L47)*
- §4 4. Recommendation  *(L98)*
- §5 5. Ownership-Tree Representation (Package → Element)  *(L129)*
- §6 6. Graph Traversal Algorithms Smith Needs  *(L148)*
- §7 7. Versioning / Undo  *(L166)*
- §8 Sources  *(L178)*

#### `docs/research/04-uml-visual-notation.md` — UML notation standards per diagram family.

- §1 Where notation lives in the UML 2.5.1 specification  *(L22)*
- §2 Cross-cutting notation rules (all diagrams)  *(L56)*
- §3 1. Class diagram  *(L139)*
- §4 2. Use Case diagram  *(L169)*
- §5 3. Activity diagram  *(L184)*
- §6 4. Sequence diagram  *(L210)*
- §7 5. State Machine diagram  *(L242)*
- §8 6. Communication diagram  *(L269)*
- §9 7. Component diagram  *(L280)*
- §10 8. Composite Structure diagram  *(L296)*
- §11 9. Deployment diagram  *(L312)*
- §12 10. Package diagram  *(L326)*
- §13 11. Object diagram  *(L340)*
- §14 12. Timing diagram  *(L352)*
- §15 13. Interaction Overview diagram  *(L366)*
- §16 14. Profile diagram  *(L383)*
- §17 Smith extension notation (requirements, test cases, traceability)  *(L400)*
- §18 Notation fixed by the spec vs. open to Smith  *(L416)*
- §19 Layout conventions (industry standard, not spec-mandated)  *(L441)*
- §20 Stereotype notation  *(L455)*
- §21 Color guidance for aesthetics (reference only — palette is the design system's job)  *(L464)*
- §22 Sources  *(L483)*

#### `docs/research/05-traceability-and-analytics.md` — Traceability relations, RTM, graph algorithms.

- §1 1. Traceability relations (UML + SysML-inspired)  *(L24)*
- §2 2. Requirements Traceability Matrix (RTM)  *(L97)*
- §3 3. Adjacency matrix and graph algorithms (reference index)  *(L149)*
- §4 4. Quality queries Smith must support  *(L170)*
- §5 5. SysML requirement & test modeling reference  *(L191)*
- §6 Sources  *(L215)*

#### `docs/research/06-canvas-and-frontend-stack.md` — Canvas rendering & frontend stack decision input.

- §1 1. What the canvas must do (requirements recap)  *(L20)*
- §2 2. Architecture-level decision matrix (options a / b / c)  *(L42)*
- §3 3. Web canvas / diagramming libraries (stack (a) internals)  *(L74)*
- §4 4. Native Rust GUI options (stack (b))  *(L100)*
- §5 5. Frontend framework (given stack (a))  *(L118)*
- §6 6. Top-3 ranked recommendations for Smith  *(L137)*
- §7 7. Connection routing libraries (orthogonal connectors)  *(L172)*
- §8 8. Auto-layout libraries  *(L198)*
- §9 9. Recommendation  *(L218)*
- §10 Sources  *(L252)*


### Architecture

#### `docs/architecture/01-system-architecture.md` — Component topology, process model, data flow.

- §1 Process model (single process)  *(L13)*
- §2 Concurrency model  *(L74)*
- §3 Module boundaries (Rust workspace)  *(L89)*
- §4 Data flow: a mutation  *(L105)*
- §5 Data flow: a read  *(L120)*
- §6 Change propagation  *(L130)*
- §7 Startup sequence  *(L144)*
- §8 Shutdown sequence  *(L156)*
- §9 Configuration  *(L166)*
- §10 Error handling  *(L170)*
- §11 Logging  *(L176)*
- §12 Open questions (resolve before STABLE)  *(L180)*

#### `docs/architecture/02-data-model.md` — Index pointer (graph schema lives in arch/03; metamodel in uml-model/01,06).

- *(no H2 sections — stub/pointer)*

#### `docs/architecture/03-persistence.md` — Storage strategy, SQLite schema, transactions, undo, file format.

- §1 Store of record vs. projection  *(L15)*
- §2 SQLite schema  *(L30)*
- §3 Closure-table maintenance  *(L170)*
- §4 Migrations  *(L179)*
- §5 Transactions & concurrency  *(L185)*
- §6 Undo / redo (command stack)  *(L193)*
- §7 `.smith` file format  *(L206)*
- §8 Performance budget  *(L216)*
- §9 Open questions (resolve before STABLE)  *(L224)*

#### `docs/architecture/04-traceability-relations.md` — Trace stereotype taxonomy + semantics.

- §1 Base metaclass  *(L15)*
- §2 Relation taxonomy  *(L33)*
- §3 Cardinality rules  *(L47)*
- §4 The canonical chain  *(L63)*
- §5 Trace graph vs. relationship graph  *(L95)*
- §6 Quality rules attached to trace relations  *(L109)*
- §7 Stereotype lifecycle  *(L121)*
- §8 Open questions (resolve before STABLE)  *(L128)*

#### `docs/architecture/05-search.md` — Search indexes (FTS5), query language, algorithms.

- §1 Full-text search (FTS5)  *(L14)*
- §2 Structured filtering  *(L37)*
- §3 Graph-traversal queries  *(L58)*
- §4 Graph scope  *(L84)*
- §5 Index strategy  *(L90)*
- §6 Performance budget  *(L106)*
- §7 MCP exposure  *(L115)*
- §8 Open questions (resolve before STABLE)  *(L122)*

#### `docs/architecture/06-tech-stack.md` — Chosen stack + rationale (Rust backend, frontend, DB).

- §1 Summary table  *(L16)*
- §2 Process topology  *(L43)*
- §3 Why SQLite (not a graph DB)  *(L96)*
- §4 Why Tauri 2 + web (not pure-Rust GUI)  *(L115)*
- §5 Version pins (to be locked at implementation start)  *(L132)*
- §6 Non-choices (explicitly rejected)  *(L160)*
- §7 Open questions (resolve before STABLE)  *(L169)*


### UML Model (the metamodel Smith implements)

#### `docs/uml-model/01-metamodel-core.md` — NamedElement, Namespace, ownership, stereotypes, comments.

- §1 Root abstractions  *(L14)*
- §2 Stereotypes & tagged values  *(L74)*
- §3 Comments  *(L114)*
- §4 Relationships — base types  *(L126)*
- §5 Visibility  *(L164)*
- §6 Multiplicity  *(L168)*
- §7 Metaclass kind registry  *(L179)*
- §8 Invariants (always checked)  *(L261)*
- §9 Open questions (resolve before STABLE)  *(L270)*

#### `docs/uml-model/02-structural-diagrams.md` — Class, Object, Component, Composite, Package, Deployment, Profile.

- §1 Principle: a diagram kind constrains view references  *(L16)*
- §2 1. Class diagram (`kind = "class"`)  *(L26)*
- §3 2. Object diagram (`kind = "object"`)  *(L58)*
- §4 3. Component diagram (`kind = "component"`)  *(L69)*
- §5 4. Composite Structure diagram (`kind = "compositeStructure"`)  *(L82)*
- §6 5. Package diagram (`kind = "package"`)  *(L94)*
- §7 6. Deployment diagram (`kind = "deployment"`)  *(L107)*
- §8 7. Profile diagram (`kind = "profile"`)  *(L120)*
- §9 Common constraints (all structural diagrams)  *(L131)*
- §10 Traceability participation  *(L146)*
- §11 Open questions (resolve before STABLE)  *(L155)*

#### `docs/uml-model/03-behavioral-diagrams.md` — UseCase, Activity, Sequence, StateMachine, Communication, Timing, Interaction Overview.

- §1 1. Use Case diagram (`kind = "useCase"`)  *(L20)*
- §2 2. Activity diagram (`kind = "activity"`)  *(L44)*
- §3 3. Sequence diagram (`kind = "sequence"`)  *(L74)*
- §4 4. State Machine diagram (`kind = "stateMachine"`)  *(L107)*
- §5 5. Communication diagram (`kind = "communication"`)  *(L133)*
- §6 6. Timing diagram (`kind = "timing"`)  *(L144)*
- §7 7. Interaction Overview diagram (`kind = "interactionOverview"`)  *(L155)*
- §8 Common constraints (all behavioral diagrams)  *(L169)*
- §9 Traceability participation  *(L179)*
- §10 Open questions (resolve before STABLE)  *(L190)*

#### `docs/uml-model/04-requirements-and-tests.md` — SysML-inspired Requirement + TestCase extensions.

- §1 Requirement (extends NamedElement)  *(L15)*
- §2 TestCase (extends Behavior)  *(L66)*
- §3 Requirement package convention  *(L124)*
- §4 Built-in profile: `smith::requirements`  *(L131)*
- §5 Relationship to traceability  *(L148)*
- §6 Requirement diagram (kind = `requirement`)  *(L158)*
- §7 Enumeration value sets  *(L183)*
- §8 Open questions (resolve before STABLE)  *(L192)*

#### `docs/uml-model/05-traceability-chain.md` — Canonical req→uc→activity→seq→class chain.

- §1 The canonical chain (restated)  *(L16)*
- §2 Per-level semantics  *(L45)*
- §3 Forward and backward trace  *(L64)*
- §4 Completeness levels  *(L82)*
- §5 Diagram participation  *(L100)*
- §6 Navigating the chain in the UI  *(L117)*
- §7 What breaks completeness (defects)  *(L127)*
- §8 Open questions (resolve before STABLE)  *(L140)*

#### `docs/uml-model/06-diagram-interchange.md` — Diagram-as-view: layout, presentation, view references.

- §1 The principle, restated  *(L14)*
- §2 Diagram element (the view container)  *(L28)*
- §3 View references  *(L58)*
- §4 CanvasState (viewport)  *(L109)*
- §5 PresentationHints  *(L118)*
- §6 Units & coordinate system  *(L134)*
- §7 Coverage (uncovered-element detection)  *(L142)*
- §8 View-reference integrity  *(L157)*
- §9 What "diagram owns" summary  *(L165)*
- §10 Open questions (resolve before STABLE)  *(L174)*


### Analytics

#### `docs/analytics/01-rtm.md` — Requirements Traceability Matrix (forward + backward).

- §1 Inputs  *(L16)*
- §2 Two views, one graph  *(L33)*
- §3 Coverage state machine  *(L66)*
- §4 Aggregate metrics  *(L80)*
- §5 Algorithms  *(L93)*
- §6 Filtering and export  *(L109)*
- §7 Defects surfaced by RTM  *(L120)*
- §8 Open questions (resolve before STABLE)  *(L133)*

#### `docs/analytics/02-adjacency-and-graph-analysis.md` — Adjacency matrix, centrality, SCC, cycles, reachability.

- §1 Two graph inputs  *(L17)*
- §2 Adjacency matrix  *(L31)*
- §3 Reachability & transitive closure  *(L46)*
- §4 Topological sort  *(L64)*
- §5 Cycle detection  *(L78)*
- §6 Strongly connected components (SCC)  *(L87)*
- §7 Shortest path  *(L98)*
- §8 Centrality  *(L110)*
- §9 Connected components  *(L124)*
- §10 Dominator tree (advanced, optional)  *(L132)*
- §11 Performance budget  *(L140)*
- §12 MCP tool surface  *(L150)*
- §13 Open questions (resolve before STABLE)  *(L167)*

#### `docs/analytics/03-orphan-and-coverage-analysis.md` — Orphan req/element, uncovered element, disconnected clusters (D1–D7).

- §1 Defect taxonomy  *(L11)*
- §2 D1 — Orphan requirement  *(L27)*
- §3 D2 — Orphan test  *(L40)*
- §4 D3 — Orphan element  *(L50)*
- §5 D4 — Uncovered element  *(L75)*
- §6 D5 — Circular trace  *(L96)*
- §7 D6 — Incomplete chain  *(L114)*
- §8 D7 — Disconnected subgraph  *(L135)*
- §9 Severity model  *(L148)*
- §10 Suppression  *(L159)*
- §11 Incremental maintenance  *(L170)*
- §12 MCP surface  *(L180)*
- §13 Open questions (resolve before STABLE)  *(L187)*

#### `docs/analytics/04-quality-rules.md` — Model-quality rule engine + severity model.

- §1 Rule definition  *(L12)*
- §2 Built-in rules  *(L27)*
- §3 Evaluation model  *(L44)*
- §4 Severity & blocking  *(L63)*
- §5 Suppression  *(L74)*
- §6 User-defined rules (future)  *(L86)*
- §7 MCP surface  *(L92)*
- §8 Open questions (resolve before STABLE)  *(L104)*


### UI / UX

#### `docs/ui-ux/01-design-system.md` — Palette, type, spacing, motion, contrast, harmony.

- §1 Design tokens  *(L20)*
- §2 Color  *(L29)*
- §3 Typography  *(L87)*
- §4 Spacing & sizing  *(L111)*
- §5 Stroke & shape  *(L119)*
- §6 Motion  *(L136)*
- §7 Theming  *(L147)*
- §8 Iconography  *(L155)*
- §9 Z-order & layering  *(L164)*
- §10 Accessibility (beyond color)  *(L170)*
- §11 Open questions (resolve before STABLE)  *(L180)*

#### `docs/ui-ux/02-canvas-and-interaction.md` — Canvas, tools, selection, context-sensitive gizmo, connection handles.

- §1 Tool model  *(L16)*
- §2 Selection model (ownership-aware)  *(L38)*
- §3 The context-sensitive gizmo (signature feature)  *(L60)*
- §4 Connection creation  *(L148)*
- §5 Inline editing  *(L169)*
- §6 Move, resize, snap  *(L178)*
- §7 Pan & zoom  *(L192)*
- §8 Undo/redo  *(L206)*
- §9 Clipboard  *(L215)*
- §10 Diagram frame & header  *(L226)*
- §11 Performance target  *(L234)*
- §12 Open questions (resolve before STABLE)  *(L239)*

#### `docs/ui-ux/03-inspector-and-explorer.md` — Inspector, model explorer tree, properties panel.

- §1 Workspace layout  *(L15)*
- §2 Model explorer (ownership tree)  *(L39)*
- §3 Inspector (context properties)  *(L72)*
- §4 Toolbar  *(L112)*
- §5 Menu bar  *(L124)*
- §6 Status bar  *(L131)*
- §7 Drag-and-drop between panels  *(L137)*
- §8 Persistence of UI state  *(L145)*
- §9 Open questions (resolve before STABLE)  *(L152)*

#### `docs/ui-ux/04-search-and-analysis-ui.md` — Search, RTM view, analysis dashboards, issue panel.

- §1 Search  *(L16)*
- §2 Issue panel  *(L45)*
- §3 RTM view  *(L69)*
- §4 Adjacency matrix view  *(L93)*
- §5 Graph analysis views  *(L106)*
- §6 Trace visualization  *(L125)*
- §7 Dashboard / overview  *(L138)*
- §8 Open questions (resolve before STABLE)  *(L145)*


### MCP Server

#### `docs/mcp/01-server-design.md` — Transport, lifecycle, endpoints, capability negotiation.

- §1 Transport  *(L18)*
- §2 Endpoints  *(L36)*
- §3 Capability negotiation  *(L50)*
- §4 Session lifecycle  *(L74)*
- §5 Discovery  *(L91)*
- §6 Authentication & security  *(L117)*
- §7 Error model  *(L135)*
- §8 Undo & batching  *(L152)*
- §9 Concurrency  *(L162)*
- §10 Transport selection rationale  *(L172)*
- §11 Open questions (resolve before STABLE)  *(L182)*

#### `docs/mcp/02-tools.md` — Every tool an agent can call (model + analysis + search).

- §1 model.* — element & relationship operations  *(L25)*
- §2 diagram.* — view operations  *(L120)*
- §3 analysis.* — analytics (read-only)  *(L138)*
- §4 search.* — search  *(L165)*
- §5 project.* — project lifecycle  *(L173)*
- §6 batch.* — composite operations (single undo unit)  *(L181)*
- §7 Annotations (per tool)  *(L189)*
- §8 Result shape  *(L199)*
- §9 Example: creating a traced requirement → usecase  *(L208)*
- §10 Open questions (resolve before STABLE)  *(L231)*

#### `docs/mcp/03-resources.md` — Readable model resources exposed to agents.

- §1 URI scheme  *(L14)*
- §2 Resource list  *(L42)*
- §3 Resource representation  *(L60)*
- §4 Subscriptions & change notifications  *(L124)*
- §5 Resource size limits  *(L141)*
- §6 Open questions (resolve before STABLE)  *(L148)*

#### `docs/mcp/04-prompts.md` — Predefined prompts for common modeling tasks.

- §1 Design intent  *(L14)*
- §2 Prompt catalog  *(L25)*
- §3 Example: `deriveUseCases` prompt body  *(L61)*
- §4 Localization  *(L85)*
- §5 Open questions (resolve before STABLE)  *(L90)*

#### `docs/mcp/05-client-config.md` — How to register Smith in Claude Code, Codex, Gemini, web.

- §1 Server endpoint  *(L13)*
- §2 Claude Code  *(L32)*
- §3 Codex (OpenAI)  *(L61)*
- §4 Gemini CLI  *(L80)*
- §5 Web chat (browser)  *(L101)*
- §6 Discovery file  *(L132)*
- §7 Security checklist  *(L154)*
- §8 Open questions (resolve before STABLE)  *(L167)*


### Implementation

#### `docs/99-implementation-guide.md` — Build order, module boundaries, acceptance-criteria template.

- §1 How to implement a feature from this spec  *(L14)*
- §2 Module boundaries (from [architecture/01](./architecture/01-system-architecture.md))  *(L24)*
- §3 Suggested build order (vertical slices, not layers)  *(L39)*
- §4 Acceptance-criteria template  *(L105)*
- §5 Cross-cutting acceptance (every slice must keep these green)  *(L122)*
- §6 Anti-patterns to refuse in review  *(L132)*
- §7 Testing strategy  *(L141)*
- §8 Open questions across the spec (consolidated)  *(L155)*


## 2. Requirement ID reverse-index

Every `[REQ-<DOMAIN>-<NNN>]` → the document where it is **defined** (normative backtick
statement) + a one-line restatement. Sorted by domain (grouped by prefix, then full domain,
then number). **352 unique IDs; 351 normatively defined.** `REQ-DATA-001` is an example
placeholder in `00-README.md` and is never defined anywhere — flagged in the table.

Domains present: `MM` metamodel · `DI` diagram interchange · `REQ` requirements · `TC` test
cases · `ENUM` · `CHAIN` · `STRUC` structural · `BEHAV` behavioral ·
`ORPH`(+`ORPH-D{1..7}`, `ORPH-SUP`, `ORPH-INC`, `ORPH-MCP`) defects · `SEV` severity · `RTM` ·
`GA` graph analysis · `ARCH` architecture · `PERS` persistence · `STACK` ·
`MCP`(+`MCP-TOOLS`, `MCP-RES`, `MCP-PROMPT`, `MCP-CFG`) · `SEARCH` · `QA` quality · `DS` design
system · `UI`(+30 `UI-*` sub-domains).

> **Cross-reference convention:** an ID defined here is referenced *unbracketed*
> (e.g. `REQ-MM-001`) elsewhere in the spec. The **Defined in** column is the single
> normative source; bracketed occurrences elsewhere are rare.

| ID | Defined in | One-line |
|----|-----------|----------|
| `REQ-ARCH-001` | `architecture/01-system-architecture.md` §L15 | Smith runs as a single OS process (P10). Inside it: |
| `REQ-ARCH-002` | `architecture/01-system-architecture.md` §L76 | The model API is single-writer: all mutations serialize through one |
| `REQ-ARCH-003` | `architecture/01-system-architecture.md` §L80 | Two sources of mutation: the UI (via Tauri commands) and MCP clients (via |
| `REQ-ARCH-004` | `architecture/01-system-architecture.md` §L84 | Long-running analyses (full transitive closure, centrality, large RTM) run |
| `REQ-ARCH-005` | `architecture/01-system-architecture.md` §L91 | Smith's Rust code is organized into crates with one-way dependencies: |
| `REQ-ARCH-006` | `architecture/01-system-architecture.md` §L103 | Dependencies are strictly one-way (core ← store ← model ← analysis ← mcp/bridge ← app). No cyclic crate dependencies. smith-core has no IO dependen… |
| `REQ-ARCH-007` | `architecture/01-system-architecture.md` §L118 | Steps 2–5 are one atomic unit (the command). Step 4's SQLite transaction commits or rolls back atomically with the in-memory projection update — th… |
| `REQ-ARCH-008` | `architecture/01-system-architecture.md` §L128 | Reads never block on the writer for long (RwLock; writer holds briefly). Render-critical reads are snapshot-consistent. |
| `REQ-ARCH-009` | `architecture/01-system-architecture.md` §L132 | smith-model emits a change event stream (tokio broadcast) with: Affected ElementIds (field changes). |
| `REQ-ARCH-010` | `architecture/01-system-architecture.md` §L138 | Two subscribers: smith-ui-bridge → forwards to the webview (Tauri events) for re-render. |
| `REQ-ARCH-011` | `architecture/01-system-architecture.md` §L142 | Change events are coalesced within a single command (one logical mutation = one event bundle), so a batch that touches 100 elements emits one bundl… |
| `REQ-ARCH-012` | `architecture/01-system-architecture.md` §L146 | On launch: 1. Parse CLI args / open the recent project. |
| `REQ-ARCH-013` | `architecture/01-system-architecture.md` §L154 | Startup MUST complete in < 1 s for a 10⁴-element model on a 2020 laptop (cold cache). |
| `REQ-ARCH-014` | `architecture/01-system-architecture.md` §L158 | On quit: 1. Flush any pending writes to SQLite (WAL checkpoint). |
| `REQ-ARCH-015` | `architecture/01-system-architecture.md` §L164 | Crash recovery: SQLite WAL guarantees durability of committed transactions. On next launch, PRAGMA integrity_check + a projection-rehydration sanit… |
| `REQ-ARCH-016` | `architecture/01-system-architecture.md` §L168 | Smith reads settings from (in priority order): CLI args → project-local .smith SQLite file (ui_state table — see [architecture/03](./03-persistence… |
| `REQ-ARCH-017` | `architecture/01-system-architecture.md` §L172 | smith-core/smith-model: thiserror-typed errors (machine-handleable). smith-app/smith-ui-bridge: anyhow with context. MCP tools map model errors… |
| `REQ-ARCH-018` | `architecture/01-system-architecture.md` §L174 | No panic crosses the model API boundary: all fallible operations return Result. Panics in analysis (bugs) are caught at the tool/command boundary and… |
| `REQ-ARCH-019` | `architecture/01-system-architecture.md` §L178 | tracing structured logs. Level configurable via MCP logging/setLevel. Default INFO; DEBUG for model mutations; TRACE for graph-traversal steps (off… |
| `REQ-BEHAV-001` | `uml-model/03-behavioral-diagrams.md` §L17 | Same constraint rule: a diagram's kind determines allowed element/edge |
| `REQ-BEHAV-002` | `uml-model/03-behavioral-diagrams.md` §L36 | UseCases render as ellipses; actors as stick figures — per |
| `REQ-BEHAV-003` | `uml-model/03-behavioral-diagrams.md` §L40 | «include»/«extend» are modeled as Dependencies with the corresponding |
| `REQ-BEHAV-004` | `uml-model/03-behavioral-diagrams.md` §L66 | Activity shapes follow [research/04](../research/04-uml-visual-notation.md) |
| `REQ-BEHAV-005` | `uml-model/03-behavioral-diagrams.md` §L70 | Swimlanes (ActivityPartitions) are container shapes on the diagram; nodes |
| `REQ-BEHAV-006` | `uml-model/03-behavioral-diagrams.md` §L94 | Message direction MUST be horizontal or downward (time increases down) — |
| `REQ-BEHAV-007` | `uml-model/03-behavioral-diagrams.md` §L98 | Message sorts render with distinct arrowheads per |
| `REQ-BEHAV-008` | `uml-model/03-behavioral-diagrams.md` §L103 | Combined fragment operators (alt, opt, loop, par, etc.) are a |
| `REQ-BEHAV-009` | `uml-model/03-behavioral-diagrams.md` §L126 | States render as rounded rectangles; pseudostates per |
| `REQ-BEHAV-010` | `uml-model/03-behavioral-diagrams.md` §L130 | Composite states contain regions; regions separated by dashed lines. A |
| `REQ-BEHAV-011` | `uml-model/03-behavioral-diagrams.md` §L141 | Communication diagrams use sequence numbers (not vertical position) for |
| `REQ-BEHAV-012` | `uml-model/03-behavioral-diagrams.md` §L152 | Time increases left→right (transposed vs. sequence) — |
| `REQ-BEHAV-013` | `uml-model/03-behavioral-diagrams.md` §L164 | Interaction Overview diagrams MUST visually distinguish themselves from |
| `REQ-BEHAV-014` | `uml-model/03-behavioral-diagrams.md` §L171 | Same as REQ-STRUC-011/012/013: elements exist in the model first; shape |
| `REQ-BEHAV-015` | `uml-model/03-behavioral-diagrams.md` §L174 | Behavioral elements (Activities, Interactions) are model elements with |
| `REQ-BEHAV-016` | `uml-model/03-behavioral-diagrams.md` §L181 | Behavioral diagrams are the middle of the canonical chain: UseCase diagrams show «satisfy» (UseCase → Requirement) — level 1. |
| `REQ-CHAIN-001` | `uml-model/05-traceability-chain.md` §L56 | The chain is a directed acyclic graph (DAG) over the trace graph. |
| `REQ-CHAIN-002` | `uml-model/05-traceability-chain.md` §L59 | Levels are NOT a hard constraint on element kinds — an Activity may |
| `REQ-CHAIN-003` | `uml-model/05-traceability-chain.md` §L66 | Forward trace from a Requirement = all elements reachable by following |
| `REQ-CHAIN-004` | `uml-model/05-traceability-chain.md` §L73 | Backward trace from a Class/Operation or TestCase = all elements reachable |
| `REQ-CHAIN-005` | `uml-model/05-traceability-chain.md` §L94 | The completeness state of every Requirement MUST be computable on demand |
| `REQ-CHAIN-006` | `uml-model/05-traceability-chain.md` §L97 | The transition between states is driven by adding/removing trace |
| `REQ-CHAIN-007` | `uml-model/05-traceability-chain.md` §L112 | Trace relations MAY be shown on any diagram as a dashed dependency arrow |
| `REQ-CHAIN-008` | `uml-model/05-traceability-chain.md` §L119 | Selecting any element, the inspector MUST offer: "Trace forward" → opens the next-level diagram(s) this element participates in. |
| `REQ-CHAIN-009` | `uml-model/05-traceability-chain.md` §L124 | The MCP trace.forward / trace.backward tools MUST return the same |
| `REQ-DATA-001` | `00-README.md` §L124 | **⚠ example-only placeholder — never normatively defined.** Appears only as `e.g. [REQ-DATA-001]` in README §Conventions. No statement to implement. |
| `REQ-DI-001` | `uml-model/06-diagram-interchange.md` §L16 | A diagram owns NO model elements. A diagram owns only view references |
| `REQ-DI-002` | `uml-model/06-diagram-interchange.md` §L19 | Deleting a diagram MUST NOT delete any model element. It deletes only the |
| `REQ-DI-003` | `uml-model/06-diagram-interchange.md` §L22 | Reparenting an element (moving it to another package) MUST NOT change its |
| `REQ-DI-004` | `uml-model/06-diagram-interchange.md` §L25 | Renaming an element MUST update every diagram that references it, because the |
| `REQ-DI-005` | `uml-model/06-diagram-interchange.md` §L42 | A Diagram is itself an element in the ownership tree (owned by a Package). |
| `REQ-DI-006` | `uml-model/06-diagram-interchange.md` §L45 | A diagram has exactly one kind. The kind constrains which element kinds |
| `REQ-DI-007` | `uml-model/06-diagram-interchange.md` §L72 | modelElementId references the model element. The shape stores NO model |
| `REQ-DI-008` | `uml-model/06-diagram-interchange.md` §L75 | The same model element MAY appear as multiple shapes on the same diagram |
| `REQ-DI-009` | `uml-model/06-diagram-interchange.md` §L102 | An edge view references BOTH a relationship (model) and two shapes (view). |
| `REQ-DI-010` | `uml-model/06-diagram-interchange.md` §L106 | A model relationship MAY be shown on multiple diagrams via multiple edge |
| `REQ-DI-011` | `uml-model/06-diagram-interchange.md` §L115 | Canvas state is per-diagram and persisted, so reopening a diagram restores |
| `REQ-DI-012` | `uml-model/06-diagram-interchange.md` §L129 | Presentation hints are the ONLY place color/style live on a shape. They are |
| `REQ-DI-013` | `uml-model/06-diagram-interchange.md` §L136 | Canvas coordinates are in device-independent units (1 unit = 1 logical |
| `REQ-DI-014` | `uml-model/06-diagram-interchange.md` §L139 | The origin (0,0) is the canvas origin; the canvas is infinite in all four |
| `REQ-DI-015` | `uml-model/06-diagram-interchange.md` §L146 | Smith MUST maintain a derived coverage index: for every model element, |
| `REQ-DI-016` | `uml-model/06-diagram-interchange.md` §L149 | An element with an empty coverage set is an uncovered element (quality |
| `REQ-DI-017` | `uml-model/06-diagram-interchange.md` §L154 | The coverage index is derived from view references and updated incrementally |
| `REQ-DI-018` | `uml-model/06-diagram-interchange.md` §L159 | Deleting a model element MUST cascade-delete all view references to it |
| `REQ-DI-019` | `uml-model/06-diagram-interchange.md` §L162 | Reparenting a relationship (changing its owner) MUST NOT affect its edge |
| `REQ-DS-001` | `ui-ux/01-design-system.md` §L22 | The design system MUST be expressed as a set of named tokens (not raw |
| `REQ-DS-002` | `ui-ux/01-design-system.md` §L32 | Color usage rules: Meaning never depends on color alone (WCAG 2.1 SC 1.4.1; UML helps — line style and |
| `REQ-DS-003` | `ui-ux/01-design-system.md` §L44 | Smith defines tonal roles (each with light + dark values, all ≥ WCAG AA): |
| `REQ-DS-004` | `ui-ux/01-design-system.md` §L66 | Category tints are low-saturation, pastel-range fills/borders (not vivid), |
| `REQ-DS-005` | `ui-ux/01-design-system.md` §L72 | Contrast minimums (verified for BOTH light and dark themes): Text on its background: ≥ 4.5:1 (SC 1.4.3). |
| `REQ-DS-006` | `ui-ux/01-design-system.md` §L79 | The design system MUST include a contrast-verification test (automated, in the |
| `REQ-DS-007` | `ui-ux/01-design-system.md` §L83 | Categorical hues (structure/behavior/requirement/test, severity) MUST be |
| `REQ-DS-008` | `ui-ux/01-design-system.md` §L89 | Typography uses a single type family for UI and canvas (a neutral humanist |
| `REQ-DS-009` | `ui-ux/01-design-system.md` §L104 | Element-name font weight: regular for concrete, italic for abstract (UML |
| `REQ-DS-010` | `ui-ux/01-design-system.md` §L107 | Canvas text MUST be crisp at all zoom levels — render via Canvas 2D fillText |
| `REQ-DS-011` | `ui-ux/01-design-system.md` §L116 | Element internals follow the grid: compartment padding = space.2 (8px); |
| `REQ-DS-012` | `ui-ux/01-design-system.md` §L121 | Stroke weights: Element border: 1.5px (≥ 3:1 contrast against fill). |
| `REQ-DS-013` | `ui-ux/01-design-system.md` §L128 | Corner radius: 6px for element rectangles (class, component); 12px for |
| `REQ-DS-014` | `ui-ux/01-design-system.md` §L132 | Shadow: subtle, single-direction (top-down light), small blur. Default |
| `REQ-DS-015` | `ui-ux/01-design-system.md` §L138 | Motion principles: Predictable and short. Transitions 120–200ms, ease-out. No bounce, no spring on |
| `REQ-DS-016` | `ui-ux/01-design-system.md` §L149 | Smith ships light and dark themes, auto-switching with the OS by |
| `REQ-DS-017` | `ui-ux/01-design-system.md` §L152 | The token system MUST be data-driven (JSON/TS) so custom themes are a future |
| `REQ-DS-018` | `ui-ux/01-design-system.md` §L157 | Icons: a single consistent icon set (e.g., Lucide or Phosphor — both MIT, |
| `REQ-DS-019` | `ui-ux/01-design-system.md` §L161 | Tool-bar icons have text tooltips (≥ font.caption); no icon-only actions |
| `REQ-DS-020` | `ui-ux/01-design-system.md` §L166 | Canvas layer order (back to front): grid → connectors/edges → element shapes → |
| `REQ-DS-021` | `ui-ux/01-design-system.md` §L172 | Smith MUST meet WCAG 2.1 AA for the chrome (panels, menus, inspector): keyboard navigation, visible focus, ARIA labels on icon buttons, sufficient… |
| `REQ-DS-022` | `ui-ux/01-design-system.md` §L176 | The canvas itself (a <canvas>) cannot be screen-reader-navigated natively; |
| `REQ-ENUM-001` | `uml-model/04-requirements-and-tests.md` §L189 | Persisting an element with an out-of-range enum value MUST be rejected at |
| `REQ-GA-001` | `analytics/02-adjacency-and-graph-analysis.md` §L28 | Each analysis tool MUST declare which graph it operates on. The default is the |
| `REQ-GA-002` | `analytics/02-adjacency-and-graph-analysis.md` §L33 | Smith MUST compute and render an adjacency matrix view for any selected |
| `REQ-GA-003` | `analytics/02-adjacency-and-graph-analysis.md` §L42 | The adjacency matrix MUST be available as an MCP tool |
| `REQ-GA-004` | `analytics/02-adjacency-and-graph-analysis.md` §L48 | Smith MUST compute reachability ("is there a path from A to B?") and |
| `REQ-GA-005` | `analytics/02-adjacency-and-graph-analysis.md` §L58 | Reachability is the engine behind: RTM coverage state (requirement → Class/Operation path). |
| `REQ-GA-006` | `analytics/02-adjacency-and-graph-analysis.md` §L66 | Smith MUST compute a topological ordering of the trace graph (or any DAG |
| `REQ-GA-007` | `analytics/02-adjacency-and-graph-analysis.md` §L75 | If the graph has a cycle, topological sort MUST return the detected cycle as a |
| `REQ-GA-008` | `analytics/02-adjacency-and-graph-analysis.md` §L80 | Smith MUST detect cycles in the trace graph continuously (incrementally on |
| `REQ-GA-009` | `analytics/02-adjacency-and-graph-analysis.md` §L89 | Smith MUST compute SCCs (Tarjan's or Kosaraju's) to identify |
| `REQ-GA-010` | `analytics/02-adjacency-and-graph-analysis.md` §L100 | Smith MUST compute the shortest path between two elements: Unweighted (BFS) for "minimum number of hops". Reference: |
| `REQ-GA-011` | `analytics/02-adjacency-and-graph-analysis.md` §L112 | Smith MUST compute betweenness centrality and closeness centrality as |
| `REQ-GA-012` | `analytics/02-adjacency-and-graph-analysis.md` §L121 | Centrality is informational (LOW severity); it is not a defect. The UI shows |
| `REQ-GA-013` | `analytics/02-adjacency-and-graph-analysis.md` §L126 | Smith MUST compute weakly connected components of the trace graph. Reference: [Connected… |
| `REQ-GA-014` | `analytics/02-adjacency-and-graph-analysis.md` §L134 | Smith MAY compute a dominator tree for Activity control-flow analysis. |
| `REQ-GA-015` | `analytics/02-adjacency-and-graph-analysis.md` §L142 | For models under 10⁵ elements, all on-demand analyses (except full |
| `REQ-MCP-001` | `mcp/01-server-design.md` §L20 | Smith MUST implement the Streamable HTTP transport (MCP spec |
| `REQ-MCP-002` | `mcp/01-server-design.md` §L25 | The server MUST bind to 127.0.0.1 (loopback) only. It MUST NOT bind to |
| `REQ-MCP-003` | `mcp/01-server-design.md` §L29 | The port is OS-assigned ephemeral by default, written to the discovery file |
| `REQ-MCP-004` | `mcp/01-server-design.md` §L33 | The server MUST speak JSON-RPC 2.0 over the HTTP transport, per the MCP |
| `REQ-MCP-005` | `mcp/01-server-design.md` §L44 | The server MUST support session resumption via the Mcp-Session-Id |
| `REQ-MCP-006` | `mcp/01-server-design.md` §L47 | The server MUST emit notifications/resources/updated on the SSE stream |
| `REQ-MCP-007` | `mcp/01-server-design.md` §L67 | Smith MUST implement tools/list, tools/call, resources/list, |
| `REQ-MCP-008` | `mcp/01-server-design.md` §L71 | Smith SHOULD implement logging/setLevel and emit structured log |
| `REQ-MCP-009` | `mcp/01-server-design.md` §L83 | Multiple concurrent sessions MUST be supported (Claude Code and a web chat |
| `REQ-MCP-010` | `mcp/01-server-design.md` §L87 | Every model mutation from a tool call MUST be attributable to the session |
| `REQ-MCP-011` | `mcp/01-server-design.md` §L111 | Smith MUST write this file on server start and MUST remove it on clean |
| `REQ-MCP-012` | `mcp/01-server-design.md` §L114 | Smith MUST regenerate the file if the project changes (open a different |
| `REQ-MCP-013` | `mcp/01-server-design.md` §L119 | The server MUST accept an optional bearer token (Authorization: Bearer |
| `REQ-MCP-014` | `mcp/01-server-design.md` §L124 | For browser-based chat clients, the server MUST send CORS headers and |
| `REQ-MCP-015` | `mcp/01-server-design.md` §L132 | The server MUST NOT expose filesystem-escape or shell-execution tools. The |
| `REQ-MCP-016` | `mcp/01-server-design.md` §L149 | Every tool error MUST include a human-readable message and a data |
| `REQ-MCP-017` | `mcp/01-server-design.md` §L154 | Every model-mutating tool call pushes one command onto the shared undo |
| `REQ-MCP-018` | `mcp/01-server-design.md` §L158 | The batch.undo and batch.redo tools MUST be exposed so an agent can |
| `REQ-MCP-019` | `mcp/01-server-design.md` §L164 | The model API MUST be single-writer (all mutations serialize through one |
| `REQ-MCP-020` | `mcp/01-server-design.md` §L168 | A long-running analysis tool (e.g. full RTM on a huge model) MUST run off |
| `REQ-MCP-CFG-001` | `mcp/05-client-config.md` §L27 | Smith's UI MUST display, on demand: The full endpoint URL. |
| `REQ-MCP-CFG-002` | `mcp/05-client-config.md` §L58 | The type field MUST be "http" (or alias "streamable-http"). Omitting |
| `REQ-MCP-CFG-003` | `mcp/05-client-config.md` §L77 | Codex supports Streamable HTTP only (no legacy SSE). Smith's Streamable |
| `REQ-MCP-CFG-004` | `mcp/05-client-config.md` §L98 | Gemini uses httpUrl for Streamable HTTP (NOT url, which means the |
| `REQ-MCP-CFG-005` | `mcp/05-client-config.md` §L120 | For browser clients, Smith MUST: Apply the CORS policy of [mcp/01 REQ-MCP-014](./01-server-design.md) on /mcp (permissive |
| `REQ-MCP-CFG-006` | `mcp/05-client-config.md` §L129 | Smith's own embedded chat pane (if shipped) SHOULD be served from |
| `REQ-MCP-CFG-007` | `mcp/05-client-config.md` §L151 | Clients or wrapper scripts MAY read this file to auto-discover the |
| `REQ-MCP-CFG-008` | `mcp/05-client-config.md` §L158 | Smith MUST: Bind to 127.0.0.1 only (never 0.0.0.0). |
| `REQ-MCP-PROMPT-001` | `mcp/04-prompts.md` §L21 | Every prompt MUST declare its arguments (name, required, description) |
| `REQ-MCP-PROMPT-002` | `mcp/04-prompts.md` §L79 | Prompt bodies MUST reference Smith resources and tools by their exact |
| `REQ-MCP-PROMPT-003` | `mcp/04-prompts.md` §L82 | Prompts MUST be conservative: they instruct the agent to check |
| `REQ-MCP-PROMPT-004` | `mcp/04-prompts.md` §L87 | v1 ships English prompts. The prompt catalog is data-driven (not |
| `REQ-MCP-RES-001` | `mcp/03-resources.md` §L16 | Smith defines these URI schemes: |
| `REQ-MCP-RES-002` | `mcp/03-resources.md` §L35 | Element IDs in URIs are the opaque stable ElementId strings. Agents |
| `REQ-MCP-RES-003` | `mcp/03-resources.md` §L39 | Unknown-element URIs return error -32602 (Invalid Params) with a clear |
| `REQ-MCP-RES-004` | `mcp/03-resources.md` §L44 | resources/list returns all listable resources. For element-scoped |
| `REQ-MCP-RES-005` | `mcp/03-resources.md` §L51 | Smith MUST also expose resources/templates/list with RFC 6570 URI |
| `REQ-MCP-RES-006` | `mcp/03-resources.md` §L62 | Each resource's resources/read returns JSON (mimeType: application/json) with a stable, documented shape: |
| `REQ-MCP-RES-007` | `mcp/03-resources.md` §L88 | The kind field names the metaclass; the remaining fields are |
| `REQ-MCP-RES-008` | `mcp/03-resources.md` §L126 | Agents MAY subscribe to any resource via resources/subscribe. On |
| `REQ-MCP-RES-009` | `mcp/03-resources.md` §L137 | Smith MUST NOT over-notify: if a single batch command touches 100 |
| `REQ-MCP-RES-010` | `mcp/03-resources.md` §L143 | The resources/read response for a single resource MUST stay under |
| `REQ-MCP-TOOLS-001` | `mcp/02-tools.md` §L191 | Every tool MUST declare annotations: All model.create, model.add, model.delete, model.rename, model.reparent, |
| `REQ-MCP-TOOLS-002` | `mcp/02-tools.md` §L201 | Mutating tools return { ok: true, ...ids } plus the affected |
| `REQ-MCP-TOOLS-003` | `mcp/02-tools.md` §L204 | Every mutating tool MUST emit a notifications/resources/updated for |
| `REQ-MM-001` | `uml-model/01-metamodel-core.md` §L27 | Every element MUST have a non-empty id that is unique within the project |
| `REQ-MM-002` | `uml-model/01-metamodel-core.md` §L30 | owner forms a tree. Every element (except the project root) has exactly one |
| `REQ-MM-003` | `uml-model/01-metamodel-core.md` §L43 | qualifiedName MUST be derived on demand from the ownership chain. It MUST |
| `REQ-MM-004` | `uml-model/01-metamodel-core.md` §L46 | name MAY be empty. Two siblings MAY share a name (names are not unique |
| `REQ-MM-005` | `uml-model/01-metamodel-core.md` §L56 | Adding an element to a namespace sets its owner; removing it unsets the |
| `REQ-MM-006` | `uml-model/01-metamodel-core.md` §L68 | The project root MUST be a Package with isModel = true. It owns all |
| `REQ-MM-007` | `uml-model/01-metamodel-core.md` §L71 | Packages MAY nest arbitrarily. The ownership tree has no depth limit in the |
| `REQ-MM-008` | `uml-model/01-metamodel-core.md` §L103 | Applying a stereotype to an element whose metaclass is not in the |
| `REQ-MM-009` | `uml-model/01-metamodel-core.md` §L106 | An element MAY carry multiple stereotypes, from multiple profiles. Tag value |
| `REQ-MM-010` | `uml-model/01-metamodel-core.md` §L109 | The built-in profiles (smith::traceability, smith::requirements, |
| `REQ-MM-011` | `uml-model/01-metamodel-core.md` §L123 | A comment MUST have a non-empty body. Deleting the element that owns a |
| `REQ-MM-012` | `uml-model/01-metamodel-core.md` §L143 | Most Smith relationships are binary (one source, one target). N-ary is |
| `REQ-MM-013` | `uml-model/01-metamodel-core.md` §L165 | Visibility is one of public (+), private (-), protected (#), |
| `REQ-MM-014` | `uml-model/01-metamodel-core.md` §L176 | Notation: lower..upper, abbreviated: 1 = 1..1, = 0.., 0..1, |
| `REQ-MM-015` | `uml-model/01-metamodel-core.md` §L258 | The set of kinds is closed for v1. User extension is via stereotypes, not new |
| `REQ-ORPH-001` | `analytics/03-orphan-and-coverage-analysis.md` §L23 | Smith MUST detect D1–D6 continuously (incrementally on each model mutation) |
| `REQ-ORPH-D1-001` | `analytics/03-orphan-and-coverage-analysis.md` §L35 | Detection: in-degree test on the requirement node in the trace subgraph |
| `REQ-ORPH-D1-002` | `analytics/03-orphan-and-coverage-analysis.md` §L38 | Severity: HIGH. The issue panel lists orphan requirements first. |
| `REQ-ORPH-D2-001` | `analytics/03-orphan-and-coverage-analysis.md` §L46 | Detection: out-degree test on the TestCase node for «verify» edges. O(1). |
| `REQ-ORPH-D2-002` | `analytics/03-orphan-and-coverage-analysis.md` §L48 | Severity: HIGH. |
| `REQ-ORPH-D3-001` | `analytics/03-orphan-and-coverage-analysis.md` §L59 | Detection (a): ownership-tree reachability — every element MUST be |
| `REQ-ORPH-D3-002` | `analytics/03-orphan-and-coverage-analysis.md` §L64 | Detection (b): total-degree test (in + out) across ALL relationship kinds |
| `REQ-ORPH-D3-003` | `analytics/03-orphan-and-coverage-analysis.md` §L67 | Severity: MEDIUM for isolated nodes (case b); model corruption (case a) |
| `REQ-ORPH-D3-004` | `analytics/03-orphan-and-coverage-analysis.md` §L70 | Exclusions from the isolated-node check: Package, Profile, |
| `REQ-ORPH-D4-001` | `analytics/03-orphan-and-coverage-analysis.md` §L85 | Detection: set difference — modelableElements − elementsReferencedByAnyDiagram. |
| `REQ-ORPH-D4-002` | `analytics/03-orphan-and-coverage-analysis.md` §L90 | Severity: LOW (informational). The issue panel groups uncovered elements |
| `REQ-ORPH-D4-003` | `analytics/03-orphan-and-coverage-analysis.md` §L93 | The uncovered-element check MUST be available as a filter in the model |
| `REQ-ORPH-D5-001` | `analytics/03-orphan-and-coverage-analysis.md` §L103 | Detection: DFS cycle detection (or equivalently, any SCC with >1 vertex |
| `REQ-ORPH-D5-002` | `analytics/03-orphan-and-coverage-analysis.md` §L107 | The detection MUST identify the exact cycle (list of edges) so the user |
| `REQ-ORPH-D5-003` | `analytics/03-orphan-and-coverage-analysis.md` §L110 | Severity: HIGH. Adding a trace edge that would create a cycle is rejected |
| `REQ-ORPH-D6-001` | `analytics/03-orphan-and-coverage-analysis.md` §L123 | Detection: reachability (BFS) from the requirement following |
| `REQ-ORPH-D6-002` | `analytics/03-orphan-and-coverage-analysis.md` §L128 | Severity: MEDIUM. Surfaced in the RTM as the coverage state |
| `REQ-ORPH-D6-003` | `analytics/03-orphan-and-coverage-analysis.md` §L131 | Note: a direct Requirement → Class path (skipping levels) is valid and |
| `REQ-ORPH-D7-001` | `analytics/03-orphan-and-coverage-analysis.md` §L143 | Detection: weakly connected components on the trace graph. Reference: [Connected component](https://en.wikipedia.org/wiki/Connected_component_(grap… |
| `REQ-ORPH-D7-002` | `analytics/03-orphan-and-coverage-analysis.md` §L146 | Severity: MEDIUM. Computed on demand (not continuous — graph-wide query). |
| `REQ-ORPH-INC-001` | `analytics/03-orphan-and-coverage-analysis.md` §L172 | D1, D2, D3(b), D4, D5, D6 MUST be maintained incrementally: On element creation: evaluate defects for the new element. |
| `REQ-ORPH-INC-002` | `analytics/03-orphan-and-coverage-analysis.md` §L177 | D3(a) (ownership reachability) is an invariant enforced at the model |
| `REQ-ORPH-MCP-001` | `analytics/03-orphan-and-coverage-analysis.md` §L182 | These defects MUST be exposed via MCP: analysis.issues tool — returns current defects (filterable by severity/code/package). |
| `REQ-ORPH-SUP-001` | `analytics/03-orphan-and-coverage-analysis.md` §L161 | A user MAY suppress a specific defect instance (e.g. "this uncovered |
| `REQ-ORPH-SUP-002` | `analytics/03-orphan-and-coverage-analysis.md` §L167 | Suppression is per-element-per-defect-code, not global. Suppressing D4 |
| `REQ-PERS-001` | `architecture/03-persistence.md` §L17 | SQLite is the store of record. Every committed model mutation is durable |
| `REQ-PERS-002` | `architecture/03-persistence.md` §L21 | The in-memory projection (petgraph DiGraph<ElementNode, EdgeNode>) |
| `REQ-PERS-003` | `architecture/03-persistence.md` §L25 | Rationale (per [research/03](../research/03-graph-database-options.md) §4.3): at UML scale (≤10⁵ elements) the whole graph fits in RAM; traversal i… |
| `REQ-PERS-004` | `architecture/03-persistence.md` §L32 | The schema (illustrative DDL; the implementer finalizes types/constraints): |
| `REQ-PERS-005` | `architecture/03-persistence.md` §L166 | The data JSON blobs per element/relationship hold kind-specific fields. This avoids a wide sparse table; the smith-model crate owns the… |
| `REQ-PERS-006` | `architecture/03-persistence.md` §L168 | The shape_views.model_element_id index IS the coverage index (REQ-DI-015): SELECT model_element_id FROM shape_views gives all covered elements; set… |
| `REQ-PERS-007` | `architecture/03-persistence.md` §L172 | The ownership_closure table is maintained in the same transaction as the owning edge. On: Element creation (with owner O): insert rows (anc, new,… |
| `REQ-PERS-008` | `architecture/03-persistence.md` §L177 | Closure-table maintenance MUST be atomic with the element/owner mutation (same SQLite transaction). A failure rolls back both. |
| `REQ-PERS-009` | `architecture/03-persistence.md` §L181 | Smith ships a smith_meta.schema_version and a forward-only migration runner. On open: check version, run pending migrations in a transaction, bump… |
| `REQ-PERS-010` | `architecture/03-persistence.md` §L183 | Migrations are append-only SQL/Rust files; no down-migrations (Smith is local-first; a broken migration is a bug to fix forward, not a rollback). |
| `REQ-PERS-011` | `architecture/03-persistence.md` §L187 | SQLite is opened in WAL mode (PRAGMA journal_mode=WAL), synchronous=NORMAL. This gives concurrent readers + one writer, crash-safe commits. |
| `REQ-PERS-012` | `architecture/03-persistence.md` §L189 | Smith holds a single write connection (the model API's writer). Read connections are pooled for UI/analysis/MCP queries. No long-running write tran… |
| `REQ-PERS-013` | `architecture/03-persistence.md` §L191 | Every model command = one SQLite transaction. If the transaction fails, the in-memory projection is NOT updated (they update together or not at all —… |
| `REQ-PERS-014` | `architecture/03-persistence.md` §L195 | Undo is a command pattern in smith-model (NOT SQLite-level — see [research/03](../research/03-graph-database-options.md) §7: MVCC is concurrency, not… |
| `REQ-PERS-015` | `architecture/03-persistence.md` §L197 | Command stack: Bounded depth (default 200); oldest evicted. |
| `REQ-PERS-016` | `architecture/03-persistence.md` §L204 | Undo/redo replays the inverse/forward delta through the same model API (re-validating invariants). An undo that would violate an invariant is rejec… |
| `REQ-PERS-017` | `architecture/03-persistence.md` §L208 | A .smith "file" is a single SQLite database file (with -wal and -shm sidecars during use, checkpointed on close). Extension: .smith (the file is… |
| `REQ-PERS-018` | `architecture/03-persistence.md` §L210 | The file is inspectable by any sqlite3 tool — agents and humans can run sqlite3 project.smith ".tables" to understand the model directly (agent-fri… |
| `REQ-PERS-019` | `architecture/03-persistence.md` §L212 | Smith MUST set a SQLite PRAGMA application_id to a Smith-specific magic number and verify it on open (reject non-Smith SQLite files with a clear er… |
| `REQ-PERS-020` | `architecture/03-persistence.md` §L214 | Backup: Smith writes .smith.bak (previous version) on each successful open, rotated to keep the last 3. Users can also export to XMI/JSON (see… |
| `REQ-PERS-021` | `architecture/03-persistence.md` §L218 | For a 10⁵-element model: Cold load (hydrate projection): < 1 s. |
| `REQ-QA-001` | `analytics/04-quality-rules.md` §L14 | A quality rule is a declarative predicate over the model state with metadata: |
| `REQ-QA-002` | `analytics/04-quality-rules.md` §L41 | These seven are built-in, always enabled (cannot be disabled), and shipped with |
| `REQ-QA-003` | `analytics/04-quality-rules.md` §L46 | Rules run in two modes: Incremental (D1–D6): re-evaluated for affected elements on each mutation. Cheap (degree |
| `REQ-QA-004` | `analytics/04-quality-rules.md` §L52 | Incremental evaluation scope (which elements to re-check on a mutation): Element field change → re-check that element against all rules whose appli… |
| `REQ-QA-005` | `analytics/04-quality-rules.md` §L60 | Evaluation results are stored as DefectInstance { ruleId, elementId, |
| `REQ-QA-006` | `analytics/04-quality-rules.md` §L65 | Severity ordering: HIGH > MEDIUM > LOW. The issue panel sorts HIGH first (per |
| `REQ-QA-007` | `analytics/04-quality-rules.md` §L68 | HIGH-severity defects MAY optionally block project export (configurable in |
| `REQ-QA-008` | `analytics/04-quality-rules.md` §L76 | A user MAY suppress a specific defect instance via the issue panel or MCP. |
| `REQ-QA-009` | `analytics/04-quality-rules.md` §L80 | Suppression is per-element-per-rule (not global). Suppressing D4 on element X |
| `REQ-QA-010` | `analytics/04-quality-rules.md` §L83 | Suppressions are model data (persisted in the .smith file) and travel with |
| `REQ-QA-011` | `analytics/04-quality-rules.md` §L88 | v1 ships ONLY the seven built-in rules. A user-defined rule system (declarative |
| `REQ-QA-012` | `analytics/04-quality-rules.md` §L94 | Quality state is exposed via: analysis.issues tool — filterable list of current defects (see |
| `REQ-QA-013` | `analytics/04-quality-rules.md` §L101 | Agents SHOULD check analysis.issues after any mutation to avoid introducing |
| `REQ-REQ-001` | `uml-model/04-requirements-and-tests.md` §L32 | Every Requirement MUST have a non-empty requirementId unique within the |
| `REQ-REQ-002` | `uml-model/04-requirements-and-tests.md` §L36 | text MUST be non-empty. An empty-text requirement is invalid. |
| `REQ-REQ-003` | `uml-model/04-requirements-and-tests.md` §L38 | requirementId is mutable; renaming updates all «deriveReqt» and RTM |
| `REQ-REQ-004` | `uml-model/04-requirements-and-tests.md` §L60 | Requirements MAY appear on Use Case diagrams (as the target of |
| `REQ-REQ-005` | `uml-model/04-requirements-and-tests.md` §L126 | Smith RECOMMENDS (does not mandate) a top-level Requirements package with |
| `REQ-REQ-006` | `uml-model/04-requirements-and-tests.md` §L144 | The model API and MCP tools MUST treat Requirement and TestCase as |
| `REQ-REQ-007` | `uml-model/04-requirements-and-tests.md` §L164 | Allowed shapes: Requirement — rendered as the «requirement» box (see §Requirement visual notation). |
| `REQ-REQ-008` | `uml-model/04-requirements-and-tests.md` §L169 | Allowed edges — all trace relations (see |
| `REQ-REQ-009` | `uml-model/04-requirements-and-tests.md` §L174 | Layout conventions: Tabular layout (default): one row per Requirement, with columns for satisfying |
| `REQ-RTM-001` | `analytics/01-rtm.md` §L24 | The RTM MUST be computable from: The set of Requirement elements. |
| `REQ-RTM-002` | `analytics/01-rtm.md` §L62 | Both views MUST be available as MCP analysis tools |
| `REQ-RTM-003` | `analytics/01-rtm.md` §L77 | The RTM MUST show the coverage state per requirement and MUST allow filtering |
| `REQ-RTM-004` | `analytics/01-rtm.md` §L82 | The RTM view MUST show these rollups at the top: |
| `REQ-RTM-005` | `analytics/01-rtm.md` §L105 | For models under 10⁵ elements, the full RTM MUST compute in under 500 ms |
| `REQ-RTM-006` | `analytics/01-rtm.md` §L111 | The RTM MUST support filtering by: Requirement package (ownership subtree). |
| `REQ-RTM-007` | `analytics/01-rtm.md` §L117 | The RTM MUST be exportable as CSV and as a model resource |
| `REQ-RTM-008` | `analytics/01-rtm.md` §L130 | RTM computation MUST be cycle-safe: if the trace graph has a cycle, the RTM |
| `REQ-SEARCH-001` | `architecture/05-search.md` §L16 | Smith maintains an FTS5 virtual table (elements_fts) indexing, per |
| `REQ-SEARCH-002` | `architecture/05-search.md` §L21 | The FTS index is updated incrementally on every element mutation |
| `REQ-SEARCH-003` | `architecture/05-search.md` §L25 | Tokenizer: unicode61 (Unicode-aware word splitting; case-insensitive; |
| `REQ-SEARCH-004` | `architecture/05-search.md` §L29 | Ranking: FTS5 bm25 (built-in) for relevance. Results ranked by bm25 |
| `REQ-SEARCH-005` | `architecture/05-search.md` §L32 | Query syntax: FTS5's standard query syntax (quoted phrases, prefix term, |
| `REQ-SEARCH-006` | `architecture/05-search.md` §L39 | FTS results compose with structured filters (applied in SQL after the FTS |
| `REQ-SEARCH-007` | `architecture/05-search.md` §L52 | A query without a text term returns all elements matching the structured |
| `REQ-SEARCH-008` | `architecture/05-search.md` §L79 | Graph queries operate on one of two derived graphs (relationship graph or |
| `REQ-SEARCH-009` | `architecture/05-search.md` §L86 | Graph queries accept a scope: whole model, a package subtree (via the |
| `REQ-SEARCH-010` | `architecture/05-search.md` §L92 | Smith maintains these derived indexes (all in the in-memory projection, |
| `REQ-SEARCH-011` | `architecture/05-search.md` §L103 | Indexes are derived — never stored as primary data, always rebuildable |
| `REQ-SEARCH-012` | `architecture/05-search.md` §L108 | For a 10⁵-element model: FTS query (top 50 results): < 50 ms. |
| `REQ-SEARCH-013` | `architecture/05-search.md` §L117 | Search is exposed as MCP tools (search.elements, search.relations, |
| `REQ-SEV-001` | `analytics/03-orphan-and-coverage-analysis.md` §L150 | Severities: HIGH (blocking — model is wrong or dangerously incomplete), |
| `REQ-SEV-002` | `analytics/03-orphan-and-coverage-analysis.md` §L153 | The issue panel MUST sort by severity (HIGH first) and MUST allow filtering. |
| `REQ-SEV-003` | `analytics/03-orphan-and-coverage-analysis.md` §L156 | Each defect carries a stable defectCode (D1–D7) for programmatic reference |
| `REQ-STACK-001` | `architecture/06-tech-stack.md` §L84 | The Rust core, the SQLite store, the MCP server, and the Tauri host MUST |
| `REQ-STACK-002` | `architecture/06-tech-stack.md` §L88 | The model API MUST be the single writer (all mutations serialize through |
| `REQ-STACK-003` | `architecture/06-tech-stack.md` §L92 | The MCP server MUST bind to 127.0.0.1 only, on a configurable port |
| `REQ-STACK-004` | `architecture/06-tech-stack.md` §L134 | All dependencies MUST be pinned to exact versions (= in Cargo.toml, no |
| `REQ-STRUC-001` | `uml-model/02-structural-diagrams.md` §L18 | A diagram's kind determines which element kinds may appear on it as |
| `REQ-STRUC-002` | `uml-model/02-structural-diagrams.md` §L23 | Element kinds NOT listed for a diagram kind are prohibited on it, even |
| `REQ-STRUC-003` | `uml-model/02-structural-diagrams.md` §L49 | A Class shape renders the class's compartments per |
| `REQ-STRUC-004` | `uml-model/02-structural-diagrams.md` §L54 | Associations on a class diagram show role names, multiplicities, and |
| `REQ-STRUC-005` | `uml-model/02-structural-diagrams.md` §L65 | Instance names render underlined (instanceName : TypeName); slots |
| `REQ-STRUC-006` | `uml-model/02-structural-diagrams.md` §L78 | Components render with the «component» keyword and optional component |
| `REQ-STRUC-007` | `uml-model/02-structural-diagrams.md` §L90 | Parts render as boxes inside the classifier's internal-structure |
| `REQ-STRUC-008` | `uml-model/02-structural-diagrams.md` §L103 | A package diagram MAY show elements inside packages (nested shapes), but |
| `REQ-STRUC-009` | `uml-model/02-structural-diagrams.md` §L116 | Nodes render as 3D perspective boxes (per |
| `REQ-STRUC-010` | `uml-model/02-structural-diagrams.md` §L128 | Profile diagrams are the only place Smith users DEFINE stereotypes. On |
| `REQ-STRUC-011` | `uml-model/02-structural-diagrams.md` §L133 | All element kinds must already exist in the model (be owned by a package) |
| `REQ-STRUC-012` | `uml-model/02-structural-diagrams.md` §L138 | Deleting a shape from a structural diagram removes ONLY the view reference |
| `REQ-STRUC-013` | `uml-model/02-structural-diagrams.md` §L142 | A shape's bounds, bend points, and label positions are view data |
| `REQ-STRUC-014` | `uml-model/02-structural-diagrams.md` §L148 | Structural diagrams participate in the traceability chain at the |
| `REQ-TC-001` | `uml-model/04-requirements-and-tests.md` §L92 | Every TestCase MUST have a non-empty testCaseId unique within the project. |
| `REQ-TC-002` | `uml-model/04-requirements-and-tests.md` §L94 | A TestCase with no steps AND no specification is a draft |
| `REQ-TC-003` | `uml-model/04-requirements-and-tests.md` §L99 | A TestCase MUST have ≥1 outgoing «verify» relation to a Requirement. |
| `REQ-UI-001` | `ui-ux/02-canvas-and-interaction.md` §L18 | Smith has a tool palette with these tools (keyboard shortcuts in parens): |
| `REQ-UI-002` | `ui-ux/02-canvas-and-interaction.md` §L29 | The active tool is a single global state. Pressing Esc returns to Select. |
| `REQ-UI-003` | `ui-ux/02-canvas-and-interaction.md` §L32 | The palette shows ONLY the element kinds valid for the current diagram kind |
| `REQ-UI-004` | `ui-ux/02-canvas-and-interaction.md` §L40 | Selection is a set of view references (shapes/edges) on the current |
| `REQ-UI-005` | `ui-ux/02-canvas-and-interaction.md` §L43 | Click on a shape → selects that shape (single selection; replaces previous). |
| `REQ-UI-006` | `ui-ux/02-canvas-and-interaction.md` §L47 | Ownership-aware selection (user's explicit concern — elements belong to |
| `REQ-UI-007` | `ui-ux/02-canvas-and-interaction.md` §L56 | Right-click on a selection opens a context menu with actions valid for the |
| `REQ-UI-ADJ-001` | `ui-ux/04-search-and-analysis-ui.md` §L95 | A heat-map view of the adjacency matrix (see |
| `REQ-UI-ADJ-002` | `ui-ux/04-search-and-analysis-ui.md` §L99 | Cells colored by edge presence/count (monochrome intensity scale — NOT |
| `REQ-UI-ADJ-003` | `ui-ux/04-search-and-analysis-ui.md` §L103 | For large scopes (>200 elements), the view paginates or offers to filter |
| `REQ-UI-CLIP-001` | `ui-ux/02-canvas-and-interaction.md` §L217 | Copy/paste (Cmd/Ctrl+C/V) copies selected elements (deep — with |
| `REQ-UI-CLIP-002` | `ui-ux/02-canvas-and-interaction.md` §L223 | Cut (Cmd/Ctrl+X) = copy + delete. Delete (Del) removes selected |
| `REQ-UI-CONN-001` | `ui-ux/02-canvas-and-interaction.md` §L150 | Two ways to create a relationship: 1. Connection handle — when a shape is selected, a small handle appears on each edge (midpoint). Drag from a han… |
| `REQ-UI-CONN-002` | `ui-ux/02-canvas-and-interaction.md` §L158 | On drop, Smith validates the relationship against the metamodel (source |
| `REQ-UI-CONN-003` | `ui-ux/02-canvas-and-interaction.md` §L165 | New edges get orthogonal routing by default (see |
| `REQ-UI-DASH-001` | `ui-ux/04-search-and-analysis-ui.md` §L140 | An optional project dashboard (menu → View → Dashboard): model stats |
| `REQ-UI-DND-001` | `ui-ux/03-inspector-and-explorer.md` §L139 | Supported drags: Explorer element → canvas: adds a view reference (shape) for that element on the target |
| `REQ-UI-EDIT-001` | `ui-ux/02-canvas-and-interaction.md` §L171 | Double-click a shape → inline edit the element name (Enter commits, Esc |
| `REQ-UI-EDIT-002` | `ui-ux/02-canvas-and-interaction.md` §L175 | For richer editing (multi-field, stereotypes, tagged values), use the |
| `REQ-UI-EXP-001` | `ui-ux/03-inspector-and-explorer.md` §L45 | Tree nodes: Packages — folder icon; expandable; show child count. |
| `REQ-UI-EXP-002` | `ui-ux/03-inspector-and-explorer.md` §L52 | Tree features: Expand/collapse all, per package. |
| `REQ-UI-EXP-003` | `ui-ux/03-inspector-and-explorer.md` §L64 | Selection in the explorer syncs with the canvas: selecting an element in |
| `REQ-UI-EXP-004` | `ui-ux/03-inspector-and-explorer.md` §L68 | The explorer is DOM-based (React), making it the accessible view of the |
| `REQ-UI-FRAME-001` | `ui-ux/02-canvas-and-interaction.md` §L228 | Each diagram MAY render a UML frame (per |
| `REQ-UI-GA-001` | `ui-ux/04-search-and-analysis-ui.md` §L108 | On-demand analysis dialogs (opened from the inspector or menu), each |
| `REQ-UI-GA-002` | `ui-ux/04-search-and-analysis-ui.md` §L121 | Long-running analyses (full transitive closure, centrality) show a progress |
| `REQ-UI-GIZMO-001` | `ui-ux/02-canvas-and-interaction.md` §L67 | When the Select tool is active and the user clicks empty canvas |
| `REQ-UI-GIZMO-002` | `ui-ux/02-canvas-and-interaction.md` §L71 | The gizmo does NOT appear if: A drag occurs (that's a marquee). |
| `REQ-UI-GIZMO-003` | `ui-ux/02-canvas-and-interaction.md` §L77 | The gizmo is a radial (pie) menu with a center hub and 4–8 slots |
| `REQ-UI-GIZMO-004` | `ui-ux/02-canvas-and-interaction.md` §L92 | Slots are context-determined by the current diagram kind and the |
| `REQ-UI-GIZMO-005` | `ui-ux/02-canvas-and-interaction.md` §L97 | Default slot allocation (the top actions for each diagram kind). Each |
| `REQ-UI-GIZMO-006` | `ui-ux/02-canvas-and-interaction.md` §L112 | The "More…" slot reveals the full element-kind list for the diagram, |
| `REQ-UI-GIZMO-007` | `ui-ux/02-canvas-and-interaction.md` §L116 | The center hub shows the most recent element kind created on this |
| `REQ-UI-GIZMO-008` | `ui-ux/02-canvas-and-interaction.md` §L121 | The gizmo accepts both click (click a slot) and gesture |
| `REQ-UI-GIZMO-009` | `ui-ux/02-canvas-and-interaction.md` §L125 | Keyboard: while the gizmo is open, number keys 1–8 select slots |
| `REQ-UI-GIZMO-010` | `ui-ux/02-canvas-and-interaction.md` §L128 | The gizmo auto-dismisses on: selection of a slot (after action), Esc, |
| `REQ-UI-GIZMO-011` | `ui-ux/02-canvas-and-interaction.md` §L132 | Gizmo visual: Translucent dark chip on the canvas (works in both light/dark themes), backdrop-blur. |
| `REQ-UI-GIZMO-012` | `ui-ux/02-canvas-and-interaction.md` §L140 | The gizmo is rendered ABOVE all canvas elements (top z-layer, per |
| `REQ-UI-GIZMO-013` | `ui-ux/02-canvas-and-interaction.md` §L144 | The gizmo MUST be keyboard-operable: when it opens, focus moves to the |
| `REQ-UI-INS-001` | `ui-ux/03-inspector-and-explorer.md` §L77 | When a single element is selected, the inspector shows sections |
| `REQ-UI-INS-002` | `ui-ux/03-inspector-and-explorer.md` §L93 | When a relationship is selected, the inspector shows: relationship |
| `REQ-UI-INS-003` | `ui-ux/03-inspector-and-explorer.md` §L97 | When a diagram is selected (no element), the inspector shows: diagram |
| `REQ-UI-INS-004` | `ui-ux/03-inspector-and-explorer.md` §L101 | When nothing is selected, the inspector shows project info (name, |
| `REQ-UI-INS-005` | `ui-ux/03-inspector-and-explorer.md` §L104 | Editing in the inspector goes through the model API (undoable, syncs to |
| `REQ-UI-INS-006` | `ui-ux/03-inspector-and-explorer.md` §L108 | Multi-selection: the inspector shows common properties only (kind, |
| `REQ-UI-ISSUE-001` | `ui-ux/04-search-and-analysis-ui.md` §L49 | The issue panel lists all current defects (D1–D7, see |
| `REQ-UI-ISSUE-002` | `ui-ux/04-search-and-analysis-ui.md` §L58 | Issue panel features: Sort by severity (default), code, or element. |
| `REQ-UI-ISSUE-003` | `ui-ux/04-search-and-analysis-ui.md` §L65 | The defect list updates live as the user edits (incremental — see |
| `REQ-UI-LAY-001` | `ui-ux/03-inspector-and-explorer.md` §L17 | Default three-pane layout: |
| `REQ-UI-LAY-002` | `ui-ux/03-inspector-and-explorer.md` §L33 | Panels are collapsible and resizable (drag the dividers). State |
| `REQ-UI-LAY-003` | `ui-ux/03-inspector-and-explorer.md` §L36 | The layout MUST be responsive to window size: at narrow widths, panels |
| `REQ-UI-MENU-001` | `ui-ux/03-inspector-and-explorer.md` §L126 | Standard menus: File (new/open/save/save as/export/recent), Edit |
| `REQ-UI-MOVE-001` | `ui-ux/02-canvas-and-interaction.md` §L180 | Dragging a shape moves its view-reference bounds (NOT the model element |
| `REQ-UI-MOVE-002` | `ui-ux/02-canvas-and-interaction.md` §L184 | Resize handles (8, on corners/edges of a selected shape) resize the |
| `REQ-UI-MOVE-003` | `ui-ux/02-canvas-and-interaction.md` §L188 | Snapping: shapes snap to the 8px grid by default; connectors snap to |
| `REQ-UI-NAV-001` | `ui-ux/02-canvas-and-interaction.md` §L194 | Pan: Space-drag, middle-mouse drag, or two-finger trackpad scroll. |
| `REQ-UI-NAV-002` | `ui-ux/02-canvas-and-interaction.md` §L198 | Zoom range: 10%–400% (per |
| `REQ-UI-NAV-003` | `ui-ux/02-canvas-and-interaction.md` §L203 | A minimap (toggleable, corner of canvas) shows the whole diagram with |
| `REQ-UI-PERF-001` | `ui-ux/02-canvas-and-interaction.md` §L236 | At ≤ 500 shapes + ≤ 1000 edges on one diagram, pan/zoom/edit MUST stay |
| `REQ-UI-PERSIST-001` | `ui-ux/03-inspector-and-explorer.md` §L147 | Smith persists per-project UI state: open diagrams, panel |
| `REQ-UI-RTM-001` | `ui-ux/04-search-and-analysis-ui.md` §L74 | The RTM view has two tabs: Forward (requirement-centric): one row per Requirement; columns per |
| `REQ-UI-RTM-002` | `ui-ux/04-search-and-analysis-ui.md` §L80 | Aggregate metrics header: total requirements, % satisfied, % verified, % |
| `REQ-UI-RTM-003` | `ui-ux/04-search-and-analysis-ui.md` §L83 | Filters: package subtree, category, priority, status, coverage state, |
| `REQ-UI-RTM-004` | `ui-ux/04-search-and-analysis-ui.md` §L86 | Cells are interactive: clicking a satisfying-artifact cell navigates to |
| `REQ-UI-RTM-005` | `ui-ux/04-search-and-analysis-ui.md` §L90 | Export: CSV, and as a model resource (smith://analysis/rtm, |
| `REQ-UI-SRCH-001` | `ui-ux/04-search-and-analysis-ui.md` §L18 | Search is triggered by Cmd/Ctrl+F (or the toolbar search). A |
| `REQ-UI-SRCH-002` | `ui-ux/04-search-and-analysis-ui.md` §L22 | Search matches: Element names (FTS5, ranked). |
| `REQ-UI-SRCH-003` | `ui-ux/04-search-and-analysis-ui.md` §L30 | Result rows show: kind icon, name, qualified name (muted), matched |
| `REQ-UI-SRCH-004` | `ui-ux/04-search-and-analysis-ui.md` §L34 | Filters (chips above results): element kind, owning package, stereotype |
| `REQ-UI-SRCH-005` | `ui-ux/04-search-and-analysis-ui.md` §L37 | An advanced search mode (toggle) exposes structured queries: "all |
| `REQ-UI-SRCH-006` | `ui-ux/04-search-and-analysis-ui.md` §L42 | Search is also exposed as the MCP tool search.elements for agents |
| `REQ-UI-STATUS-001` | `ui-ux/03-inspector-and-explorer.md` §L133 | Bottom status bar: current diagram + kind, cursor coordinates (canvas), |
| `REQ-UI-TB-001` | `ui-ux/03-inspector-and-explorer.md` §L114 | Top toolbar (left to right): Diagram tabs (open diagrams; switch; close). |
| `REQ-UI-TRACE-001` | `ui-ux/04-search-and-analysis-ui.md` §L127 | "Show full chain" (from an element, per |
| `REQ-UI-TRACE-002` | `ui-ux/04-search-and-analysis-ui.md` §L134 | The trace overview uses level-based layout (columns or rows per chain |
| `REQ-UI-UNDO-001` | `ui-ux/02-canvas-and-interaction.md` §L208 | Cmd/Ctrl+Z / Shift+Z undo/redo through the shared command stack |
| `REQ-UI-UNDO-002` | `ui-ux/02-canvas-and-interaction.md` §L212 | The undo stack persists across diagram switches (project-wide), not per |
## 3. Invariants index

| ID | Defined in | Statement |
|----|-----------|-----------|
| `INV-MM-001` | `uml-model/01-metamodel-core.md` §L263 | Ownership tree is acyclic and connected (single root). |
| `INV-MM-002` | `uml-model/01-metamodel-core.md` §L264 | No two elements share an id. |
| `INV-MM-003` | `uml-model/01-metamodel-core.md` §L265 | Every stereotype application targets a metaclass in the stereotype's |
| `INV-MM-004` | `uml-model/01-metamodel-core.md` §L267 | Every relationship's sources and targets reference existing elements. |
| `INV-MM-005` | `uml-model/01-metamodel-core.md` §L268 | qualifiedName derivation terminates (no ownership cycle). |

All invariants are defined in `uml-model/01-metamodel-core.md` §Invariants and MUST hold
after every mutation (enforced by the model API). `INV-MM-001` (ownership acyclicity) is
also the detection basis for defect D3(a).

## 4. Defect code index (D1–D7)

Defects defined in `analytics/03-orphan-and-coverage-analysis.md` §Defect taxonomy. D1–D6 are
maintained continuously (incremental on each mutation); D7 is computed on demand.

| Code | Name | Severity | Continuous? | Defined in | Detection algorithm |
|------|------|----------|-------------|-----------|---------------------|
| `D1` | Orphan requirement | HIGH | yes | `analytics/03` §`Orphan requirement` L27 | in-degree test on the requirement node in the trace subgraph (edge kinds: satisfy, veri… |
| `D2` | Orphan test | HIGH | yes | `analytics/03` §`Orphan test` L40 | out-degree test on the TestCase node for «verify» edges. O(1). |
| `D3` | Orphan element | MEDIUM | yes | `analytics/03` §`Orphan element` L50 | total-degree test (in + out) across ALL relationship kinds on the relationship graph. O… |
| `D4` | Uncovered element | LOW | yes | `analytics/03` §`Uncovered element` L75 | set difference — modelableElements − elementsReferencedByAnyDiagram. Computed from the… |
| `D5` | Circular trace | HIGH | yes | `analytics/03` §`Circular trace` L96 | DFS cycle detection (or equivalently, any SCC with >1 vertex in the trace graph). Refer… |
| `D6` | Incomplete chain | MEDIUM | yes | `analytics/03` §`Incomplete chain` L114 | reachability (BFS) from the requirement following «satisfy»/«realize» in reverse (targe… |
| `D7` | Disconnected subgraph | MEDIUM | on demand | `analytics/03` §`Disconnected subgraph` L135 | weakly connected components on the trace graph. Reference: [Connected component](https:… |

Defect sub-requirements live under domain `ORPH-D{1..7}` (e.g. `REQ-ORPH-D1-001` detection,
`REQ-ORPH-D1-002` severity) — see §2. `REQ-SEV-001` defines the severity scale
(HIGH/MEDIUM/LOW); `REQ-ORPH-SUP-*` governs suppression; `REQ-ORPH-INC-*` governs incremental
maintenance; `REQ-ORPH-MCP-001` exposes defects via `analysis.issues` + `smith://analysis/issues`.

## 5. Design principle index (P1–P10)

All authored in `03-design-principles.md`. §Decision ladder (same doc) ranks how to resolve
open decisions toward the north stars in `01-vision.md` §Design north stars.

| ID | Title | Defined in | One-line |
|----|-------|-----------|----------|
| `P1` | The model is the single source of truth | `03-design-principles.md` §`The model is the single source of truth` L10 | There is exactly one model per project, stored as a graph. |
| `P2` | Ownership lives in the model tree, never on the diagram | `03-design-principles.md` §`Ownership lives in the model tree, never on the diagram` L25 | Every element has exactly one owner (a Namespace, usually a Package), forming a tree rooted at the project. |
| `P3` | Agents and humans are peers | `03-design-principles.md` §`Agents and humans are peers` L35 | The model API is the boundary. |
| `P4` | Make defects visible immediately | `03-design-principles.md` §`Make defects visible immediately` L46 | Smith continuously runs lightweight quality checks. |
| `P5` | Boring foundations, novel UX | `03-design-principles.md` §`Boring foundations, novel UX` L58 | Persistence, the graph schema, and the MCP protocol are well-understood problems. |
| `P6` | Reference, don't re-derive | `03-design-principles.md` §`Reference, don't re-derive` L68 | Algorithms (BFS, topological sort, Tarjan SCC, Floyd-Warshall reachability) are defined once in authoritative online sources. |
| `P7` | Restraint in visuals | `03-design-principles.md` §`Restraint in visuals` L77 | The UI must be pleasant for multi-hour sessions. |
| `P8` | Stable identities, derived names | `03-design-principles.md` §`Stable identities, derived names` L86 | Every element has a stable, opaque, immutable ID (assigned at creation, never reused). |
| `P9` | Undoable, auditable | `03-design-principles.md` §`Undoable, auditable` L95 | Every model mutation is an undoable command on a global command stack. |
| `P10` | Local-first, single binary | `03-design-principles.md` §`Local-first, single binary` L104 | Smith runs as one process on the user's machine: UI + model API + embedded graph DB + MCP server. |

## 6. Goal index (G1–G10)

All authored in `01-vision.md` §Goals. Non-goals are in §Non-goals of the same doc.

| ID | Goal | Defined in |
|----|------|-----------|
| `G1` | Store a complete UML 2.5.1 model in a graph database with package ownership. | `01-vision.md` §Goals L41 |
| `G2` | Render all 14 UML diagram kinds as views over the single model; never duplicate elements. | `01-vision.md` §Goals L42 |
| `G3` | Extend UML with Requirement and TestCase (SysML-inspired) as first-class model elements. | `01-vision.md` §Goals L43 |
| `G4` | Provide traceability relations («satisfy», «verify», «realize», «deriveReqt», «trace», «refine», «copy») and keep the canonical chain queryable. | `01-vision.md` §Goals L44 |
| `G5` | Ship analytical tools: RTM, adjacency matrix, orphan/uncovered-element detection, cycle detection, connected components. | `01-vision.md` §Goals L45 |
| `G6` | Embed an MCP server (HTTP/SSE, localhost) exposing tools/resources/prompts for AI agents. | `01-vision.md` §Goals L46 |
| `G7` | Provide a beautiful, ergonomic, eye-pleasing UI with a context-sensitive gizmo and ownership-aware selection. | `01-vision.md` §Goals L47 |
| `G8` | Distribute as a single self-contained desktop binary (no external DB server, no separate backend). | `01-vision.md` §Goals L48 |
| `G9` | Support search across the model with ranked results and graph-traversal queries. | `01-vision.md` §Goals L49 |
| `G10` | Support undo/redo and version-safe project files. | `01-vision.md` §Goals L50 |

## 7. Trace stereotype index

All defined in `architecture/04-traceability-relations.md` §Relation taxonomy (table L39–45)
and §The canonical chain. Edge direction is **source → target** (see §11).

| Stereotype | Source → Target | Meaning | Quality impact |
|-----------|-----------------|---------|----------------|
| `«satisfy»` | Design element (`UseCase`, `Class`, `Component`, `Activity`, `Interaction`) → `Requirement` | Source satisfies the target requirement. | Requirement is addressed. |
| `«verify»` | `TestCase` → `Requirement` | Source TestCase verifies the target requirement. | Requirement is tested. |
| `«realize»` | Implementer (`Activity`→`UseCase`, `Interaction`→`Activity`, `Class`/`Operation`→`Interaction`) → contract | Source implements the target's specified behavior. | Implementation completeness. |
| `«deriveReqt»` | `Requirement` (parent) → `Requirement` (derived child) | Target derived from source (decomposition). SysML §16.3.2.3. | Requirement hierarchy. |
| `«refine»` | Any → Any | Source refines target at more detailed abstraction. | Refinement history. |
| `«trace»` | Any → Any | Generic trace; loose coupling, no contract. | Loose relation. |
| `«copy»` | Any → Any | Target is a copy of source (provenance). SysML §16.3.2.2. | Provenance. **Excluded from RTM** (DECIDED — see §11). |

Cardinality rules (§Cardinality rules, L47–57): a `TestCase` MUST have ≥1 outgoing
`«verify»`; a `Requirement` with zero incoming `«satisfy»`/`«verify»`/`«realize»` AND zero
outgoing `«deriveReqt»` is an **orphan requirement** (D1). The **trace graph** = only these
seven edge kinds (`«copy»` excluded from chain analysis but kept in the relationship graph);
the **relationship graph** = all relationships (associations, generalizations, dependencies,
trace relations) used for adjacency/centrality.

## 8. MCP tool index

Authoritative tool list from `mcp/02-tools.md` — **83 tools** across 8 domains. Tool names
are `camelCase`, namespaced by domain. `readOnlyHint` / `destructiveHint` annotations per
`REQ-MCP-TOOLS-001`; mutating tools return `{ ok, ...ids }` (`REQ-MCP-TOOLS-002`) and emit
`notifications/resources/updated` (`REQ-MCP-TOOLS-003`). Every mutating tool pushes one undo
command; `batch.*` groups several into one.

### `model.*` — element & relationship operations (44 tools)

**Structural (10):** `model.createPackage` · `model.createClass` · `model.createInterface` ·
`model.createDataType` · `model.createEnumeration` · `model.createActor` ·
`model.createUseCase` · `model.createComponent` · `model.createNode` · `model.createArtifact`.

**Behavioral (3):** `model.createActivity` · `model.createInteraction` ·
`model.createStateMachine`.

**Requirements & tests (2):** `model.createRequirement` · `model.createTestCase`.

**Features (3):** `model.addAttribute` · `model.addOperation` · `model.addEnumerationLiteral`.

**Relationships (5):** `model.createAssociation` · `model.createGeneralization` ·
`model.createRealization` · `model.createDependency` · `model.createTraceRelation`
(validates source/target kinds; `-32001` on mismatch, `-32003` on cycle unless
`acceptDefect: true`).

**Stereotypes & tagged values (4):** `model.applyStereotype` · `model.removeStereotype` ·
`model.setTaggedValue` · `model.addComment`.

**Profile authoring (4):** `model.createProfile` · `model.createStereotype` ·
`model.addTagDefinition` · `model.createExtension`.

**Mutation & lifecycle (8):** `model.rename` · `model.reparent` · `model.setMultiplicity` ·
`model.setVisibility` · `model.delete` (cascade-aware) · `model.updateRequirement` ·
`model.updateTestCase` · `model.setTestStatus`.

**Queries, read-only (5):** `model.get` · `model.getByName` · `model.listChildren` ·
`model.qualifiedName` · `model.ownerChain`.

### `diagram.*` — view operations (10 tools)

Manipulate **view references**, never model elements (P1/P2). `diagram.create` ·
`diagram.addElement` (`-32001` if element kind not allowed on this `DiagramKind`) ·
`diagram.addRelation` · `diagram.removeView` · `diagram.moveShape` · `diagram.setBendPoints`
· `diagram.autoLayout` (ELK; v1 ships ELK only) · `diagram.export` (svg/png/pdf) ·
`diagram.list` · `diagram.get`.

### `analysis.*` — analytics, read-only (9 tools)

`analysis.rtm.forward` · `analysis.rtm.backward` · `analysis.issues` (D1–D7) ·
`analysis.uncovered` (D4) · `analysis.orphanRequirements` (D1) · `analysis.orphanTests` (D2)
· `analysis.circularTraces` (D5) · `analysis.incompleteChains` (D6) ·
`analysis.disconnectedSubgraphs` (D7).

### `graph.*` — graph algorithms, read-only (9 tools)

`graph.adjacencyMatrix` · `graph.reachable` · `graph.transitiveClosure` (long-running) ·
`graph.topoSort` · `graph.cycles` · `graph.scc` (Tarjan) · `graph.shortestPath` (BFS/Dijkstra)
· `graph.centrality` (betweenness/closeness) · `graph.connectedComponents` (weak/strong).

### `trace.*` — chain navigation, read-only (2 tools)

`trace.forward` (downstream by chain level) · `trace.backward` (upstream).

### `search.*` — search (3 tools)

`search.elements` (FTS5 + kind/package filter) · `search.relations` · `search.diagrams`.

### `project.*` — project lifecycle (3 tools)

`project.info` · `project.save` (WAL checkpoint) · `project.export` (xmi/json/cypher).

### `batch.*` — composite / undo (3 tools)

`batch.run` (sequence as one undoable command, all-or-nothing rollback) · `batch.undo` ·
`batch.redo` (undo/redo share one stack with the UI).

Resources (`mcp/03-resources.md`, URI scheme `smith://`) and prompts (`mcp/04-prompts.md`)
are separate surfaces, not counted above.

## 9. Error code index

Smith-defined codes are in `mcp/01-server-design.md` §Error model; JSON-RPC standard and
MCP spec-reserved codes are referenced from `research/02-mcp-transport.md` §4.8. Every tool
error includes `message` + `data.code` (`REQ-MCP-016`).

### Smith-defined (`mcp/01` §Error model)

| Code | Meaning |
|------|---------|
| `-32000` | Element not found (bad ID). |
| `-32001` | Metamodel constraint violation: wrong element kinds for a relation stereotype; stereotype applied to wrong metaclass; view reference to a kind disallowed on that diagram kind. |
| `-32002` | Uniqueness violation: name collision in namespace; duplicate project-wide `requirementId`; duplicate `testCaseId`. |
| `-32003` | Operation would create a model defect (e.g. cycle); rejected unless `acceptDefect: true`. |
| `-32004` | Project read-only / locked. |
| `-32005` | Undo stack exhausted. |

### JSON-RPC 2.0 standard (`research/02` §4.8)

| Code | Meaning |
|------|---------|
| `-32700` | Parse error. |
| `-32600` | Invalid request. |
| `-32601` | Method not found. (Also: unknown URI path on resources.) |
| `-32602` | Invalid params. (Also: unknown-element resource URI per `REQ-MCP-RES-003`.) |
| `-32603` | Internal error. |

### MCP spec-reserved (`research/02` §4.8; 2026-07-28)

| Code | Meaning |
|------|---------|
| `-32020` | `HeaderMismatch` (modern-era `Mcp-Method`/`Mcp-Name` vs body). |
| `-32021` | `MissingRequiredClientCapability`. |
| `-32022` | `UnsupportedProtocolVersion`. |

Reserved ranges: `-32000…-32019` implementation-defined (Smith uses -32000…-32005);
`-32020…-32099` spec-reserved.

## 10. DiagramKind index

Enumeration defined in `uml-model/06-diagram-interchange.md` §DiagramKind enumeration
(L50–53): 14 UML kinds + the Smith-specific `requirement` = **15 kinds**. A diagram's `kind`
constrains which element/edge kinds its view references may name (`diagram.addElement`
rejects mismatches with `-32001`).

| # | `kind` | Family | Allowed shapes / edges (doc) |
|---|-------|--------|------------------------------|
| 1 | `class` | Structural | `uml-model/02` §Class diagram |
| 2 | `object` | Structural | `uml-model/02` §Object diagram |
| 3 | `component` | Structural | `uml-model/02` §Component diagram |
| 4 | `compositeStructure` | Structural | `uml-model/02` §Composite Structure diagram |
| 5 | `package` | Structural | `uml-model/02` §Package diagram |
| 6 | `deployment` | Structural | `uml-model/02` §Deployment diagram |
| 7 | `profile` | Structural | `uml-model/02` §Profile diagram |
| 8 | `useCase` | Behavioral | `uml-model/03` §Use Case diagram |
| 9 | `activity` | Behavioral | `uml-model/03` §Activity diagram |
| 10 | `sequence` | Behavioral | `uml-model/03` §Sequence diagram |
| 11 | `stateMachine` | Behavioral | `uml-model/03` §State Machine diagram |
| 12 | `communication` | Behavioral | `uml-model/03` §Communication diagram |
| 13 | `timing` | Behavioral | `uml-model/03` §Timing diagram |
| 14 | `interactionOverview` | Behavioral | `uml-model/03` §Interaction Overview diagram |
| 15 | `requirement` | Smith extension | `uml-model/04` §Requirement diagram (15th kind) |

Structural kinds 1–7: `uml-model/02-structural-diagrams.md`. Behavioral kinds 8–14:
`uml-model/03-behavioral-diagrams.md`. Notation per kind: `research/04-uml-visual-notation.md`
(one numbered H2 per kind).

## 11. Cross-cutting decision map

Load-bearing decisions and where they are authored, so an agent changing one knows what
else to re-check. **DECIDED** rows are resolved (originally open questions, now `[x]`); the
rest are foundational decisions stated normatively across the spec.

| Decision | Authored in | Affects / re-check |
|----------|------------|--------------------|
| Trace edge direction is **source → target** (stored this way) | `architecture/04` §Relation taxonomy + §The canonical chain | `uml-model/05` (chain navigation), `analytics/01` (RTM), `analytics/03` (D1/D6 detection traverses satisfy/realize in reverse), `02-glossary` |
| `.smith` = **single SQLite file** (DECIDED) | `architecture/03` §`.smith` file format (L226); per `REQ-PERS-017/018/019` | `architecture/06`, `mcp/01` §Discovery, single-binary distribution (G8) |
| **SQLite authoritative; projection cache** (DECIDED per `REQ-PERS-001/002`) | `architecture/01` (L187); `architecture/03` §Store of record vs. projection | all read paths, load/crash recovery, `architecture/03` §SQLite schema |
| **Write-through persistence** (DECIDED) | `architecture/01` (L182) | `architecture/03` §Transactions & concurrency, undo (`REQ-PERS-010`), Save = WAL checkpoint |
| `«copy»` **excluded from RTM & chain analysis** (DECIDED) | `architecture/04` (L134–135) | `analytics/01` (RTM), trace graph definition, `analytics/02` adjacency (copy stays in relationship graph) |
| D1–D6 maintained **continuously/incrementally**; D7 **on demand** | `analytics/03` §Defect taxonomy + §Incremental maintenance (`REQ-ORPH-001`, `REQ-ORPH-INC-001/002`) | model API mutation hooks, `analysis.issues`, UI issue panel |
| D3(a) ownership-reachability is **INV-MM-001** (invariant), not just a defect | `analytics/03` §D3 (`REQ-ORPH-D3-001`) ↔ `uml-model/01` §Invariants | model API enforces at mutation time; a violation is fatal model corruption |
| Coverage **`verified` requires a passing test** (Verdict from TestCase status) | `uml-model/05` §Completeness + `analytics/01` §Coverage state machine | `analytics/01` RTM Verdict column, `model.setTestStatus` |
| A `TestCase` **MUST have ≥1 outgoing `«verify»`** (else D2) | `architecture/04` §Cardinality rules + `analytics/03` §D2 | `model.createTestCase`/trace validation, `analytics/03` D2 detection |
| **Transport = Streamable HTTP, legacy era** (not stdio, not HTTP+SSE) | `mcp/01` §Transport + §Transport selection rationale; `research/02` §2.5 | `mcp/01` all endpoints/sessions, `mcp/05` client configs, `architecture/06` rmcp pin |
| MCP server on **`127.0.0.1` loopback only**, ephemeral port + discovery file | `mcp/01` §Transport (`REQ-MCP-002/003`), §Discovery (`REQ-MCP-011/012`) | `mcp/05` client config, security (`REQ-MCP-015`) |
| Diagrams **own no model elements** (P1/P2); DI shapes are view references | `03-design-principles` P1/P2; `uml-model/06` §The principle | `uml-model/02`/`03` kind constraints, `diagram.*` tools, D4 coverage index |
| `qualifiedName` is **derived** from ownership tree (P8) | `uml-model/01` §Root abstractions + P8; `INV-MM-005` | `model.rename`/`reparent` return value, search indexing, uniqueness (`REQ-MM-*`) |
| **v1 = one project per process**, multiple windows share the model API (DECIDED) | `architecture/03` (L229) | `architecture/01` process model, `mcp/01` concurrency (`REQ-MCP-009/019`) |
| WAL `synchronous=NORMAL` (DECIDED) | `architecture/03` (L231) | `architecture/03` §Performance budget, write throughput |
| **All 14 UML diagram kinds + requirement** ship in v1 (G2) | `01-vision` §Goals G2; `uml-model/02`/`03`/`04` | `research/04` notation, `ui-ux/02` gizmo per-kind, build order in `99-implementation-guide` |

## 12. Open questions (unresolved)

Every `- [ ]` checkbox under an *Open questions* section across all docs. These are the
explicit decisions still pending. `[x]` DECIDED items are NOT here — they are in §11. The
7 `- [ ]` items in `99-implementation-guide.md` §Acceptance-criteria template are a **format
template**, not real questions — excluded below.

**52 open questions** across 22 documents:

- [`analytics/01-rtm.md` §`Open questions (resolve before STABLE)` L135] Weighted coverage (a requirement with 3 of 5 UseCases traced): show as 60%? Tentative: yes, as an optional column, not replacing the discrete coverage state.
- [`analytics/01-rtm.md` §`Open questions (resolve before STABLE)` L137] Should RTM support **time-travel** (show the RTM as of a past commit)? Out of scope v1; requires versioning. Tentative: no.
- [`analytics/02-adjacency-and-graph-analysis.md` §`Open questions (resolve before STABLE)` L169] Edge weights for Dijkstra: default all 1 (unweighted)? Or allow user-tagged weights? Tentative: default 1; user may tag via stereotype tagged-value `weight`.
- [`analytics/02-adjacency-and-graph-analysis.md` §`Open questions (resolve before STABLE)` L171] Should centrality be computed incrementally and cached? Tentative: no — on demand only; cache invalidates on any relationship mutation.
- [`analytics/03-orphan-and-coverage-analysis.md` §`Open questions (resolve before STABLE)` L189] Should D6 distinguish "reached Activity but not Class" from "reached nothing"? Current spec: yes, via the coverage state in RTM (satisfied vs realized vs untraced), but D6 itself is a single MEDIUM…
- [`analytics/03-orphan-and-coverage-analysis.md` §`Open questions (resolve before STABLE)` L192] Auto-fix suggestions (e.g. "create a UseCase for this orphan requirement")? Out of scope v1; the issue panel links to the relevant create-tool instead.
- [`analytics/04-quality-rules.md` §`Open questions (resolve before STABLE)` L106] Auto-fix actions: should Smith offer one-click fixes (e.g. "create UseCase for orphan requirement")? Tentative: yes — link to the relevant prompt/tool; do not auto-execute.
- [`analytics/04-quality-rules.md` §`Open questions (resolve before STABLE)` L108] Rule customization: allow changing a built-in rule's severity per-project? Tentative: no in v1 (fixed severities); user-defined rules in v2 carry their own severities.
- [`architecture/04-traceability-relations.md` §`Open questions (resolve before STABLE)` L130] Do we allow `«realize»` from a `Class` directly to a `UseCase` (skipping the behavioral chain), or strictly Activity → UseCase / Class → Interaction? Current spec: the taxonomy permits any implemen…
- [`architecture/05-search.md` §`Open questions (resolve before STABLE)` L124] CJK tokenizer for FTS5 (out of scope v1) — note as a known limitation; users with CJK content get whole-string substring matches, not word matches.
- [`architecture/05-search.md` §`Open questions (resolve before STABLE)` L126] Fuzzy matching (Levenshtein) for typo tolerance? FTS5 has no built-in fuzzy; a trigram-based approach would help. Tentative: out of scope v1; prefix matching (`term*`) covers the common case.
- [`architecture/05-search.md` §`Open questions (resolve before STABLE)` L129] Search history / saved queries? Tentative: saved queries (named, in ui_state) in v1; history later.
- [`architecture/06-tech-stack.md` §`Open questions (resolve before STABLE)` L171] Exact `rmcp` version once pinned — verify it supports the legacy-era Streamable HTTP shape Smith targets (§2.5 of research/02).
- [`architecture/06-tech-stack.md` §`Open questions (resolve before STABLE)` L173] Whether to vendor ELK.js or load via npm — npm (with exact pin) is simpler.
- [`architecture/06-tech-stack.md` §`Open questions (resolve before STABLE)` L174] `.smith` file: single SQLite file, or a directory (for future assets like exported images)? Current spec: single file; assets stored as blobs if ever needed.
- [`mcp/01-server-design.md` §`Open questions (resolve before STABLE)` L184] Should Smith support the older pure-SSE transport for back-compat with clients that only speak the 2024-11-05 spec? Decision: yes, as a thin adapter, marked deprecated.
- [`mcp/01-server-design.md` §`Open questions (resolve before STABLE)` L186] Exact port allocation strategy: fixed default vs. ephemeral + discovery file. Current spec: ephemeral + discovery; fixed default as fallback.
- [`mcp/02-tools.md` §`Open questions (resolve before STABLE)` L233] Should `batch.run` support a declarative DSL or just a list of tool calls? Current: list of tool calls (simplest, composable).
- [`mcp/02-tools.md` §`Open questions (resolve before STABLE)` L235] Pagination for `model.listChildren` and `search.*` via MCP `cursor` — yes for search (can be large); no for listChildren (usually small).
- [`mcp/02-tools.md` §`Open questions (resolve before STABLE)` L237] Should `analysis.*` tools accept a `progressToken` for long computations? Yes — per [mcp/01](./01-server-design.md) §Concurrency.
- [`mcp/03-resources.md` §`Open questions (resolve before STABLE)` L150] Should Smith expose a `smith://search?q=...` resource (readable search results), or is the `search.elements` tool sufficient? Current: tool only; resources are for stable addresses, not query results.
- [`mcp/03-resources.md` §`Open questions (resolve before STABLE)` L153] Versioned resources (`smith://elements/{id}?at=<commit>`)? Out of scope v1 (no versioning).
- [`mcp/04-prompts.md` §`Open questions (resolve before STABLE)` L92] Should prompts support multi-turn (return an assistant message too, priming the model)? Current: no — single user message; let the agent drive.
- [`mcp/04-prompts.md` §`Open questions (resolve before STABLE)` L94] Should the host be able to pass a `style` argument (e.g. terse vs. verbose output)? Tentative: yes as an optional arg on analysis prompts.
- [`mcp/05-client-config.md` §`Open questions (resolve before STABLE)` L169] Should Smith offer a "copy config" button per client in the UI? Yes — planned.
- [`mcp/05-client-config.md` §`Open questions (resolve before STABLE)` L170] Should Smith auto-register with Claude Code by writing `.mcp.json`? No — that's a user decision; Smith only displays the snippet. Writing the file silently is surprising.
- [`ui-ux/01-design-system.md` §`Open questions (resolve before STABLE)` L182] Exact accent hue — indigo vs. a warmer hue. Tentative: indigo (#4F5BD5 family) for a calm, professional feel. Confirm with a mock.
- [`ui-ux/01-design-system.md` §`Open questions (resolve before STABLE)` L184] Full category fill vs. top-bar-only tint as the default. Tentative: top-bar-only (calmer); expose a setting.
- [`ui-ux/01-design-system.md` §`Open questions (resolve before STABLE)` L186] Font family: bundle Inter, or use system stack (`-apple-system, Segoe UI, Roboto, ...`)? Tentative: system stack for chrome (zero bundle, native feel); Inter for canvas text (consistency across web…
- [`ui-ux/02-canvas-and-interaction.md` §`Open questions (resolve before STABLE)` L241] Gizmo gesture threshold (8px) — confirm empirically with a prototype.
- [`ui-ux/02-canvas-and-interaction.md` §`Open questions (resolve before STABLE)` L242] Should the gizmo remember per-diagram-kind recent items across sessions? Tentative: yes, persisted in settings.
- [`ui-ux/02-canvas-and-interaction.md` §`Open questions (resolve before STABLE)` L244] Multi-touch (pinch) on trackpad vs. tablet — confirm Tauri webview gesture support.
- [`ui-ux/02-canvas-and-interaction.md` §`Open questions (resolve before STABLE)` L245] Right-drag vs. middle-mouse for pan — offer both; middle-mouse default on macOS (where right-drag is uncommon).
- [`ui-ux/03-inspector-and-explorer.md` §`Open questions (resolve before STABLE)` L154] Should the inspector support a "split" mode (show two elements side by side for comparison)? Tentative: no in v1; add if users request.
- [`ui-ux/03-inspector-and-explorer.md` §`Open questions (resolve before STABLE)` L156] Explorer drag-to-reparent confirmation: always ask, or only on cross-package moves? Tentative: only on moves that change qualifiedName significantly (different top-level package); same-subtree move…
- [`ui-ux/04-search-and-analysis-ui.md` §`Open questions (resolve before STABLE)` L147] Should the trace overview be interactive (click an element to recenter the chain on it)? Tentative: yes.
- [`ui-ux/04-search-and-analysis-ui.md` §`Open questions (resolve before STABLE)` L149] Dashboard scope: project-wide only, or per-package? Tentative: project-wide in v1.
- [`ui-ux/04-search-and-analysis-ui.md` §`Open questions (resolve before STABLE)` L150] Heat-map cell click behavior: select both, or open a relationships inspector? Tentative: select both + show a small popover listing the edges.
- [`uml-model/01-metamodel-core.md` §`Open questions (resolve before STABLE)` L272] Should `Comment` be a first-class element (ownable, referenceable) or strictly attached? Current spec: attached (owned by an element), with `annotatedElements` for cross-references.
- [`uml-model/01-metamodel-core.md` §`Open questions (resolve before STABLE)` L275] Are `AssociationClass` and `Signal` in v1? Tentatively: `Signal` yes (for sequence async messages); `AssociationClass` yes (class-with-association, common in domain models).
- [`uml-model/02-structural-diagrams.md` §`Open questions (resolve before STABLE)` L157] Should class diagrams support nested-package rendering (show packages as containers with classes inside)? Tentative: yes (an optional layout mode); the default is flat-with-qualified-names.
- [`uml-model/02-structural-diagrams.md` §`Open questions (resolve before STABLE)` L160] AssociationClass support in v1? Tentative: yes (common in domain models).
- [`uml-model/03-behavioral-diagrams.md` §`Open questions (resolve before STABLE)` L192] Timing diagram: full support in v1? It's optional for conforming tools. Tentative: yes, basic (lifeline + state timeline + message), advanced constraints later.
- [`uml-model/03-behavioral-diagrams.md` §`Open questions (resolve before STABLE)` L194] Communication & Interaction Overview diagrams: full v1 or defer? Tentative: v1 supports all 14 (per vision G2), but Communication/Interaction Overview get less UX polish than the core 5 (class, use…
- [`uml-model/03-behavioral-diagrams.md` §`Open questions (resolve before STABLE)` L197] Activity diagram swimlane orientation: vertical columns default (industry standard per [research/04](../research/04-uml-visual-notation.md) §Layout) — confirm with a mock.
- [`uml-model/04-requirements-and-tests.md` §`Open questions (resolve before STABLE)` L194] Do we need requirement **versioning** (trace history of text changes)? Current spec: no; `status` + `rationale` suffice for v1. Versioning is a v2 candidate.
- [`uml-model/04-requirements-and-tests.md` §`Open questions (resolve before STABLE)` L196] Should `Requirement` support nested sub-requirements via ownership (not just `«deriveReqt»`)? Current spec: no — deriveReqt is the only parent-child relation. Ownership is purely structural (packag…
- [`uml-model/05-traceability-chain.md` §`Open questions (resolve before STABLE)` L142] Do we allow skipping levels (Requirement → Class directly)? Current spec: yes; treated as a complete path of length 1.
- [`uml-model/05-traceability-chain.md` §`Open questions (resolve before STABLE)` L144] Weighted completeness (a Requirement with 1 of 5 UseCases traced is "20% complete")? Tentative: yes in v1 as an optional column in RTM.
- [`uml-model/06-diagram-interchange.md` §`Open questions (resolve before STABLE)` L176] Do we support **diagram links** (a shape on one diagram that, when clicked, opens another diagram)? Useful for navigation. Tentative: yes, via a special shape kind `DiagramLinkShape`.
- [`uml-model/06-diagram-interchange.md` §`Open questions (resolve before STABLE)` L179] Do we support **diagram frames** (the UML diagram frame border with a heading like `sd Login`)? Tentative: yes, optional, on by default for sequence diagrams.
- [`uml-model/06-diagram-interchange.md` §`Open questions (resolve before STABLE)` L181] Should bend points be **absolute** canvas coords or **relative** to source/target? Current spec: absolute (simpler, stable under element move). Reconsider if move performance suffers.

## Maintenance

This file is AUTO-GENERATED from the spec by scanning `## H2` headings + bracketed REQ/INV IDs +
defect/principle/goal/tool/error enumerations + open-question checkboxes. When the spec changes,
regenerate. A `docs/scripts/build_index.py` is a TODO; until then, regenerate by re-running the
extraction (headings via `grep '^## '`, IDs via `grep -roE '\[REQ-[A-Z][A-Z0-9-]*?-[0-9]+\]'`).

Manual edits to this file will be lost. If the spec and this index disagree, **the spec wins** —
file the discrepancy so both get reconciled.
