---
title: "Backlog — all tasks, dependencies, and the flow per item"
---
# Backlog — all tasks, dependencies, and the flow per item

Every task is one line in the table, with its dependencies. The flow per item:

1. **Tests first:** write the failing test that pins the behaviour.
2. **Implement** until the gates are green (`cargo fmt --check`, `cargo check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`).
3. **Update vault and backlog:** the pages the task touches, then `status.md`.

| ID | Task | Depends on | Read for that |
| --- | --- | --- | --- |
| T-001 | Adapt this template to the new game: names, spec, README | — | [[spec-001-prototype\|Spec 001]] |
| T-002 | Decide every template default: accept, replace or drop (ADR-0006, 0013–0017) | T-001 | [[decisions\|Decisions]], ADR-0012 |
| T-003 | Replace the example game in `game_sim::game` with the game's core loop, keeping the contract | T-002 | [[simulation-model\|Simulation model]], ADR-0013 |
| T-004 | The game's `Action`s and their input actions | T-003 | [[intent-and-controls\|Intent & controls]] |
| T-005 | The game's visibility filter in `game::observe` | T-003 | [[observation\|Observation]] |
| T-006 | Presentation: camera, visuals, HUD for the game | T-004, T-005 | [[project-structure\|Project structure]] |
| T-007 | Keep the determinism test green under every change; re-commit its hash only on deliberate changes | all | [[determinism-test\|Determinism test]] |

New tasks get the next free ID; done tasks stay in the list with a check mark until the milestone closes.
