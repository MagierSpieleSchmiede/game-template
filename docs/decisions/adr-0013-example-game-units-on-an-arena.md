---
title: "ADR-0013 — The example game: units moving on an arena"
---
# ADR-0013 — The example game: units moving on an arena

**Status:** default (ADR-0012)

**Context:** The genre-neutral core (ticks, intents, snapshots, replays, rollback, observation) needs real gameplay to be exercised: something that moves, uses randomness, and reaches a boundary, so the determinism tests can fail.

**Decision:** The template ships a minimal example: a few units spawned on a circle, moving on a disc-shaped arena floor (the x/z plane) by `Move`/`Idle` intents, with the stages `Intent → Values → Movement → Damage → Cleanup` inside `SimSet::Update`. The client draws it from above, one capsule per unit, and steers unit 1 with the move axis.

**Rejected alternatives:** *A bare core with no gameplay* (the determinism tests would pass trivially), *a full sample game* (more to delete, and it would bias every new game towards its genre).

**Consequences:** Everything in the template is tested against real gameplay; the `game` module shows what the core needs from a game. Costs: none beyond deleting it.

**To replace:** rewrite `crates/game_sim/src/game/` keeping the contract in its `mod.rs`; rewrite `crates/game_client/src/presentation.rs`; replace the scripted runs in `crates/game_sim/tests/determinism.rs` and `tests/regression.rs` with ones that exercise the new systems, commit the new hash and re-record the regression replay; adapt `tests/simulation.rs`, `tests/snapshot.rs`, `tests/replay.rs`, `tests/rollback.rs`, `tests/observation.rs` to the new `Setup` and `Action`; update the tick chain in [[simulation-model|Simulation model]].
