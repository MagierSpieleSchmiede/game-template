---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**T-010, client run lifecycle (branch `dev-run-lifecycle`, on top of `dev-headless-benchmark`):**

- `scenario::start_run` now clears an earlier run first (entities, intents, tick, IDs, control); starting again equals a fresh app (test in `tests/simulation.rs`).
- Client module `run.rs`: `RunSetup` (scenario of the next run, local player, viewer). *Confirm* in the menu starts every run from it; *quit-to-menu* (Q / pad East) leads from the pause screen back to the menu.
- No more `LOCAL_*` constants: the game observation reports each unit's `controller`, the presentation steers the unit its player controls (`presentation::local_actor`).
- Client test `run::tests` plays menu → run → menu → run headless. ADR-0016 and [[project-structure|Project structure]] updated.

## Next task

**Adapt the template to your game** ([[t-001-adapt-the-template|T-001]]), or the next `template` task in the [[backlog|Backlog]] (T-011 onward).

- **Read for that:** the task's page; for T-001: [[decisions|Decisions]] and the default ADRs, [[spec-001-prototype|Spec 001 template]], [[conventions|Conventions]].
- **Tests first:** keep the determinism test green after every change to the simulation.

## Hints for the next session

- The toolchain is pinned at 1.97.1 (`rust-version`); if the default `stable` is older, run cargo as `cargo +1.97.1`.
- Re-run the benchmark before and after simulation changes, on the same machine; afterwards `cargo clean --release`.
- Save/load and the run lifecycle were tested headless only; nobody has played them in a window yet: worth a check on desktop (F5, F9, Esc → Q → Enter).
- A save does not record the local player; loading keeps the `RunSetup`'s. A multiplayer game must store or negotiate the seat.
- Rename `game_sim`/`game_client` everywhere for a new game; the save folder follows the client crate name (`storage::APP_FOLDER`).
- Replacing the example game means rewriting `game_sim/src/game/` and `game_client/src/presentation.rs` (keep `Frames`, `observe_restored` and a way to find the local actor in the observation).
