---
title: "Glossary — terms from intent to rollback"
---
# Glossary — terms from intent to rollback

| Term | Meaning |
| --- | --- |
| **Tick** | One step of the simulation, `1 / TICKS_PER_SECOND` seconds. All durations in the simulation are ticks. |
| **Intent** | Plain, serialisable data saying what a unit wants to do in a tick. The only thing the presentation writes back. |
| **Simulation** | The deterministic gameplay in `game_sim`, running in `FixedUpdate`, headless-capable. |
| **Presentation** | Everything the player sees: rendering, HUD, animation, sound. A pure observer. |
| **Observation** | The one function through which HUD, AI, add-ons and the network read simulation state, filtered by visibility. |
| **SimId** | A stable ID for simulation entities; ordering and serialisation key on it, never on `Entity`. |
| **SimRng** | The one seeded RNG resource; the only randomness source in the simulation. |
| **Scenario** | The complete start state of a run: seed, units, arena. Same scenario + same inputs = same end state. |
| **Rollback** | Netcode technique: a client predicts ahead and rewinds to a confirmed state when a late input arrives. |
| **Referee** | The server that decides in conflicts; each client only computes what it is allowed to see. |
| **ADR** | Architecture Decision Record: one decision, its context, the rejected alternative, its consequences. |
| **P** | An open problem page: a question, who is affected, and its next step. |
| **Vault** | This documentation: an Obsidian-style folder of linked Markdown pages, published as a website. |
| **Balance** | The central resource holding all gameplay constants; no magic numbers in systems. |
