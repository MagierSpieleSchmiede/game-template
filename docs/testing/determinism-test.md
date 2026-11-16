---
title: "Determinism test — same scenario, same end state"
---
# Determinism test — same scenario, same end state

The contract of the whole architecture, as an executable check (`crates/game_sim/tests/determinism.rs`):

1. Build **two headless apps** from the **same scenario** (same seed, same units).
2. Feed **identical scripted intents** into both.
3. Step both the same number of ticks.
4. Assert: every unit's position and health are **exactly equal**, and the RNG state agrees.

A second test asserts that **different seeds diverge** — otherwise the test above would pass trivially.

Rules:

- The test must stay green under **every** change to the simulation; a red determinism test blocks the commit, no exceptions.
- Extend it as the state grows: every field that desyncs PvP belongs in the comparison.
- Cross-platform determinism (same run on x86 and ARM) is a separate, open question (see the [[problem-log|Problem log]]).
