---
description: Decompose a slice from docs/99-implementation-guide.md into GitHub issues (tickets only, no workers)
---

Dispatch the `team-lead` agent (task tool, agent: "team-lead") with this assignment:

Decompose **Slice $1** from `docs/99-implementation-guide.md` into GitHub issues on
`mjaric/smith`. If `$1` is empty, ask the user which slice before proceeding. The repo owner
waived the DRAFT-status gate for the implementation loop.

Context the agent must follow:

- Decompose-only: create issues per `.github/ISSUE_TEMPLATE/implementation-task.md`
  (labels `impl` + `slice-N`, wire `--blocked-by`, complete acceptance criteria with one
  named test per `REQ-*` plus the zero-warnings gate).
- Board: items auto-add via workflow; verify Status=Backlog and Slice=Slice N, correct via
  GraphQL if needed. Board contract is in `.omp/agents/team-lead.md`.
- No workers, no PRs, no code, nothing promoted to Ready.
- Final report: issue list with numbers, dependency graph, REQ coverage table, root issues.
