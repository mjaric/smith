---
status: REVIEW
---

# CONTRIBUTING.md — Working on the Smith Spec

How future AI agent sessions (and humans) should consume, implement from, and refine the
Smith specification in `docs/`. Read this at the start of every session that touches Smith.

**Companion file:** [AGENTS.md](./AGENTS.md) is the reverse-index (every REQ-*, INV-*, D*,
P*, G*, tool, error code, and open question mapped to its definition site). Read AGENTS.md
once to orient; use this file for *how to work*.

---

## 0. What Smith is (30-second orientation)

Smith is a **desktop UML modeling tool** (Rust backend, Tauri 2 + React + Konva frontend) with
three distinguishing properties: (1) the model is a **graph** in an embedded SQLite store with
an in-memory `petgraph` projection; (2) it is **agent-native** — an embedded MCP server
(HTTP/SSE on `127.0.0.1`) lets Claude Code / Codex / Gemini / web chat read and mutate the
same model as the human; (3) it models the **whole story** — full UML 2.5.1 + SysML-inspired
Requirements/TestCases + traceability relations, so the canonical chain
`Requirement → UseCase → Activity → Sequence → Class` is first-class and continuously
analyzed (RTM, adjacency, orphan/coverage detection).

This directory is **the specification** — no implementation exists yet. Every `REQ-*` is a
normative contract; every `INV-*` is an invariant that must hold after any mutation.

---

## 1. Reading order for a new session

Don't read all 418 KB. Read in this order, stopping as soon as you have enough context for
your task:

1. **[00-README.md](./00-README.md)** — document map (what lives where).
2. **[01-vision.md](./01-vision.md)** — goals (G1–G10) and non-goals. *2 minutes.*
3. **[02-glossary.md](./02-glossary.md)** — vocabulary. Skim; return when a term is unclear.
4. **[03-design-principles.md](./03-design-principles.md)** — principles P1–P10. **Load-bearing.**
   When a spec is silent, resolve ambiguity toward these in the order listed.
5. **[AGENTS.md](./AGENTS.md)** — the reverse-index. Jump from here to the specific doc/section.
6. **The one or two subsystem docs your task needs** (found via AGENTS.md or 00-README).
7. **The linked research doc(s)** ONLY if the subsystem doc cites them for background.

**Never read the research/ docs front-to-back.** They are references, cited on demand.

---

## 2. The agent's two modes

### Mode A — Implementation (build a feature)

You are given a feature (e.g. "implement the class diagram canvas" or "add the RTM tool").
Your job is to produce working code that satisfies the spec.

**Workflow:**

1. **Identify the subsystem** from [00-README.md](./00-README.md) or AGENTS.md §1.
2. **Read the normative spec doc(s)** for that subsystem. Every `REQ-*` is a contract you must
   satisfy; every `INV-*` must hold after your code runs.
3. **Read [99-implementation-guide.md](./99-implementation-guide.md)** — it gives the build
   order (Slice 0 → Slice 6) and the acceptance-criteria template.
4. **Check the decision map** in AGENTS.md §11 — know which decisions are settled so you don't
   relitigate them.
5. **Check AGENTS.md §12 (open questions)** — if your task touches an unresolved question,
   flag it to the human before guessing; do not silently pick.
6. **Implement.** For each `REQ-*` you satisfy, write a test that asserts it (the
   acceptance-criteria template in 99-guide is the contract between you and the reviewer).
7. **Verify** per the implementation guide's cross-cutting acceptance (ownership tree acyclic;
   deleting a diagram deletes zero elements; no panic crosses the model API; etc.).
8. **Run** `cargo test`, `cargo clippy --all-targets -- -D warnings`, `vitest`, `oxlint`.
   Zero warnings is the baseline (per repo rules).

**Anti-patterns to refuse in your own code** (from 99-guide §Anti-patterns):
- Storing model facts (name, attribute) on the diagram — violates P1.
- A UI capability with no MCP equivalent — violates P3.
- Re-deriving an algorithm instead of referencing it — violates P6.
- A destructive operation that bypasses undo — violates P9.
- Color carrying meaning without a shape/text backup — violates REQ-DS-002.

**When the spec is ambiguous:** climb the decision ladder (03-design-principles §Decision
ladder). If still ambiguous, **file it as a spec issue and ask** — do not guess on
load-bearing decisions. Trace edge direction (architecture/04), `.smith` file format
(architecture/03), and the Verified-completeness definition (uml-model/05) are settled;
re-litigating them wastes a round-trip.

### Mode B — Refinement (improve the spec)

You are given a spec-change task (e.g. "add support for Activity final-node semantics" or
"resolve the open question about CJK tokenizers"). Your job is to edit `docs/`.

**Workflow:**

1. **Find every doc that touches the topic** via AGENTS.md reverse-index. Search for the
   relevant `REQ-*`, term in the glossary, or H2 section in the outline map.
2. **Read all of them before editing any.** A change to one doc usually requires parallel
   changes in 2–5 others (the spec is densely cross-referenced — see AGENTS.md §11 decision
   map for the load-bearing ones).
3. **Edit minimally.** Touch only the REQs/sections that change. Do not rewrite whole docs.
4. **Propagate.** If you change a definition, every cross-reference must be updated. Use
   `grep` to find mentions: `grep -rn 'REQ-MM-001' docs/`.
5. **Update AGENTS.md** if you add/remove/rename a REQ-*, INV-*, defect code, MCP tool, or
   settle an open question. (AGENTS.md is auto-generated in spirit; regenerate the affected
   section by re-running the extraction described in its header.)
6. **Run a consistency review** before declaring done. Two options:
   - **Light:** re-read every doc you changed + every doc that cites a REQ-* you changed.
   - **Full:** dispatch a `reviewer` subagent to cross-check the whole spec (the way the
     initial review found 2 BLOCKERs — see Refinement history below). Recommended for any
     change touching traceability, the data model, or error codes.
7. **Bump status** in the frontmatter of changed docs: `DRAFT → REVIEW → STABLE`. Don't claim
   `STABLE` unless there are zero open questions in that doc.

**Refinement anti-patterns:**
- Editing one doc without propagating to its dependents (the #1 source of the BLOCKERs the
  initial review found — trace direction was stated one way in architecture/04 and the
  opposite way in 4 other docs).
- Adding a new REQ-* without adding it to AGENTS.md §2.
- Resolving an open question without moving it from AGENTS.md §12 to §11 (decision map).

---

## 3. Where things live (quick map)

| You need... | Read... |
|-------------|---------|
| The model's data structure | [uml-model/01-metamodel-core.md](./uml-model/01-metamodel-core.md), [architecture/03-persistence.md](./architecture/03-persistence.md) §SQLite schema |
| What elements a diagram may show | [uml-model/02-structural-diagrams.md](./uml-model/02-structural-diagrams.md), [03-behavioral-diagrams.md](./uml-model/03-behavioral-diagrams.md) |
| How diagrams-as-views work (critical!) | [uml-model/06-diagram-interchange.md](./uml-model/06-diagram-interchange.md) |
| The traceability chain & relations | [architecture/04-traceability-relations.md](./architecture/04-traceability-relations.md) (authoritative direction), [uml-model/05-traceability-chain.md](./uml-model/05-traceability-chain.md) |
| How RTM / orphan detection computes | [analytics/01-rtm.md](./analytics/01-rtm.md), [03-orphan-and-coverage-analysis.md](./analytics/03-orphan-and-coverage-analysis.md) |
| What graph algorithm to use | [analytics/02-adjacency-and-graph-analysis.md](./analytics/02-adjacency-and-graph-analysis.md) + [research/05](./research/05-traceability-and-analytics.md) §3 (reference, not derivation) |
| What MCP tools exist | [mcp/02-tools.md](./mcp/02-tools.md) + AGENTS.md §8 |
| How to register Smith in Claude Code/Codex/Gemini | [mcp/05-client-config.md](./mcp/05-client-config.md) |
| The canvas / gizmo interaction | [ui-ux/02-canvas-and-interaction.md](./ui-ux/02-canvas-and-interaction.md) |
| The design tokens (color/type/spacing) | [ui-ux/01-design-system.md](./ui-ux/01-design-system.md) |
| Why a tech choice was made | [architecture/06-tech-stack.md](./architecture/06-tech-stack.md) + the cited research doc |
| Build order | [99-implementation-guide.md](./99-implementation-guide.md) §Suggested build order |

---

## 4. The load-bearing decisions (don't relitigate)

These were settled after the initial consistency review. Changing one cascades — see
AGENTS.md §11 for the full list with affected docs.

- **Trace edge direction:** source → target, with satisfy/verify/realize pointing FROM the
  design/test element TO the requirement; deriveReqt parent → child. Authoritative in
  architecture/04 §Relation taxonomy.
- **`.smith` is a single SQLite file** (not a directory).
- **SQLite is authoritative;** the petgraph projection is a cache, rebuilt on load.
- **Write-through persistence:** every command commits immediately; no autosave timer.
- **`«copy»` is excluded from RTM** (provenance only).
- **Verified requires a passing test** (not just an existing `«verify»`).
- **Tech stack:** Tauri 2 + React + TS + Konva (renderer-only) + ELK.js (layout); SQLite +
  petgraph; rmcp on axum for MCP. Fallback (JointJS) and contingency (egui) documented but
  NOT the v1 choice.

---

## 5. Conventions

- **RFC 2119** keywords (MUST, SHOULD, MAY) carry their standard meaning.
- **Stable IDs** — every normative requirement has a `[REQ-<DOMAIN>-<NNN>]` ID. Tests and
  feature requests trace back to these IDs. Never renumber; if you must insert, use a letter
  suffix (`REQ-MM-001a`).
- **Status banner** in each doc's frontmatter: `DRAFT` < `REVIEW` < `STABLE`. Don't implement
  against a `DRAFT` doc.
- **Cross-references** use relative markdown links with the file path. Verify links resolve
  before marking a doc `STABLE`.
- **External references** use full URLs with a one-line citation. Don't re-derive algorithm
  math; link to the authoritative source (P6).
- **`grep` is your friend.** To find where a REQ is defined: `grep -rn '\[REQ-MM-001\]' docs/`.
  To find all mentions of a concept: `grep -rn 'trace graph' docs/`.

---

## 6. When you get stuck

- **Ambiguous spec?** Climb the decision ladder (03-design-principles). Then check AGENTS.md
  §11 (decisions) and §12 (open questions). Still stuck? Ask the human — don't guess on
  load-bearing decisions.
- **Two docs contradict?** Trust architecture/04 for trace direction, architecture/03 for
  persistence, uml-model/06 for diagram-as-view, uml-model/01 for the metamodel. File the
  contradiction as a spec bug and fix the non-authoritative doc.
- **Need to add a feature the spec doesn't cover?** Draft a new section in the relevant
  subsystem doc, mark it `DRAFT`, add REQ-* IDs, update AGENTS.md, and dispatch a reviewer
  subagent before claiming `STABLE`.
- **Unsure which algorithm to use?** Look at analytics/02 (it names each algorithm + links
  the reference). Don't derive — reference (P6).

---

## 7. Tooling notes

- **No build step for the spec.** It's markdown. Validate links with any markdown link-checker
  if you want; otherwise `grep` + `read` suffice.
- **AGENTS.md is auto-generated in spirit.** When the spec changes, re-extract the affected
  index sections (the file's header describes the method). A `docs/scripts/build_index.py`
  is a TODO; until then, manual re-extraction via `grep` is fine.
- **Don't create new top-level docs** without adding them to 00-README.md's map AND AGENTS.md's
  outline. Orphan docs are how specs rot.

---

## 8. History (so you don't repeat it)

The initial spec was written by a 6-scout research team + main author, then consistency-
reviewed by a reviewer subagent. The review found 2 BLOCKERs (both about trace edge direction
contradictions across 5 docs) + 9 MAJOR + 10 MINOR issues. Three fix agents propagated the
settled convention. **The lesson: the spec is densely cross-referenced; a change to one
definition almost always requires parallel changes in 2–5 other docs.** Always grep for
mentions before declaring a change done.
