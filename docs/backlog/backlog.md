---
title: "Backlog — all tasks, dependencies, and the flow per item"
---
# Backlog — all tasks, dependencies, and the flow per item

One page per task in this folder, following the [[t-nnn|task template]], and one row per task in the table below; the next free number is **T-016**. The row is the overview; the page holds what the task is, what to read, and when it is done.

The flow per task:

1. **Tests first:** write the failing test that pins the behaviour.
2. **Implement** until the gates are green (`cargo fmt --check`, `cargo check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`).
3. **Update vault and backlog:** the pages the task touches, the task's status (page and row), then [[status|Status]].

**Scope:** `game` tasks are done once in each new game made from the template, `template` tasks improve the template itself.

| Task | What | Scope | Depends on | Status |
| --- | --- | --- | --- | --- |
| [[t-001-adapt-the-template\|T-001]] | Adapt the template to the new game: names, spec, README | game | — | open |
| [[t-002-decide-the-template-defaults\|T-002]] | Decide every template default: accept, replace or drop | game | T-001 | open |
| [[t-003-replace-the-example-game\|T-003]] | Replace the example game with the game's core loop | game | T-002 | open |
| [[t-004-actions-and-input\|T-004]] | The game's actions and their input actions | game | T-003 | open |
| [[t-005-visibility-filter\|T-005]] | The game's visibility filter | game | T-003 | open |
| [[t-006-presentation\|T-006]] | Presentation: camera, visuals, HUD for the game | game | T-004, T-005 | open |
| [[t-007-keep-determinism-green\|T-007]] | Keep the determinism tests green under every change | game | all | open |
| [[t-008-client-save-and-load\|T-008]] | Client: save and load, stored per platform | template | — | done |
| [[t-009-headless-benchmark\|T-009]] | Headless benchmark: ticks per second | template | — | done |
| [[t-010-client-run-lifecycle\|T-010]] | Client: back to the menu, fresh runs, local player from setup | template | — | done |
| [[t-011-rename-script\|T-011]] | A script that renames the crates for a new game | template | — | done |
| [[t-012-non-ascii-font\|T-012]] | A font asset for texts beyond ASCII | template | — | open |
| [[t-013-ci-web-build-of-the-client\|T-013]] | CI: build the client for the web | template | — | open |
| [[t-014-overlay-hash-only-when-visible\|T-014]] | Debug overlay: compute the state hash only while visible | template | — | open |
| [[t-015-storage-backends-for-mobile-and-web\|T-015]] | Client storage backends for Android, iOS and the web | template | T-008 | open |

Status is `open`, `in progress` or `done`. Done tasks stay in the table until the milestone closes; then their rows move to the milestone's implementation log and their pages are deleted.
