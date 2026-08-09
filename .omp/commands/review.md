---
description: Review a PR (or the PR linked to an issue) with the reviewer agent against its acceptance criteria
---

Review the change referenced by **$1**:

1. If `$1` is a PR number, use `pr://$1/diff/all`. If it is an issue number, find the linked
   PR from `gh issue view $1 --repo mjaric/smith` (cross-references / closing references)
   and use that PR's diff.
2. Fetch the issue's Spec references and Acceptance sections as the review contract.
3. Dispatch the bundled `reviewer` agent (task tool, agent: "reviewer") with the diff and
   contract. Review axes:
   - Does the change satisfy each cited `REQ-*` (with a real test)?
   - Zero-warnings gate: cargo test / clippy -D warnings / vitest / oxlint.
   - Anti-patterns from `docs/99-implementation-guide.md`: model facts on diagrams (P1),
     UI capability without MCP equivalent (P3), re-derived algorithms (P6), destructive ops
     bypassing undo (P9), color-only meaning (REQ-DS-002).
4. Report findings with file:line references by severity. Clean: recommend merge. Otherwise
   list exactly what the worker must fix.
