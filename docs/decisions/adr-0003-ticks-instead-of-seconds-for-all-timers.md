---
title: "ADR-0003 — Ticks instead of seconds for all timers"
---
# ADR-0003 — Ticks instead of seconds for all timers

**Status:** accepted

**Context:** Cooldowns, durations, regeneration — all gameplay timing must survive a rollback (rewind and re-simulate) and produce identical results everywhere. Floating-point seconds accumulate rounding differences per frame and per platform.

**Decision:** All durations in the simulation are integer **ticks**. `TICKS_PER_SECOND` exists only to convert per-second balance rates into per-tick values.

**Rejected alternative:** `f32`/`f64` seconds with `Timer` resources. Reads naturally, but accumulates error, is hard to rewind, and desyncs across platforms.

**Consequences:** Exact, rewindable timing; balance rates need a documented conversion point. Costs: designers and players think in seconds, so balance data carries both.
