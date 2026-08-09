---
name: Implementation task
about: A decomposed implementation task created by TeamLead from the spec
labels: impl
---

<!-- TeamLead MUST fill every section. A task without complete acceptance criteria does not
     enter "Ready". Scope must fit one crate/module concern per docs/99-implementation-guide.md. -->

## Scope

<!-- Exact crates/files, e.g. "smith-store: closure table maintenance". Explicit non-goals. -->

## Spec references

<!-- Every REQ-*/INV-* this task satisfies, e.g. REQ-PERS-007, INV-MM-001.
     Link to definition site in docs/AGENTS.md reverse-index. -->

- REQ-

## Dependencies

<!-- GitHub blocked-by links to prerequisite issues, e.g. "Blocked by #12".
     Leave empty only if the task is a true root. -->

## Acceptance

<!-- One bullet per REQ-* satisfied: test name asserting the requirement holds.
     Plus the standing gate: cargo test, clippy --all-targets -D warnings,
     vitest, oxlint — zero warnings (docs/CONTRIBUTING.md §2 Mode A step 8). -->

- [ ] REQ-...: verified by `test_name`
- [ ] Cross-cutting acceptance holds (docs/99-implementation-guide.md)
