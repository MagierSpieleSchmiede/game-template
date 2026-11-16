---
title: "Backlog — all tasks, dependencies, and the flow per item"
---
# Backlog — all tasks, dependencies, and the flow per item

Every task is one line in the table, with its dependencies. The flow per item:

1. **Tests first:** write the failing test that pins the behaviour (ADR-0026).
2. **Implement** until the gates are green (`cargo fmt --check`, `cargo check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`).
3. **Update vault and backlog:** the pages the task touches, then `status.md`.

| ID | Task | Depends on | Read for that |
| --- | --- | --- | --- |
| T-001 | Adapt this template to the new game: names, spec, README | — | [[spec-001-prototype\|Spec 001]] |
| T-002 | Define the game's core loop and units | T-001 | [[simulation-model\|Simulation model]] |
| T-003 | Extend `Intent`/`Action` for the game's actions | T-002 | [[intent-and-controls\|Intent & controls]] |
| T-004 | Observation interface with visibility filter | T-002 | [[observation\|Observation]] |
| T-005 | Presentation: camera, HUD, localisation keys | T-003, T-004 | [[project-structure\|Project structure]] |
| T-006 | Keep the determinism test green under every change | all | [[determinism-test\|Determinism test]] |

New tasks get the next free ID; done tasks stay in the list with a check mark until the milestone closes.
