---
title: "Test strategy — what is tested how, and where the boundaries are"
---
# Test strategy — what is tested how, and where the boundaries are

| Layer | What | Where |
| --- | --- | --- |
| Unit | One system or function in isolation | `#[cfg(test)]` next to the code |
| Integration | Whole ticks against a headless app | `crates/game_sim/tests/` |
| Determinism | Same scenario → same end state | [[determinism-test\|Determinism test]] |
| Snapshots, replays, rollback | Restore + re-run, playback, late intents all reach the same state hash | `crates/game_sim/tests/{snapshot,replay,rollback}.rs` |
| Headless | The sim runs without window or renderer | [[headless-integration-test\|Headless integration test]] |
| Performance | Simulation and rendering benchmarks | [[benchmarks\|Benchmarks]] |
| Platforms | CI checks the sim for wasm32, Android, iOS | [[platforms\|Platforms]] |

Principles:

- **Tests first:** the failing test pins the behaviour before the implementation.
- The simulation is tested headless; nothing gameplay needs a window.
- The presentation's logic (bindings, localisation, interpolation) is unit-tested; the rendering itself is exercised by playing and by platform CI.
- Every test that touches randomness seeds its `SimRng`; every test that touches time counts ticks.

The gates before every commit: `cargo fmt --check`, `cargo check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
