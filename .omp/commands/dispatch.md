---
description: Dispatch a worker subagent for one issue number (verifies blockers, moves card to In progress)
---

Dispatch a worker for issue **#$1** on `mjaric/smith` using the `task` tool:

1. Verify: `gh issue view $1 --repo mjaric/smith --json title,state,blockedBy,labels`.
   Stop and report if the issue is closed or has open blockers.
2. Move its board card to **In progress** (GraphQL `updateProjectV2ItemFieldValue`; project
   `PVT_kwHOAAT9884Bf1hn`, Status field `PVTSSF_lAHOAAT9884Bf1hnzhaFhXM`, option id
   `47fc9ee4`). Add the item first if it is missing from the board.
3. Spawn the worker: task tool, `agent: "task"`, isolated worktree. Worker contract:
   - Branch `impl/$1-<slug>`; never touch `main`.
   - TDD red-green-refactor: failing test(s) for every `REQ-*` in the issue first.
   - Gate before yielding: `cargo test`, `cargo clippy --all-targets -- -D warnings`,
     `bun run test`, `bunx oxlint` — zero warnings, no placeholders.
   - Open a draft PR with `Fixes #$1`; report the PR URL and the test names.
