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

## The headless benchmark

```sh
cargo run -p game_sim --release --example bench
cargo run -p game_sim --release --example bench -- --units 2,100,1000 --ticks 600 --runs 7 --window 8
```

`crates/game_sim/examples/bench.rs` prints a Markdown table, one row per scenario size. Every unit gets a move intent every tick. Each figure is the median over the runs, with min–max beside it. Afterwards run `cargo clean --release` (AGENTS.md).

| Column | What is measured |
| --- | --- |
| Tick | One `App::update` of the headless app with intents, averaged over `--ticks` ticks |
| Capture, Restore, Hash | `Snapshot::capture`, `Snapshot::restore`, `Snapshot::hash` after one simulated second |
| Rollback of *n* | `RollbackSession::insert_late` for the oldest tick of an *n*-tick window: one restore plus *n* captures and ticks |
| Ticks per 60 Hz frame | (16.7 ms − restore) / (tick + capture): the upper bound of the rollback window when the frame does nothing else |

The workload (`intents_for`) belongs to the example game; a new game replaces it with a load typical for it, and the `cargo test` self-check keeps the benchmark compiling and running.

## Results

2026-11-17, template example game, `SIM_VERSION` 1. Cloud container, 4 vCPUs (Intel Xeon @ 2.10 GHz), shared, so the spread is wide. Release build, 600 ticks, 7 runs, window 8; µs, median (min–max):

| Units | Tick | Ticks/s | Capture | Restore | Hash | Rollback of 8 | Ticks per 60 Hz frame |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 2 | 8.46 (8.05–14.4) | 118231 | 0.53 (0.51–0.56) | 1.12 (0.98–3.04) | 0.13 (0.13–0.20) | 88.5 (78.7–132) | 1854 |
| 100 | 11.4 (11.1–13.4) | 87527 | 3.01 (1.89–3.43) | 15.7 (9.31–17.4) | 6.79 (6.67–8.44) | 182 (133–251) | 1153 |
| 1000 | 118 (106–164) | 8497 | 17.7 (16.5–23.3) | 105 (99.4–134) | 65.9 (61.4–67.6) | 1114 (1082–1387) | 122 |

Reading them:

- About 8 µs per tick is fixed cost of Bevy's schedules with almost nothing to do; above that, cost grows roughly linearly with units (about 0.11 µs per unit and tick).
- Restore costs about six captures, since it despawns and respawns every simulation entity.
- The state hash costs about 66 ns per unit, almost four times a capture. The client computes it once per tick only while the debug overlay is visible ([[t-014-overlay-hash-only-when-visible|T-014]]); desync checks would run it per tick, so it is the first thing to look at if large scenarios matter.
- Even at 1000 units, an 8-tick rollback takes about 1.1 ms of a 16.7 ms frame: the example game leaves rollback plenty of headroom. A real game re-measures with its own systems.
