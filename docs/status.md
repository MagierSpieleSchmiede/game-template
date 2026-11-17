---
title: "Status — handover: what was last done and what comes next"
---
# Status — handover: what was last done and what comes next

> [!important] What this page is for
> It is the **handover from one session to the next**, so nobody has to read the whole vault. Whoever starts a task reads **only this page** and the pages listed under *Read for that*. Whoever finishes a task **overwrites** the three sections below (do not append; history belongs in the milestone's implementation log). Keep it short: about 30 lines at most.

## Last done

**Branches tidied up:** the template work T-009 to T-015 (stranded on `dev-rename-script` after PR #7 went into an already merged branch) is now in `main` via PR #8. AGENTS.md now says **one branch per person** (`dev-<person>`), which agents reuse; the per-task `dev-*` branches are deleted. The maintainer's branch is `dev-daniel`.

T-015 is still in progress: web checked in headless Chromium, Android and iOS not yet compiled or run (see the task page).

## Next task

**Finish T-015 on a device** (a session with a phone connected, once a game packages for Android/iOS), or **adapt the template to your game** ([[t-001-adapt-the-template|T-001]]).

- **Read for that:** [[t-015-storage-backends-for-mobile-and-web|T-015]] (what is still open); for T-001: [[decisions|Decisions]] and the default ADRs, [[spec-001-prototype|Spec 001 template]], [[conventions|Conventions]].
- **Tests first:** keep the determinism test green after every change to the simulation.

## Hints for the next session

- Running the web client: `cargo build -p game_client --target wasm32-unknown-unknown`, then `wasm-bindgen --target web` (CLI 0.2.130, matching the lock) and a page that imports the generated JS; headless Chromium renders it with `--use-angle=swiftshader`. The debug wasm is ~170 MB after bindgen. `cargo clean --target …` wiped all of `target/`, not just that target.
- The toolchain is pinned at 1.97.1 (`rust-version`); if the default `stable` is older, run cargo as `cargo +1.97.1` (also `cargo +1.97.1 rename-crates …`).
- Re-run the benchmark before and after simulation changes, on the same machine; afterwards `cargo clean --release`.
- Save/load and the run lifecycle were tested headless only; nobody has played them in a window yet: worth a check on desktop (F5, F9, Esc → Q → Enter).
- A save does not record the local player; loading keeps the `RunSetup`'s. A multiplayer game must store or negotiate the seat.
- A full debug build with tests is ~27 GB of `target/`; a trial rename builds everything again under the new names and fills the session disk: `cargo clean` between the two.
- The rename tool rewrites prose too, including history in the vault; it does not touch the repository name `game-template` (site config, deploy workflow, README badge): that stays manual in T-001. The save folder follows the client crate name (`storage::APP_FOLDER`).
- `target/` grows past the 20 GB limit fast: this session hit a full disk at ~34 GB of stale artefacts; after the gates it was 27 GB and was cleaned.
- Seen in a window (Xvfb, software Vulkan): the menu renders in Noto Sans with `·`. Pause screen (`—`) and overlay (F3) not yet looked at: synthetic keys did not reach the window. Bevy's own Fira Mono subset still ships (the `default_font` feature); dropping it means listing Bevy's features by hand.
- A locale in a script beyond Latin/Greek/Cyrillic fails the coverage test: add a second font and use a font list (`FontSource::List`), see ADR-0016.
- Replacing the example game means rewriting `game_sim/src/game/` and `game_client/src/presentation.rs` (keep `Frames`, `observe_restored` and a way to find the local actor in the observation).
