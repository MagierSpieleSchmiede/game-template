# Game Template

**A template for new games by Magier SpieleSchmiede: Rust + Bevy, a deterministic, headless tick simulation, and a documented design vault.**

[![Docs](https://img.shields.io/badge/docs-vault-7bd88f)](https://magierspieleschmiede.github.io/game-template/)
[![License: MIT](https://img.shields.io/badge/license-MIT-lightgrey)](LICENSE)

Use this repository as a template for a new game (GitHub: *Use this template*).
It ships the decisions and structure that every game of ours builds on:

- **Rust** with [Bevy](https://bevy.org) (and [Avian](https://github.com/avianphysics/avian) for physics, as a template default).
- **Genre-neutral core, replaceable example:** the simulation core (ticks, intents, snapshots, state hashes, replays, rollback, observation) knows no genre; a small example game (units on an arena) and a playable client shell (menu, pause, keyboard/gamepad actions, localisation, debug overlay) show it working.
- **Template defaults:** genre-dependent decisions (netcode model, tick rate, physics, the example game, the client shell) are ADRs with status `default` — working code that each game accepts, replaces or drops.
- **A deterministic, headless tick simulation** in its own crate (`game_sim`):
  same start state, same inputs, same seed → same end state, byte for byte,
  on the same build, and across the CI platforms as long as the simulation keeps
  the floating-point discipline ([Determinism](docs/architecture/determinism.md)).
- **One seeded RNG resource** for all randomness in the simulation.
- **Snapshots, save and replay files, rollback**, tested: restoring a snapshot
  and re-running, loading a save, playing back a recording, and rolling back a
  late intent all reach the same state hash. A committed regression replay is
  played back by CI on every platform.
- **A documented vault** (`docs/`, published as a website with Quartz):
  specification, architecture, decisions (ADRs), open problems, and a
  handover page ([Status](docs/status.md)) that every session reads first.
- **AGENTS.md** with the workflow rules for humans and AI agents.

## What to do after forking

1. Rename the two crates in one step, e.g. `cargo rename-crates dragon_sim dragon_client`
   (commit first: the tool refuses to run on uncommitted changes), then replace the
   repository name `game-template` with your game's.
2. Rewrite `docs/index.md`, `docs/specifications/spec-001-prototype.md` and
   `docs/status.md` for your game; keep the structure and conventions.
3. Adjust `site/quartz.config.ts` (title, `baseUrl`) and the deploy workflow to
   your repository name.
4. Decide every template default (status `default` in
   `docs/decisions/decisions.md`): accept, replace or drop it, following the
   ADR's *To replace* section. Start with `crates/game_sim/src/game/`, the
   example game, whose `mod.rs` documents what the core needs from a game.

## Repository layout

| Path | What it is |
| --- | --- |
| `AGENTS.md` | Start here: workflow rules for humans and AI agents |
| `crates/game_sim` | The deterministic, headless tick simulation (no rendering) |
| `crates/game_client` | The client: presentation, input, HUD, localisation |
| `tools/rename_crates` | `cargo rename-crates`: renames both crates for a new game |
| `docs/` | The vault: specification, architecture, decisions, problems, status |
| `site/` | Quartz build of the vault, published to GitHub Pages |
| `.github/workflows/` | CI (fmt, clippy, test, cross-platform checks) and vault deploy |

## Building

```sh
cargo run --package game_client   # Enter/Start to play, WASD/stick to move, Esc/Start to pause, F3/Select for the overlay
cargo test
```

On Linux the client needs `libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev`.

The simulation runs headless and builds for `wasm32-unknown-unknown`, so it can
be tested without a window and embedded anywhere.

## License

MIT — see [LICENSE](LICENSE).

The bundled UI font, Noto Sans (`crates/game_client/data/fonts/`), is under the
SIL Open Font License 1.1 — see [OFL.txt](crates/game_client/data/fonts/OFL.txt).
