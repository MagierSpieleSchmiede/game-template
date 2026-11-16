---
title: "ADR-0004 — One seeded RNG resource instead of scattered generators"
---
# ADR-0004 — One seeded RNG resource instead of scattered generators

**Status:** accepted

**Context:** Randomness (spread, crits, spawns) must reproduce exactly across runs, clients and platforms, and rewind cleanly in a rollback. A `rand::thread_rng()` anywhere in the simulation destroys all of that.

**Decision:** One seeded resource, `SimRng` (Xoshiro256\*\* seeded via SplitMix64, own implementation, no external RNG crate), seeded from the scenario. It is the **only** randomness source in the simulation; `clippy.toml` forbids every other (`RandomState`, `Instant::now`, `SystemTime::now`).

**Rejected alternative:** `rand` with per-system `SmallRng`s. Convenient, but unseedable as a whole and impossible to rewind or audit.

**Consequences:** Same seed → same sequence everywhere; the RNG state is part of the snapshot for rollback; randomness consumption is auditable in one place. Costs: every draw must go through the resource; the draw order is part of the contract.
