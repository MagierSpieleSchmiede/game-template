---
title: "ADR-0014 — 60 ticks per second with f32 maths"
---
# ADR-0014 — 60 ticks per second with f32 maths

**Status:** default (ADR-0012)

**Context:** The tick rate and the number type depend on the genre. A shooter or fighting game wants 60 ticks per second or more; racing physics often 120 or more with substeps; a strategy game with thousands of units 10–20, and possibly fixed-point maths so every machine computes bit-identical results.

**Decision:** `TICKS_PER_SECOND = 60` and `f32` (Bevy's `Vec3`), restricted to the operations IEEE 754 defines exactly (`+ - * /`, `sqrt`) in the simulation. CI compares a committed state hash on Linux, Windows and macOS (x86-64 and ARM64).

**Rejected alternatives:** *Fixed-point from the start* (slower to write, no Bevy maths types, and unnecessary for many genres), *Bevy's default of 64 Hz* (no reason for it; 60 matches common displays).

**Consequences:** The example runs smoothly and its runs match bit for bit across the CI platforms. Costs: `f32` determinism holds only as long as the simulation avoids platform maths functions (`sin`, `cos`, `powf`, …) and compiler settings stay the same; see [[determinism|Determinism]].

**To replace:** change `TICKS_PER_SECOND` in `crates/game_sim/src/lib.rs` (and balance rates that assume 60); for fixed-point, replace `Vec3`/`f32` in the game's state, `StateHasher::write_f32` users, and the observation, then commit the new determinism hash.
