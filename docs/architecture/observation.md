---
title: "Observation — the single code path for reading state"
---
# Observation — the single code path for reading state

HUD, AI, add-ons and the network never query the ECS world directly. They all read through the **one observation interface**: a function that snapshots the simulation state a given team is allowed to see, with a visibility filter (ADR-0007).

Why one code path:

- **Visibility is a simulation concern:** what a team cannot see, it must not receive — one filter, one place to audit.
- **AI and RL** train on exactly what a player sees, no more.
- **The network** sends exactly what the observation returns; no second, leakier path.

In code: `game_sim::observation::observe(world, viewer)` returns the tick and the game's `GameObservation`; the visibility filter is the game's `game::observe` (the example shows everything). The client's presentation reads only this, once per tick, and draws between the last two observations.

Rules: the observation is read-only, returns plain data (serialisable, no `Entity`), and is versioned like the intent. Whoever touches visibility or the network reads [[security|Security]] first.
