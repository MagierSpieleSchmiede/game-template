---
title: "ADR-0011 — Rust with Bevy and Avian"
---
# ADR-0011 — Rust with Bevy and Avian

**Status:** accepted

**Context:** The architecture needs a game engine that runs gameplay headless in a fixed timestep, serialises state cheaply, builds for desktop, mobile and web, and stays debuggable down to the source. Proprietary engines fight the deterministic-simulation design; most others do not reach the platforms.

**Decision:** **Rust** with **Bevy** (ECS, app scheduling, rendering, assets) and **Avian** (physics queries; kinematic movement, shape casts). No gameplay logic in visual scripting; everything is Rust in this repository.

**Rejected alternatives:** *Godot/Unity* (determinism and headless testing fight the engine; licensing), *raw Rust without an engine* (renderer, input and asset pipelines would be rebuilt for years).

**Consequences:** One language for simulation and presentation; the ECS fits the intent/observation model perfectly; the whole toolchain is free and inspectable. Costs: Bevy moves fast — APIs change between versions and migrations are regular work; platform-specific edge cases are ours to solve.
