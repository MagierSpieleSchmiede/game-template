---
title: "Determinism test — same scenario, same end state"
---
# Determinism test — same scenario, same end state

The contract of the whole architecture, as an executable check (`crates/game_sim/tests/determinism.rs`):

1. Build **two headless apps** from the **same scenario** (same seed, same units).
2. Feed **identical scripted intents** into both — moves in several directions, long enough to reach the arena edge.
3. Step both the same number of ticks.
4. Assert, in `SimId` order: every unit's position, velocity and health are **exactly equal**, and so is the `state_hash` (which includes the RNG state).
5. Assert that the moving units **did move**: two runs in which nothing happens agree trivially.

Two more tests guard against trivial passes and against platform drift:

- **Different seeds diverge.**
- **Committed hash:** the scripted run's `state_hash` (`crates/game_sim/src/checksum.rs`) must equal a value committed in the test. CI runs it on Linux, Windows and macOS, so it checks the same run across platforms, not only within one process. A deliberate change to the simulation that changes the value updates it in the same commit, saying why.

Rules:

- The test must stay green under **every** change to the simulation; a red determinism test blocks the commit, no exceptions.
- Extend it as the state grows: every field that desyncs PvP belongs in the comparison.
- Extend `state_hash` as the state grows, in the same way.
- The committed hash covers the desktop CI runners only (x86-64 and ARM64 macOS); mobile and wasm are only built, not run. Full cross-platform floating-point determinism stays an open question (see the [[problem-log|Problem log]]).
