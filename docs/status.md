---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**T-008, client save and load (branch `dev-client-save-load`):**

- `game_client/src/storage.rs`: `SlotStore` (named byte slots), `FileStore` (user data folder on Windows/Linux/macOS, found from the environment, atomic writes, slot names cannot leave the folder), `MemoryStore` (Android, iOS, web for now).
- `game_client/src/saving.rs`: actions `save` (F5/North, while playing or paused) and `load` (F9/West, from the menu too); a localised notice for every outcome, one per refusal reason; after loading the presentation re-observes without interpolating.
- New task T-015: real storage backends for Android, iOS and the web.

## Next task

**Adapt the template to your game** ([[t-001-adapt-the-template|T-001]]), or the next `template` task in the [[backlog|Backlog]] (T-009 onward).

- **Read for that:** the task's page; for T-001: [[decisions|Decisions]] and the default ADRs, [[spec-001-prototype|Spec 001 template]], [[conventions|Conventions]].
- **Tests first:** keep the determinism test green after every change to the simulation.

## Hints for the next session

- The toolchain is pinned at 1.97.1 (`rust-version`); if the default `stable` is older, run cargo as `cargo +1.97.1`.
- Save/load was tested headless only; nobody has played it in a window yet: worth a check on desktop (F5, F9, a damaged `quicksave.save`).
- Rename `game_sim`/`game_client` everywhere for a new game; the save folder follows the client crate name (`storage::APP_FOLDER`).
- Replacing the example game means rewriting `game_sim/src/game/` and `game_client/src/presentation.rs` (keep `Frames` and `observe_restored`, the saving module uses them).
