---
status: DRAFT
---

# Analytics 04 — Quality Rules Engine

Smith's model-quality rule engine. Defines how rules are expressed, evaluated, and surfaced.
The defect taxonomy (D1–D7) lives in
[analytics/03-orphan-and-coverage-analysis.md](./03-orphan-and-coverage-analysis.md); this
document specifies the **mechanism** that runs them (and any future user-defined rules).

## Rule definition

`[REQ-QA-001]` A quality rule is a declarative predicate over the model state with metadata:

| Field | Type | Meaning |
|-------|------|---------|
| `id` | `RuleId` | Stable code (`D1`–`D7` for built-ins). |
| `name` | `String` | Human label ("Orphan requirement"). |
| `description` | `String` | What it detects + why it matters. |
| `severity` | `Severity` | HIGH / MEDIUM / LOW. |
| `appliesTo` | `MetaclassKind[]` | Which element kinds the rule evaluates. |
| `predicate` | `RulePredicate` | The condition that, when true, flags the element. |
| `incremental` | `bool` | Whether it's maintained incrementally on mutation (vs. on-demand). |
| `fixHint` | `Option<ToolName>` | Suggested tool/prompt to fix (e.g. `deriveUseCases`). |

## Built-in rules

The seven built-in rules, each mapped to its analytics spec:

| ID | Rule | Spec |
|----|------|------|
| D1 | Orphan requirement | [analytics/03](./03-orphan-and-coverage-analysis.md) §D1 |
| D2 | Orphan test | [analytics/03](./03-orphan-and-coverage-analysis.md) §D2 |
| D3 | Orphan element (corrupt/isolated) | [analytics/03](./03-orphan-and-coverage-analysis.md) §D3 |
| D4 | Uncovered element (no diagram) | [analytics/03](./03-orphan-and-coverage-analysis.md) §D4 |
| D5 | Circular trace | [analytics/03](./03-orphan-and-coverage-analysis.md) §D5 |
| D6 | Incomplete chain | [analytics/03](./03-orphan-and-coverage-analysis.md) §D6 |
| D7 | Disconnected subgraph | [analytics/03](./03-orphan-and-coverage-analysis.md) §D7 |

`[REQ-QA-002]` These seven are built-in, always enabled (cannot be disabled), and shipped with
Smith. Their severities are fixed (per [analytics/03](./03-orphan-and-coverage-analysis.md)).

## Evaluation model

`[REQ-QA-003]` Rules run in two modes:
- **Incremental** (D1–D6): re-evaluated for affected elements on each mutation. Cheap (degree
  + reachability checks). The defect state per element is kept current at all times.
- **On-demand** (D7, and graph-wide analyses like centrality): run when the user/agent opens
  the analysis view; results cached until the next mutation invalidates them.

`[REQ-QA-004]` Incremental evaluation scope (which elements to re-check on a mutation):
- Element field change → re-check that element against all rules whose `appliesTo` includes
  its kind.
- Relationship add/remove → re-check both endpoints + (for trace rules) re-run reachability
  for affected chains.
- View-reference add/remove → re-check D4 (coverage) for the referenced element.
- Reparent → re-check D3(a) (ownership reachability) for the moved subtree.

`[REQ-QA-005]` Evaluation results are stored as `DefectInstance { ruleId, elementId,
message, data?, suppressed }`. The set of current instances is the issue panel's content.

## Severity & blocking

`[REQ-QA-006]` Severity ordering: HIGH > MEDIUM > LOW. The issue panel sorts HIGH first (per
[ui-ux/04-search-and-analysis-ui.md](../ui-ux/04-search-and-analysis-ui.md) REQ-UI-ISSUE-002).

`[REQ-QA-007]` HIGH-severity defects MAY optionally block project export (configurable in
settings, default OFF — export with warnings). Mutations that would CREATE a HIGH defect
are rejected by default (e.g., adding a trace edge that creates a cycle returns MCP error
`-32003` unless `acceptDefect: true` is passed — see
[mcp/01-server-design.md](../mcp/01-server-design.md) §Error model).

## Suppression

`[REQ-QA-008]` A user MAY suppress a specific defect instance via the issue panel or MCP.
Suppression records a tagged value `smith::suppressDefect { ruleId, until? }` on the element.
Suppressed instances are hidden by default and visible behind a "show suppressed" toggle.

`[REQ-QA-009]` Suppression is per-element-per-rule (not global). Suppressing D4 on element X
does not affect D1 on X.

`[REQ-QA-010]` Suppressions are model data (persisted in the `.smith` file) and travel with
the project — a deliberate "this is fine, I checked" that survives sessions.

## User-defined rules (future)

`[REQ-QA-011]` v1 ships ONLY the seven built-in rules. A user-defined rule system (declarative
predicates, custom severities, project-scoped rule packs) is a v2 candidate; the rule engine
is designed to accept new rules without architectural change.

## MCP surface

`[REQ-QA-012]` Quality state is exposed via:
- `analysis.issues` tool — filterable list of current defects (see
  [mcp/02-tools.md](../mcp/02-tools.md)).
- `smith://analysis/issues` resource — same data, readable + subscribable (see
  [mcp/03-resources.md](../mcp/03-resources.md)).
- Each defect references the offending element by `ElementId` and the rule by `ruleId`.

`[REQ-QA-013]` Agents SHOULD check `analysis.issues` after any mutation to avoid introducing
defects. The prompts (see [mcp/04-prompts.md](../mcp/04-prompts.md)) instruct agents to do so.

## Open questions (resolve before STABLE)

- [ ] Auto-fix actions: should Smith offer one-click fixes (e.g. "create UseCase for orphan
      requirement")? Tentative: yes — link to the relevant prompt/tool; do not auto-execute.
- [ ] Rule customization: allow changing a built-in rule's severity per-project? Tentative:
      no in v1 (fixed severities); user-defined rules in v2 carry their own severities.
