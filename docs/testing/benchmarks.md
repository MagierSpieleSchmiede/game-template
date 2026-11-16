---
title: "Benchmarks — performance before and after"
---
# Benchmarks — performance before and after

Whoever changes the simulation or rendering compares **before and after**:

- **Simulation:** ticks per second of the headless app with a representative scenario (units, abilities, projectiles). Rollback needs headroom — the sim must re-run several ticks per frame.
- **Rendering:** frame time of the client on the minimum hardware (see the problem log for the budget question).

Rules:

- Measure on the same machine, same power mode, nothing else running; report median and spread, not a single run.
- A regression that exceeds the budget blocks the change until explained or fixed.
- Benchmarks are scripts in the repository, so "before" is reproducible from the previous commit.
