---
title: "Simulation model — the invariants and the tick chain"
---
# Simulation model — the invariants and the tick chain

The entire gameplay is a deterministic tick simulation in `FixedUpdate` (ADR-0001). One source per fact: the invariants live here, the reasoning in the ADRs.

## Invariants

1. **One tick, one chain.** Every gameplay system runs in exactly one `SimSet`, in `schedule.rs`, in `FixedUpdate`. Nothing gameplay runs anywhere else.
2. **No frame time.** The simulation never reads wall-clock or frame deltas; all durations are ticks (ADR-0003).
3. **One randomness source.** All randomness draws from `SimRng`; `clippy.toml` forbids everything else (ADR-0004).
4. **Ordering by `SimId`.** Never by `Entity`; entity indices are ephemeral and unordered (ADR-0008).
5. **Presentation is an observer.** It writes exactly one thing back: the intent (ADR-0002). The simulation never reads presentation state.
6. **One code path per quantity.** Damage, health, death — each has exactly one place where it happens.
7. **No gameplay state in `Local<>`**, no meshes, materials, lights or UI on simulation entities.
8. **Teams, not "the opponent".** No code assumes exactly two players.

## The tick chain

```text
Intent  →  Values  →  Movement  →  Damage  →  Visibility  →  Cleanup
```

| Set | What happens |
| --- | --- |
| `Intent` | The queued intents are frozen and applied for this tick. |
| `Values` | Cooldowns, regeneration, attribute modifiers. |
| `Movement` | Units and projectiles move by their tick velocity. |
| `Damage` | Hits are collected, sorted by `SimId`, then effects, damage and death. |
| `Visibility` | Who sees what this tick — a sim of its own. |
| `Cleanup` | Spent entities are despawned, respawns happen. |

Extend the chain as the game grows; the order is the contract that keeps runs reproducible.
