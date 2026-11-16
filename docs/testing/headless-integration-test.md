---
title: "Headless integration test — the simulation without a window"
---
# Headless integration test — the simulation without a window

The normal case for tests, benchmarks, the server and RL training is the **headless app** (`game_sim::headless`):

```rust
let mut app = headless_app(Scenario { seed: 42, setup: Setup { unit_count: 2 } });
step_ticks(&mut app, 600); // ten simulated seconds
```

What it guarantees:

- **No window, no renderer, no threads:** time never comes from a clock; every `App::update` advances exactly one fixed timestep, so `FixedUpdate` runs exactly once.
- **Startup runs once**, before the app is handed out, so the first `step_ticks` is already a real tick.
- A test that needs no randomness beyond the scenario needs no setup at all.

Rules: integration tests live in `crates/game_sim/tests/`; they script intents, step ticks and assert on simulation state (the game's components, the observation, or the state hash); they never read presentation state.
