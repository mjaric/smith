---
status: DRAFT
---

# MCP 04 — Prompts

Predefined prompt templates Smith exposes via `prompts/list` and `prompts/get`. Prompts are
invoked by users through the host client (typically as slash commands) and return a
prompt-message sequence the host feeds to its LLM. They encode Smith's modeling expertise.

See [research/02-mcp-transport.md](../research/02-mcp-transport.md) §4.3 for the prompt
protocol primitives.

## Design intent

Prompts are **parameterized recipes** for common modeling tasks. They do not execute work;
they produce a prompt that, when the agent processes it, leads the agent to call Smith's
tools in a productive order. This keeps the human and agent aligned on workflow without
hard-coding behavior in the agent.

`[REQ-MCP-PROMPT-001]` Every prompt MUST declare its `arguments` (name, required, description)
and return `messages` with at least one `user` message containing the templated instructions
and references to relevant Smith resources/tools.

## Prompt catalog

### Modeling & derivation

| Prompt | Arguments | Purpose |
|--------|-----------|---------|
| `deriveUseCases` | `requirementId` (required) | Produce UseCases that `«satisfy»` the requirement. Instructs the agent to read the requirement, propose use cases, create them, and link with `«satisfy»`. |
| `deriveActivity` | `useCaseId` (required) | Produce an Activity that `«realize»`-es the use case. |
| `deriveSequence` | `activityId` (required) | Produce a Sequence (Interaction) realizing the Activity; identify participating classes/operations. |
| `deriveClasses` | `interactionId` (required) | Identify/update Classes and Operations participating in the interaction; link with `«realize»`. |
| `writeTestCase` | `requirementId` (required), `approach?` | Draft a TestCase that `«verify»`-ies the requirement. |

### Analysis & quality

| Prompt | Arguments | Purpose |
|--------|-----------|---------|
| `rtmGapReport` | `packageId?` | Read `smith://analysis/rtm` and produce a gap report: orphan requirements, incomplete chains, unverified requirements. |
| `fixOrphans` | `packageId?` | List orphan requirements / uncovered elements and propose actions (link or delete). |
| `reviewDiagram` | `diagramId` (required) | Read the diagram + its elements; check notation correctness, missing multiplicities, unnamed elements, and suggest fixes. |
| `breakCycles` | none | Read `smith://analysis/issues` (D5); propose edge removals to break trace cycles. |

### Refactoring

| Prompt | Arguments | Purpose |
|--------|-----------|---------|
| `extractPackage` | `elementIds[]` (required), `packageName` (required) | Move selected elements into a new package (reparent). |
| `mergeClasses` | `sourceId`, `targetId` | Merge two classes: move attributes/operations, rewire relationships, delete source. |
| `splitClass` | `classId`, `featureIds[]` | Extract selected features into a new class, linked by association. |

### Import / scaffolding

| Prompt | Arguments | Purpose |
|--------|-----------|---------|
| `scaffoldProject` | `domain` (required), `style?` | Create a starting package structure (Requirements, UseCases, Activities, Sequences, Classes, Tests) for a new project. |
| `importRequirements` | `sourceText` (required), `packageId?` | Parse a requirements document (plain text / markdown) into Requirement elements. |

## Example: `deriveUseCases` prompt body

The returned `messages` for `deriveUseCases({ requirementId: "req-abc" })`:

```jsonc
{
  "description": "Derive use cases satisfying a requirement.",
  "messages": [
    { "role": "user",
      "content": {
        "type": "text",
        "text": "Read the requirement at smith://elements/req-abc. Then:\n\n1. Identify the actor(s) and the system capabilities implied by the requirement.\n2. For each capability, create a UseCase (model.createUseCase) in the /System/UseCases package (or create it if missing).\n3. For each UseCase, create a «satisfy» trace relation (model.createTraceRelation) from the UseCase to requirement req-abc.\n4. After creating, run analysis.issues to confirm no new defects were introduced.\n5. Report the created UseCase IDs and their qualified names.\n\nConventions:\n- UseCase names are verb-phrases (\"Authenticate user\", not \"Authentication\").\n- One UseCase per distinct actor goal.\n- Do not create UseCases for non-functional concerns (those become Requirements with category=performance/security/etc., derived via «deriveReqt»)."
      }
    }
  ]
}
```

`[REQ-MCP-PROMPT-002]` Prompt bodies MUST reference Smith resources and tools by their exact
URIs / names so the agent does not guess.

`[REQ-MCP-PROMPT-003]` Prompts MUST be conservative: they instruct the agent to check
`analysis.issues` after mutations and to report what it did (element IDs created).

## Localization

`[REQ-MCP-PROMPT-004]` v1 ships English prompts. The prompt catalog is data-driven (not
hardcoded), so localization is a matter of adding translations; not in v1 scope.

## Open questions (resolve before STABLE)

- [ ] Should prompts support multi-turn (return an assistant message too, priming the
      model)? Current: no — single user message; let the agent drive.
- [ ] Should the host be able to pass a `style` argument (e.g. terse vs. verbose output)?
      Tentative: yes as an optional arg on analysis prompts.
