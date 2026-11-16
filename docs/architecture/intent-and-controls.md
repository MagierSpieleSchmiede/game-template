---
title: "Intent & controls — the one thing the presentation writes back"
---
# Intent & controls — the one thing the presentation writes back

The presentation never touches simulation state. It produces an **intent**: plain, serialisable data (`serde`), no Bevy types, no entity handles (ADR-0002). That is what makes intents usable by a player, an AI, an add-on, a replay, and the network alike.

```rust
struct Intent { actor: SimId, action: Action }      // core: game_sim::intent
enum Action { Move { x: f32, z: f32 }, Idle }      // the game's: game_sim::game (example, ADR-0013)
```

Rules:

- Intents are queued in the `IntentBuffer`. At the start of each tick (`SimSet::Input`) they are frozen into `TickIntents`, **sorted by actor** (arrival order kept per actor): arrival order differs between machines, `SimId` order does not. Game systems read `TickIntents` only.
- In the client, the input layer (ADR-0016) turns devices into actions; the presentation turns actions into intents, once per tick, in `FixedPreUpdate`.
- Input is mapped through **actions**, never raw key codes; keyboard/mouse and gamepad are equal.
- Extend `Action` with the game's abilities; once variants travel the network, they are stable (see [[serialization-and-ids|Serialization & IDs]]).
- No visible text in code; everything goes through localisation keys.
