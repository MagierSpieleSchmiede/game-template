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

Inside `game_sim`:

| Module | Responsibility |
| --- | --- |
| `schedule` | The tick chain and its `SimSet`s — the only place gameplay systems run. |
| `intent` | `Intent`, `Action`, the `IntentBuffer`: the input of the tick chain. |
| `rng` | `SimRng`: the one seeded randomness source (ADR-0004). |
| `ids` | `SimId`: stable IDs for ordering and serialisation (ADR-0008). |
| `balance` | The central balance resource (ADR-0010). |
| `scenario` | The start state of a run, as data. |
| `unit` | The simulated units and their gameplay state. |
| `headless` | The app driver for tests, benchmarks, server and RL. |

Rules: `game_sim` has no dependency on rendering, windowing, audio or threads; `game_client` never simulates. New modules follow the same one-responsibility split.
