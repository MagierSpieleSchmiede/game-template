---
title: "T-003 — Replace the example game with the game's core loop"
---
# T-003 — Replace the example game with the game's core loop

**Status:** open · **Scope:** game · **Depends on:** [[t-002-decide-the-template-defaults|T-002]]

**Read for that:** [[simulation-model|Simulation model]], ADR-0013

**What:** Rewrite `crates/game_sim/src/game/` for the game, keeping the contract documented in its `mod.rs`; replace the scripted runs of the determinism and regression tests; bump `SIM_VERSION` and re-record the regression replay.

**Done when:** The core is untouched, all tests pass, and the regression replay exercises the game's own systems.
