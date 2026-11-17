---
title: "T-017 — Pin that a game resource missing from the snapshot turns the tests red"
---
# T-017 — Pin that a game resource missing from the snapshot turns the tests red

**Status:** done · **Scope:** template · **Depends on:** —

**Read for that:** [[determinism|Determinism]] (complete snapshots), `crates/game_sim/src/snapshot.rs`, `crates/game_sim/tests/snapshot.rs`, the contract in `crates/game_sim/src/game/mod.rs`

**What:** `snapshots_are_complete` claims that state the game forgets to capture, *a resource or a component*, makes it red. For components the example proves that; for resources it does not: the example game has no resource that changes during a run (only `Balance`, which is constant). A real game will have some (score, timers, weather, objectives), and a forgotten one only shows if it feeds back into hashed state within the test's 40 ticks; a resource that is only read by the presentation never shows. Make the contract say that every run-mutable game resource belongs in `GameState` (captured, restored and hashed), and pin with a test that a run-mutable resource left out of the snapshot turns the completeness check red. Keep the example small: a test-only resource is enough if it does not need a gameplay feature.

**Done when:** the contract in `game/mod.rs` names run-mutable resources explicitly, a test fails when such a resource is not captured and passes when it is, and [[determinism|Determinism]] says what the completeness test can and cannot catch.

**Built:** the contract in `game/mod.rs` names run-mutable resources: they belong in `GameState` (captured, restored, hashed, reset by `spawn`). `tests/snapshot.rs` adds a test-only `Countdown` resource that starts drawing from the RNG at tick 50: `resources_left_out_of_the_snapshot_turn_the_check_red` shows the completeness check red when it is left out and green when it is restored with the snapshot. [[determinism|Determinism]] says what the check catches and what it cannot.
