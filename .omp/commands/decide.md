---
description: Record a human decision on a needs-decision issue, close it, and report unblocked dependents
---

Resolve needs-decision issue **#$1**. The decision (owner's words): $ARGUMENTS

1. Read the issue; confirm the `needs-decision` label. If `$ARGUMENTS` contains no decision
   text beyond the issue number, ask the user for the decision before continuing.
2. Update the issue body's `## Decision` section with the decision text, then close the
   issue with a comment quoting it.
3. If the decision settles or changes spec content, dispatch the `team-lead` agent for a
   Mode B refinement pass: propagate the decision into the affected `docs/` file(s), update
   `docs/AGENTS.md` (§11 decision map / §12 open questions), and report which REQ/sections
   changed.
4. Inspect the blocked-by graph and report which issues are now unblocked; recommend Ready
   promotion for those whose acceptance criteria are complete.
