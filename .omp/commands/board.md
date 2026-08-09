---
description: Show the Smith board state grouped by Status (optionally filtered)
---

Print the current state of the **Smith** project board:

1. GraphQL `node(id: "PVT_kwHOAAT9884Bf1hn") { ... on ProjectV2 { items(first: 100) ... } }`
   with content as Issue (number, title, state) and single-select field values.
2. Render one table grouped by Status (Backlog, Ready, In progress, In review, Done):
   issue #, title, Slice, open blockers.
3. List open `needs-decision` issues separately, and flag any card whose Status contradicts
   GitHub state (e.g. closed issue not in Done).

If `$1` is given (e.g. `ready`, `slice-1`, `blocked`), narrow the output to that group.
