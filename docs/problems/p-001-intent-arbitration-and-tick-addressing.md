---
title: "P-001 — Networked intents need a tick address and an arbitration rule"
---
# P-001 — Networked intents need a tick address and an arbitration rule

`IntentBuffer` collects intents "for the next tick" in arrival order; `TickIntents` sorts them by actor, but keeps arrival order for several intents of the **same actor** (`crates/game_sim/src/intent.rs`). Locally that is deterministic, because there is one arrival order. Over a network it is not: two machines can receive two intents for the same actor in different orders, or in different ticks, and then diverge. Replay and rollback also have to reconstruct *which tick* an intent belonged to from where it was recorded, not from the intent itself.

Every networked model (rollback, lockstep, server-authoritative with prediction) needs two things the template does not define yet: **a tick address** on each intent (the tick it applies to, separate from when it arrived), and **an arbitration rule** for the same actor in the same tick (one intent per actor per tick, a sequence number, or the server decides). Single-player and local multiplayer are not affected.

Next step: decide both as part of the network model when a game picks one (ADR-0006 or its replacement, with [[network|Network]] and [[security|Security]]); until then, no code change in the template.
