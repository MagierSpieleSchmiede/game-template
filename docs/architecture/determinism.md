---
title: "Determinism — same seed, same inputs, same end state"
---
# Determinism — same seed, same inputs, same end state

The contract: **same start state, same inputs, same seed → same end state, byte for byte.** It is what makes the game testable without a window, replayable, and the foundation for [[network|rollback netcode]] and RL training.

What it takes:

- **Ticks, not seconds** (ADR-0003): all durations are integer ticks; `TICKS_PER_SECOND` only converts per-second balance rates.
- **One seeded RNG** (ADR-0004): `SimRng`, Xoshiro256\*\* with SplitMix64, seeded from the scenario. No other randomness — enforced by `clippy.toml`.
- **No hash maps** in the simulation: `HashMap`/`HashSet` (std's, Bevy's, `hashbrown`'s) iterate in an order that depends on hashing or insertion history; `BTreeMap`/`BTreeSet` instead (enforced by `clippy.toml`, which also forbids `rand` and `getrandom` should they ever become dependencies).
- **Ordering by `SimId`** (ADR-0008): never by `Entity`.
- **Floating point discipline:** the same operations in the same order on all platforms; only operations IEEE 754 defines exactly (`+ - * /`, `sqrt`), no `sin`, `cos`, `powf` and the like, whose results come from the platform's maths library; cross-platform floating-point determinism for real PvP is an open question (see the problem log).
- **No clocks:** no `Instant::now`, no `SystemTime::now` in the simulation; count ticks.
- **One tick rate:** `SimPlugin` sets the fixed timestep to `TICKS_PER_SECOND` (Bevy's default is 64 Hz; the value 60 is a template default, ADR-0014); systems use `TICK_SECONDS`, never `Time`.
- **One seeding point:** `scenario::start_run` seeds `SimRng` from the scenario before the game spawns, for the client and the headless app alike; IDs come from the `SimIdCounter` resource.
- **Intents in actor order:** `TickIntents` is sorted by `SimId`, not by arrival.
- **Complete snapshots:** everything that changes during a run is captured by `Snapshot` (tick, RNG, ID counter, who controls which actor, the game's state); `tests/snapshot.rs` proves it by re-running from a restored snapshot.

How it is tested: [[determinism-test|Determinism test]]. What it enables: [[network|Network]].
