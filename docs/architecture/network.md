---
title: "Network — rollback with the server as referee"
---
# Network — rollback with the server as referee

PvP is designed, not built yet. The template default (ADR-0006, decided per game): **rollback netcode**, with a **server as referee**. The rollback itself exists headless (`game_sim::rollback`, tested); transport and server do not.

```text
client A ──intent──▶ ┌─────────┐ ◀──intent── client B
   ▲                  │ server  │              ▲
   └──observation──── │ referee │ ──observation┘
```

- **Each client simulates itself** — locally, at full frame rate, predicting ahead on its own intent.
- **On a late intent**, the client rewinds to the last confirmed tick, applies it, and re-simulates to the present: rollback. This is only possible because the simulation is deterministic ([[determinism|Determinism]]) and intents are plain data ([[intent-and-controls|Intent & controls]]).
- **The server is the referee:** it validates intents, keeps the authoritative tick, and decides conflicts. When client and server disagree, the server wins.
- **Each client sees only what it may see:** the server sends the observation with the visibility filter of that team ([[observation|Observation]], [[security|Security]]).

Open questions live in the problem log: intent tick addressing and arbitration ([[p-001-intent-arbitration-and-tick-addressing|P-001]]), transport crate, hit rating under latency, direct client connections.
