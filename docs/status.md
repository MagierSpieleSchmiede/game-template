---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**Template created (game-template):**

- Workspace with `crates/game_sim` (deterministic, headless tick simulation) and `crates/game_client` (presentation skeleton).
- `SimRng` (Xoshiro256\*\* with SplitMix64, seeded from the scenario), `SimId`, `Intent`/`IntentBuffer`, `Balance`, `Scenario`, the tick chain with `SimSet`s, and a headless app driver.
- Tests: determinism (same scenario → same end state), seed divergence, headless run without window; `clippy.toml` forbids other randomness, clocks and hash maps in the simulation.
- Vault set up in English; ADRs 0001–0010 accepted; CI and vault deploy workflows in place.

## Next task

**Adapt the template to your game (first session of the new project):**

- **Clarify first:** the game's concept, scope and platforms; which template ADRs do not apply.
- **Read for that:** the [[spec-001-prototype|Spec 001 template]]; [[conventions|Conventions]]; [[decisions|Decisions]] for the accepted ADRs.
- **Tests first:** keep the determinism test green after every change to the simulation.

## Hints for the next session

- Rename `game_sim`/`game_client` everywhere (Cargo.toml, workflows, docs) to the game's name.
- The simulation must keep building for `wasm32-unknown-unknown`; check `docs/architecture/platforms.md` before adding dependencies.
- Everything gameplay stays in `FixedUpdate`, ordered by `SimSet`; the presentation only writes intents.
