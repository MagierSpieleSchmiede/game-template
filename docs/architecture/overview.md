---
title: "Overview — the system in one diagram"
---
# Overview — the system in one diagram

```text
        intents (serialisable data)
  ┌──────────────┐              ┌────────────────────┐
  │ Presentation │  ──intent──▶ │    Simulation      │
  │ game_client  │              │ game_sim, FixedUpdate│
  │ camera, HUD, │ ◀─observation─ deterministic tick │
  │ input, audio │              │ chain, seeded RNG  │
  └──────────────┘              └────────────────────┘
        ▲                                 ▲
        │                        confirmed state / rollback
   player input                        (later: server)
```

The one big bet: **gameplay is a deterministic, headless tick simulation** ([[simulation-model|Simulation model]]). Everything else follows from it:

- The presentation is a pure observer; it writes back exactly one thing, the [[intent-and-controls|intent]].
- Everything that reads state — HUD, AI, add-ons, the network — reads through the [[observation|observation]].
- Randomness comes only from the seeded RNG, time only from ticks ([[determinism|Determinism]]).
- Later, PvP runs as [[network|rollback netcode]] with a server as referee, which the determinism makes possible.

Why this architecture: it makes the game testable without a window, replayable, desync-debuggable, and it is the foundation for rollback netcode and RL-trained AI.
