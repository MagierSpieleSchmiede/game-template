---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**T-009, headless benchmark (branch `dev-headless-benchmark`, on top of `dev-client-save-load`):**

- `crates/game_sim/examples/bench.rs`: ticks per second, snapshot capture/restore, state hash and one rollback per scenario size; median and spread; `cargo test` runs a small self-check.
- Command, columns and first results in [[benchmarks|Benchmarks]]. At 1000 units: 118 µs per tick, 8-tick rollback 1.1 ms; the state hash costs about four captures.
- Before that, T-008 (client save/load) went up for review as MagierSpieleSchmiede/game-template#6.

## Next task

**Adapt the template to your game** ([[t-001-adapt-the-template|T-001]]), or the next `template` task in the [[backlog|Backlog]] (T-010 onward).

- **Read for that:** the task's page; for T-001: [[decisions|Decisions]] and the default ADRs, [[spec-001-prototype|Spec 001 template]], [[conventions|Conventions]].
- **Tests first:** keep the determinism test green after every change to the simulation.

## Hints for the next session

- The toolchain is pinned at 1.97.1 (`rust-version`); if the default `stable` is older, run cargo as `cargo +1.97.1`.
- Re-run the benchmark before and after simulation changes, on the same machine; afterwards `cargo clean --release`.
- Save/load was tested headless only; nobody has played it in a window yet: worth a check on desktop (F5, F9, a damaged `quicksave.save`).
- Rename `game_sim`/`game_client` everywhere for a new game; the save folder follows the client crate name (`storage::APP_FOLDER`).
- Replacing the example game means rewriting `game_sim/src/game/` and `game_client/src/presentation.rs` (keep `Frames` and `observe_restored`, the saving module uses them).
