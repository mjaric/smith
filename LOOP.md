# Smith Implementation Loop — Usage Guide

Two ways to drive the loop: **manual** (you issue each command) and **`/loop`** (omp
re-runs a command automatically after every yield). Both use the same machinery — GitHub is
the state machine, the board is the control surface, you merge.

Prerequisites (already done once; listed for recovery):
`gh` authenticated; `PROJECT_TOKEN` secret set (Projects write scope); board **Smith**
(`github.com/users/mjaric/projects/3`); branch protection on `main` (PR + green CI
required); `.omp/agents/team-lead.md` and `.omp/commands/*.md` in the repo.

## Concepts

```
docs/ spec  ──decompose──▶  issues (milestone = slice)
                               │ impl + slice-N labels
                               ▼
                    board Smith (Status/Slice fields)
                               │ dispatch (verified unblocked)
                               ▼
              worker in isolated worktree (branch impl/N-slug, TDD)
                               │ draft PR "Fixes #N"
                               ▼
                    CI (Rust + Frontend) ──▶ review ──▶ you merge ──▶ Done
```

- **Milestones** = which slices remain + progress (`/milestones` on GitHub).
- **Board Smith** = per-task status.
- **`needs-decision` issues** = the only thing that pauses the loop; everything else is
  mechanical.

## Manual mode

Start a session in the repo root. Commands are in `.omp/commands/`; autocomplete on `/`.
If commands changed upstream, start a **new** session (they load at session start).

| Command | Effect |
|---|---|
| `/decompose 2` | TeamLead reads Slice 2 spec, creates issues with `--blocked-by`, acceptance criteria, labels, milestone. No workers. |
| `/board` | Board snapshot grouped by Status (also `/board ready`, `/board slice-1`). |
| `/dispatch 3` | Verify #3's blockers, move card to In progress, spawn one TDD worker in a worktree; worker opens draft PR `Fixes #3`. |
| `/round` | Full round: dispatch every unblocked Ready issue (max 4), review open PRs with the `reviewer` agent, sync cards, promote newly unblocked issues. |
| `/review 4` | Reviewer agent checks PR/issue #4 against its acceptance criteria and the anti-patterns list. |
| `/decide 12 Option B: …` | Record your decision on a `needs-decision` issue, close it, propagate to spec, report unblocked issues. |

Typical manual day:

```
/round              # move everything that can move
/board ready        # what's next
/dispatch 4         # or drive one task at a time
/review 12          # when a worker reports its PR
# merge the PR in the GitHub UI when CI is green and review is clean
/decompose 3        # when the next slice's issues are needed
```

You merge PRs yourself (`gh pr merge N --squash --delete-branch` or the UI). Issues close
automatically via `Fixes #N`; cards move to Done via the board workflow.

## `/loop` mode

`/loop` is omp's built-in: **while enabled, the prompt you send re-submits after every
yield.** Esc cancels the current iteration; `/loop` again disables.

```
/loop 3 /round        # run up to 3 full rounds, one per yield
/loop /round          # keep going until you stop it (Esc, then /loop)
```

Each iteration is one complete `/round` (dispatch → workers yield → review → sync).
Because merging stays human and `main` requires green CI, an unattended loop can dispatch,
implement, and review — but it cannot ship. That is the intended safety boundary.

Recommended ramp:

1. `/loop 1 /round` — watch one full iteration end to end.
2. Fix any process surprises (board desync, flaky worker), then `/loop 2 /round`.
3. Only then consider leaving `/loop /round` running for a longer session.

The loop self-pauses on decisions: TeamLead converts ambiguities into `needs-decision`
issues and refuses to guess, so an unattended round simply reports "awaiting decision" and
stops dispatching that subtree. Check after each yield:

```
/board
gh issue list --label needs-decision --state open
```

## Escalation points (human required)

- `needs-decision` issues → `/decide N <decision>`
- CI red after a worker retry → inspect `gh run view <id> --log-failed`
- Review findings a worker cannot resolve → you arbitrate
- Spec ambiguity beyond the decision ladder → Mode B refinement (`docs/CONTRIBUTING.md`)

## Recovery

- Board wrong vs GitHub: `/round` re-syncs cards from issue state.
- Stale worktree from a dead worker: `git worktree list`, remove, re-`/dispatch N`.
- Workflow failure (auto-add): `gh run list --workflow add-to-project.yml`,
  `gh run view <id> --log-failed`.
- Milestone missing for a slice: `/decompose` creates it on demand.
