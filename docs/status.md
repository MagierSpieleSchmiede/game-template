---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**Backlog as a folder (branch `dev-backlog-folder`):**

- `docs/backlog/` holds one page per task (template: [[t-nnn|Task template]]) and the index table in `backlog.md`; tasks have a scope: `game` (done once per new game) or `template` (improves the template).
- T-001–T-007 moved over; new template tasks T-008–T-014 from the last review: client save/load, headless benchmark, client run lifecycle, rename script, non-ASCII font, CI web build of the client, overlay hash only when visible.

## Next task

**Adapt the template to your game (first session of the new project):**

- **Clarify first:** the game's concept, scope and platforms; then decide each template default (`default` rows in Decisions).
- **Read for that:** [[decisions|Decisions]] and the default ADRs; [[spec-001-prototype|Spec 001 template]]; [[conventions|Conventions]].
- **Tests first:** keep the determinism test green after every change to the simulation.

## Hints for the next session

- Rename `game_sim`/`game_client` everywhere (Cargo.toml, workflows, docs) to the game's name.
- Replacing the example game means rewriting `game_sim/src/game/` and `game_client/src/presentation.rs`; the contract is in `game/mod.rs`. The core must not name genre concepts.
- The simulation must keep building for `wasm32-unknown-unknown`; check `docs/architecture/platforms.md` before adding dependencies.
- Open template work is in the [[backlog|Backlog]] with scope `template` (T-008–T-014); e.g. the client cannot save or load yet (T-008), and the built-in font covers ASCII only (T-012).
