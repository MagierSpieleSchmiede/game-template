---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**T-011, rename tool (branch `dev-rename-script`, on top of `dev-run-lifecycle`):**

- New workspace member `tools/rename_crates` (no dependencies) with the cargo alias in `.cargo/config.toml`: `cargo rename-crates <sim> <client>` checks the names, refuses uncommitted changes, replaces both crate names as whole identifiers in every tracked text file except its own, moves the crate folders, then runs `cargo update --workspace` (offline first) and `cargo fmt --all`.
- Unit tests in the tool; verified by renaming this repository to `dragon_sim`/`dragon_client`: all four gates green with `--locked`, then reverted.
- README, [[project-structure|Project structure]] and [[t-001-adapt-the-template|T-001]] point at the tool.

## Next task

**Adapt the template to your game** ([[t-001-adapt-the-template|T-001]]), or the next `template` task in the [[backlog|Backlog]] (T-012 onward).

- **Read for that:** the task's page; for T-001: [[decisions|Decisions]] and the default ADRs, [[spec-001-prototype|Spec 001 template]], [[conventions|Conventions]].
- **Tests first:** keep the determinism test green after every change to the simulation.

## Hints for the next session

- The toolchain is pinned at 1.97.1 (`rust-version`); if the default `stable` is older, run cargo as `cargo +1.97.1` (also `cargo +1.97.1 rename-crates …`).
- Re-run the benchmark before and after simulation changes, on the same machine; afterwards `cargo clean --release`.
- Save/load and the run lifecycle were tested headless only; nobody has played them in a window yet: worth a check on desktop (F5, F9, Esc → Q → Enter).
- A save does not record the local player; loading keeps the `RunSetup`'s. A multiplayer game must store or negotiate the seat.
- A full debug build with tests is ~27 GB of `target/`; a trial rename builds everything again under the new names and fills the session disk: `cargo clean` between the two.
- The rename tool rewrites prose too, including history in the vault; it does not touch the repository name `game-template` (site config, deploy workflow, README badge): that stays manual in T-001. The save folder follows the client crate name (`storage::APP_FOLDER`).
- Replacing the example game means rewriting `game_sim/src/game/` and `game_client/src/presentation.rs` (keep `Frames`, `observe_restored` and a way to find the local actor in the observation).
