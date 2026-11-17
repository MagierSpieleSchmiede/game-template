---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**Hardening (branch `dev-hardening`):**

- Intents are checked at the start of every tick: the game's `validate` refuses malformed actions (NaN, infinity), and the new `Control` resource (who controls which actor, part of snapshots and the hash) refuses intents from other players. Rejections are listed in `RejectedIntents`. `Intent` now carries `player`.
- CI: a job checks the workspace with the declared `rust-version` (1.97.1); `main` is built after every merge; Quartz is pinned to a commit in `site/build.sh`.
- `clippy.toml` also forbids Bevy's and hashbrown's hash maps and sets, `rand` and `getrandom`.
- The determinism hash changed because it now covers control; the units' states are unchanged.

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
