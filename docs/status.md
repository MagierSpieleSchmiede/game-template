---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**Template defaults and a genre-neutral core (branch `dev-template-defaults`):**

- New ADR status `default` (ADR-0012): genre-dependent decisions ship working code and are decided per game. Defaults: 0006 rollback, 0013 example game, 0014 60 Hz + f32, 0015 Avian, 0016 client shell.
- `game_sim` split into a core (tick, intents frozen in actor order, scenario, snapshot, state hash, replay, rollback, observation) and `game/`, the example arena game behind a documented contract.
- Client: menu → playing ⇄ paused, keyboard/gamepad actions with bindings in `data/input/`, localisation in `data/locales/`, units drawn from the observation and interpolated between ticks, debug overlay (F3). Run and checked on a software renderer.
- Tests for snapshots, replays, rollback, observation, bindings, localisation, interpolation. The determinism hash changed because it now includes the tick and ID counter; the unit states are unchanged.

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
