---
title: "ADR-0001 — Tick simulation in FixedUpdate instead of frame deltas"
---
# ADR-0001 — Tick simulation in FixedUpdate instead of frame deltas

**Status:** accepted

**Context:** The game must be testable without a window, replayable, and later run as rollback netcode and RL training. Frame-dependent simulation is none of that: results depend on the machine, and tests would need a renderer.

**Decision:** The entire gameplay runs in Bevy's `FixedUpdate` as a tick simulation: fixed timestep, one tick = `1 / TICKS_PER_SECOND` seconds. Every gameplay system lives in the tick chain (`schedule.rs`).

**Rejected alternative:** Simulating with the frame delta in `Update`. Simpler at first, but results differ per frame rate, replays and rollback become impossible, and tests need a window.

**Consequences:** Determinism by construction; headless tests, replays and rollback become possible. Costs: animation must interpolate between ticks; designers think in ticks, not seconds.
