---
title: "T-010 — Client: back to the menu, fresh runs, local player from setup"
---
# T-010 — Client: back to the menu, fresh runs, local player from setup

**Status:** done · **Scope:** template · **Depends on:** —

**Read for that:** ADR-0016, [[project-structure|Project structure]]

**What:** The client can go from the menu into a run only once: there is no way back to the menu, and a second run would not reset the simulation. `LOCAL_PLAYER` and `LOCAL_ACTOR` are constants. Add *quit to menu*, start every run from its scenario, and take the local player and actor from the run's setup.

**Done when:** Menu → run → menu → run works and the second run starts from its scenario; no constant names the local player.
