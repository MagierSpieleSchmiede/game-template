---
title: "Project structure — the crate layout and why each part exists"
---
# Project structure — the crate layout and why each part exists

```text
crates/
  game_sim/     the deterministic, headless tick simulation
  game_client/  presentation: window, rendering, input, HUD, localisation
docs/           the vault
site/           Quartz build of the vault
```

| Crate | Exists because |
| --- | --- |
| `game_sim` | Gameplay must run without window, renderer or threads: in tests, on the server, in RL training, as wasm. It builds for `wasm32-unknown-unknown` (ADR-0005, ADR-0009). |
| `game_client` | The presentation is a replaceable observer; it depends on `game_sim`, never the other way around. |

Inside `game_sim`, the **core** (genre-neutral, kept by every game):

| Module | Responsibility |
| --- | --- |
| `schedule` | The core `SimSet`s: `Input → Update → End`. |
| `tick` | `SimTick`: the number of the last simulated tick. |
| `intent` | `Intent`, the `IntentBuffer`, the checked and frozen `TickIntents`, the `RejectedIntents`: the input of a tick. |
| `control` | `PlayerId` and `Control`: which player may steer which actor. |
| `rng` | `SimRng`: the one seeded randomness source (ADR-0004). |
| `ids` | `SimId` and the `SimIdCounter`: stable IDs (ADR-0008). |
| `scenario` | The start state of a run, as data; `start_run` seeds the RNG and lets the game spawn. |
| `snapshot` | The whole simulation state as a value, and back: save games, rollback, debugging. |
| `checksum` | The state hash: determinism tests, replay checks, desync detection. |
| `replay` | Record a run's intents; play them back and compare the hash. |
| `rollback` | Re-simulate when an intent arrives late (template default, ADR-0006). |
| `observation` | The one read path for presentation, AI and network (ADR-0007). |
| `headless` | The app driver for tests, benchmarks, server and RL. |

And the **game** (`game/`), a template default (ADR-0013) that every game replaces. Its `mod.rs` documents the contract the core relies on: `Action`, `validate`, `Setup`, `build`, `spawn`, `GameState` with `capture`/`restore`/`hash_into`, `GameObservation` with `observe`. The example's units, balance (ADR-0010) and systems live there.

Inside `game_client`:

| Module | Responsibility | Genre-neutral? |
| --- | --- | --- |
| `screens` | Menu → playing ⇄ paused (ADR-0016) | yes |
| `input` | Devices → actions; bindings in `data/input/` (ADR-0016) | yes |
| `localisation`, `kv` | Texts by key from `data/locales/` (ADR-0016) | yes |
| `overlay` | Tick, state hash, frame rate (ADR-0016) | yes |
| `presentation` | The example game's camera, visuals and intent (ADR-0013) | no: replace with the game |

Rules: `game_sim` has no dependency on rendering, windowing, audio or threads; `game_client` never simulates. New modules follow the same one-responsibility split.
