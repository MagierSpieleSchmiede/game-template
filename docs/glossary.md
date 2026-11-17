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
| **Scenario** | The complete start state of a run: the seed and the game's `Setup`. Same scenario + same inputs = same end state. |
| **Snapshot** | The whole simulation state as one value (tick, RNG, ID counter, control, game state); restoring it and re-running the same intents gives the same state. |
| **Player** | Whoever sends intents: a person, an AI or an add-on, numbered by `PlayerId`. |
| **Control** | Which player may steer which actor; intents from anyone else are rejected. |
| **State hash** | A platform-independent checksum of a snapshot; equal states hash equally. Compared by determinism tests, replays and desync checks. |
| **Replay** | A scenario plus the intents that arrived in every tick, and the final state hash playback must reach. Stored as a file (ADR-0017). |
| **Save game** | A scenario, a snapshot and its state hash: continues a run exactly. |
| **SIM_VERSION** | The version of the simulation's behaviour, recorded in every file. Bumped whenever a change alters what a run produces; files from other versions are refused. |
| **Template default** | An ADR with status `default`: a genre-dependent decision the template ships working code for, decided per game (ADR-0012). |
| **Rollback** | Netcode technique: a client predicts ahead and rewinds to a confirmed state when a late input arrives. |
| **Referee** | The server that decides in conflicts; each client only computes what it is allowed to see. |
| **ADR** | Architecture Decision Record: one decision, its context, the rejected alternative, its consequences. |
| **P** | An open problem page: a question, who is affected, and its next step. |
| **Vault** | This documentation: an Obsidian-style folder of linked Markdown pages, published as a website. |
| **Balance** | The central resource holding all gameplay constants; no magic numbers in systems. |
