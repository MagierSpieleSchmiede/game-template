---
title: "ADR-0002 — Intent as serialisable data instead of raw input"
---
# ADR-0002 — Intent as serialisable data instead of raw input

**Status:** accepted

**Context:** Input reaches the simulation from more than one source: a local player, an AI, an add-on, a replay file, the network. Raw `Res<Input<KeyCode>>` in simulation systems would couple the simulation to one device and make all other sources impossible.

**Decision:** The presentation (or AI, add-on, network) produces an **intent**: plain, `serde`-serialisable data — no Bevy types, no entity handles. The simulation consumes intents and nothing else. This is the single thing the presentation writes back.

**Rejected alternative:** Reading `Input<KeyCode>` directly in simulation systems. Fewer indirections, but device-bound, not serialisable, not trainable, not networkable.

**Consequences:** One input model for player, AI, add-on, replay and network; rollback can record and replay intents. Costs: an extra layer between device and simulation; intents need a stable format once they travel the network.
