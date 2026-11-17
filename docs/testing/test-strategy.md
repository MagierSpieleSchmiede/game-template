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
| Files | Saves continue runs exactly; damaged, edited, foreign and outdated files are refused | `crates/game_sim/tests/files.rs`, `src/file.rs` |
| Client saves | Save, advance, load reaches the saved hash; refusals leave the world alone and have a localised notice; stores round-trip and keep slots inside their folder | `crates/game_client/src/{saving,storage}.rs` |
| Regression | A committed replay still reproduces on every platform | `crates/game_sim/tests/regression.rs` |
| Headless | The sim runs without window or renderer | [[headless-integration-test\|Headless integration test]] |
| Performance | Simulation and rendering benchmarks | [[benchmarks\|Benchmarks]] |
| Platforms | CI checks the sim for wasm32, Android, iOS | [[platforms\|Platforms]] |
| Toolchain | CI checks the workspace with the declared `rust-version` too, and builds `main` after every merge | `.github/workflows/ci.yml` |

Principles:

- **Tests first:** the failing test pins the behaviour before the implementation.
- The simulation is tested headless; nothing gameplay needs a window.
- The presentation's logic (bindings, localisation, interpolation, save and load) is unit-tested; the rendering itself is exercised by playing and by platform CI.
- Every test that touches randomness seeds its `SimRng`; every test that touches time counts ticks.

The gates before every commit: `cargo fmt --check`, `cargo check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
