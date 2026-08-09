---
description: Run one full loop round - dispatch every unblocked Ready issue, review finished PRs, sync the board
---

Dispatch the `team-lead` agent (task tool, agent: "team-lead") to run one execution round on
`mjaric/smith`:

- Query board **Smith** (`PVT_kwHOAAT9884Bf1hn`) for items with Status=Ready; verify their
  blocked-by issues are closed (`gh issue view N --json blockedBy`); dispatch a worker per
  verified issue (max 4 concurrent; isolated worktrees; worker contract from
  `.omp/agents/team-lead.md`).
- PRs awaiting review: run the bundled `reviewer` agent on `pr://<N>/diff/all` against the
  issue's acceptance criteria; route findings back to the worker until clean.
- PR clean + CI green: card to **In review** (option `37cbfc5d`). Merged PRs: verify the
  issue auto-closed (`Fixes #N`), card to **Done** (`98236657`).
- Promote newly unblocked issues to Ready only when their acceptance section is complete;
  otherwise list what is missing.
- Report: dispatched / reviewed / blocked / decisions needed / next recommended action.
