---
title: "T-009 — Headless benchmark: ticks per second"
---
# T-009 — Headless benchmark: ticks per second

**Status:** done · **Scope:** template · **Depends on:** —

**Read for that:** [[benchmarks|Benchmarks]]

**What:** Measure how many ticks per second the headless simulation runs for a scenario of a given size, and how long a snapshot capture, restore and state hash take. Rollback re-simulates several ticks per frame, so these numbers bound the rollback window.

**Done when:** A benchmark command documented in [[benchmarks|Benchmarks]], with first results recorded there.

**Result:** `crates/game_sim/examples/bench.rs` (`cargo run -p game_sim --release --example bench`); command, columns and first results in [[benchmarks|Benchmarks]].
