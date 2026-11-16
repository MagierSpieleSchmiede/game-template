---
title: "Determinism — same seed, same inputs, same end state"
---
# Determinism — same seed, same inputs, same end state

The contract: **same start state, same inputs, same seed → same end state, byte for byte.** It is what makes the game testable without a window, replayable, and the foundation for [[network|rollback netcode]] and RL training.

What it takes:

- **Ticks, not seconds** (ADR-0003): all durations are integer ticks; `TICKS_PER_SECOND` only converts per-second balance rates.
- **One seeded RNG** (ADR-0004): `SimRng`, Xoshiro256\*\* with SplitMix64, seeded from the scenario. No other randomness — enforced by `clippy.toml`.
- **No hash maps** in the simulation: `HashMap`/`HashSet` iterate in random order; `BTreeMap`/`BTreeSet` instead (also enforced by `clippy.toml`).
- **Ordering by `SimId`** (ADR-0008): never by `Entity`.
- **Floating point discipline:** the same operations in the same order on all platforms; cross-platform floating-point determinism for real PvP is an open question (see the problem log).
- **No clocks:** no `Instant::now`, no `SystemTime::now` in the simulation; count ticks.

How it is tested: [[determinism-test|Determinism test]]. What it enables: [[network|Network]].
