---
title: "ADR-0015 — Avian for physics"
---
# ADR-0015 — Avian for physics

**Status:** default (ADR-0012); splits the Avian part off ADR-0011, whose Rust-and-Bevy part stays accepted

**Context:** Whether a game needs physics, and which, depends on the genre: a racing game needs vehicle dynamics; a shooter needs ray and shape casts; an RTS or an RPG may need only simple collision checks, or none. Physics inside a deterministic simulation must run single-threaded and in a fixed order.

**Decision:** Avian (`avian3d`) is a dependency of `game_sim`, without its `parallel` and `debug-plugin` features, for collision and spatial queries. The example game does not use it yet.

**Rejected alternatives:** *Rapier directly* (Avian integrates with Bevy's ECS and schedules), *no physics dependency* (every 3D game would start by choosing one; the wasm/mobile build checks would not cover it).

**Consequences:** The template proves Avian builds for every target in CI. Costs: compile time for a dependency the example does not use.

**To replace:** remove `avian3d` from the workspace and `crates/game_sim/Cargo.toml` (or switch to `avian2d` for a 2D game), run `cargo clean` once, and update [[platforms|Platforms]].
