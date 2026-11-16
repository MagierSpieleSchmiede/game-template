---
title: "ADR-0005 — The simulation as its own crate"
---
# ADR-0005 — The simulation as its own crate

**Status:** accepted

**Context:** The simulation must run without window, renderer, audio or threads — in tests, on a server, in RL training, and as wasm in the browser. In a single-crate game it is only a matter of time before a gameplay system touches a rendering type, and then none of those environments work anymore.

**Decision:** The simulation is its own workspace crate, `game_sim`, with dependencies limited to `bevy` (core, no rendering features), Avian physics queries and `serde`. It must build for `wasm32-unknown-unknown`. The client (`game_client`) depends on the simulation, never the other way around.

**Rejected alternative:** One crate with feature flags separating "sim" from "client". Cheaper at first, but the boundary erodes and CI cannot enforce it.

**Consequences:** The boundary is enforced by the compiler and CI (`cargo check -p game_sim --target wasm32-…`). Costs: a workspace, and Bevy types that are presentation-only must be mirrored as plain data.
