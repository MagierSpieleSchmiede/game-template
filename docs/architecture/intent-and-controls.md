---
title: "Intent & controls — the one thing the presentation writes back"
---
# Intent & controls — the one thing the presentation writes back

The presentation never touches simulation state. It produces an **intent**: plain, serialisable data (`serde`), no Bevy types, no entity handles (ADR-0002). That is what makes intents usable by a player, an AI, an add-on, a replay, and the network alike.

```rust
struct Intent { unit: SimId, action: Action }
enum Action { Move { x: f32, z: f32 }, Idle }
```

Rules:

- Intents are queued in the `IntentBuffer` and applied in `SimSet::Intent`, in arrival order.
- Input is mapped through **actions**, never raw key codes; keyboard/mouse and gamepad are equal.
- Extend `Action` with the game's abilities; once variants travel the network, they are stable (see [[serialization-and-ids|Serialization & IDs]]).
- No visible text in code; everything goes through localisation keys.
