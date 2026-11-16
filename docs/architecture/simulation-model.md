---
title: "Simulation model — the invariants and the tick chain"
---
# Simulation model — the invariants and the tick chain

The entire gameplay is a deterministic tick simulation in `FixedUpdate` (ADR-0001). One source per fact: the invariants live here, the reasoning in the ADRs.

## Invariants

1. **One tick, one chain.** Every gameplay system runs in `FixedUpdate`, in `SimSet::Update`, in exactly one of the game's own ordered sets. Nothing gameplay runs anywhere else.
2. **No frame time.** The simulation never reads wall-clock or frame deltas; all durations are ticks (ADR-0003).
3. **One randomness source.** All randomness draws from `SimRng`; `clippy.toml` forbids everything else (ADR-0004).
4. **Ordering by `SimId`.** Never by `Entity`; entity indices are ephemeral and unordered (ADR-0008).
5. **Presentation is an observer.** It writes exactly one thing back: the intent (ADR-0002). The simulation never reads presentation state.
6. **One code path per quantity.** Damage, health, death — each has exactly one place where it happens.
7. **No gameplay state in `Local<>`**, no meshes, materials, lights or UI on simulation entities.
8. **Teams, not "the opponent".** No code assumes exactly two players.

## The tick chain

The core owns three sets (`game_sim::schedule::SimSet`), in this order every tick:

| Set | What happens |
| --- | --- |
| `Input` | The tick is counted (`SimTick`), the queued intents are frozen into `TickIntents` in actor order, and recorded if a replay is recording. |
| `Update` | The game's systems, in the game's own ordered sets. |
| `End` | End-of-tick work of the core (empty so far: checks, statistics). |

Inside `Update`, the example game (ADR-0013, a template default) runs:

```text
Intent  →  Values  →  Movement  →  Damage  →  Cleanup
```

| Set | What happens |
| --- | --- |
| `Intent` | This tick's intents set the units' velocities. |
| `Values` | Cooldowns, regeneration, attribute modifiers (empty in the example). |
| `Movement` | Units move by their velocity and are clamped to the arena. |
| `Damage` | Hits, effects, damage and death, sorted by `SimId` (empty in the example). |
| `Cleanup` | Spent entities are despawned, respawns happen (empty in the example). |

A new game replaces these stages with its own; the order is the contract that keeps runs reproducible.
