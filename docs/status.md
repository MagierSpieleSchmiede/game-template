---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**Review fixes (branch `dev-review-fixes`):**

- `SimId` is now its own component, spawned next to `Unit` (before, `apply_intents` found no unit and every intent was dropped); IDs come from the `SimIdCounter` resource.
- Arena clamp works on the x/z floor (it used x/y); `SimPlugin` sets the fixed timestep to 60 Hz (Bevy's default is 64); `spawn_scenario` seeds `SimRng` from the scenario, so the client honours the seed; spawn directions without `sin`/`cos`; the `SimSet`s are chained.
- Tests: movement, arena, tick rate, spawn counter; the determinism test now orders by `SimId`, checks that units moved, and compares `state_hash` with a committed value on every CI platform.
- ADRs 0001–0011 accepted.

## Next task

**Adapt the template to your game (first session of the new project):**

- **Clarify first:** the game's concept, scope and platforms; which template ADRs do not apply.
- **Read for that:** the [[spec-001-prototype|Spec 001 template]]; [[conventions|Conventions]]; [[decisions|Decisions]] for the accepted ADRs.
- **Tests first:** keep the determinism test green after every change to the simulation.

## Hints for the next session

- Rename `game_sim`/`game_client` everywhere (Cargo.toml, workflows, docs) to the game's name.
- The simulation must keep building for `wasm32-unknown-unknown`; check `docs/architecture/platforms.md` before adding dependencies.
- Everything gameplay stays in `FixedUpdate`, ordered by `SimSet`; the presentation only writes intents.
