---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**Save and replay files (branch `dev-save-files`):**

- ADR-0017 (template default): postcard behind a header with magic, format version, file kind and `game::SIM_VERSION`. `game_sim::file` refuses foreign, outdated, wrong-kind, damaged and padded files; `SaveGame` also checks its state hash. The simulation only produces bytes.
- Replays record intents as they arrived, so playback re-runs validation and control.
- A committed regression replay (`crates/game_sim/tests/data/regression.replay`) is played back by CI on every platform. On a deliberate behaviour change: bump `SIM_VERSION`, then `cargo test -p game_sim --test regression -- --ignored record_regression_replay`.
- Fixed: the state hash covered only the RNG's next output (one of four state words); it now covers all four. The determinism hash changed; unit states are unchanged.

## Next task

**Adapt the template to your game (first session of the new project):**

- **Clarify first:** the game's concept, scope and platforms; then decide each template default (`default` rows in Decisions).
- **Read for that:** [[decisions|Decisions]] and the default ADRs; [[spec-001-prototype|Spec 001 template]]; [[conventions|Conventions]].
- **Tests first:** keep the determinism test green after every change to the simulation.

## Hints for the next session

- Rename `game_sim`/`game_client` everywhere (Cargo.toml, workflows, docs) to the game's name.
- Replacing the example game means rewriting `game_sim/src/game/` and `game_client/src/presentation.rs`; the contract is in `game/mod.rs`. The core must not name genre concepts.
- The simulation must keep building for `wasm32-unknown-unknown`; check `docs/architecture/platforms.md` before adding dependencies.
- Bevy's built-in font covers ASCII only; add a font asset before non-ASCII texts.
- The client does not save or load yet: it needs a storage place per platform (file, app data folder, browser storage).
