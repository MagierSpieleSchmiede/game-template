# Game Template

**A template for new games by Magier SpieleSchmiede: Rust + Bevy, a deterministic, headless tick simulation, and a documented design vault.**

[![Docs](https://img.shields.io/badge/docs-vault-7bd88f)](https://magierspieleschmiede.github.io/game-template/)
[![License: MIT](https://img.shields.io/badge/license-MIT-lightgrey)](LICENSE)

Use this repository as a template for a new game (GitHub: *Use this template*).
It ships the decisions and structure that every game of ours builds on:

- **Rust** with [Bevy](https://bevy.org) and [Avian](https://github.com/avianphysics/avian) for physics queries.
- **A deterministic, headless tick simulation** in its own crate (`game_sim`):
  same start state, same inputs, same seed → same end state, byte for byte.
- **One seeded RNG resource** for all randomness in the simulation.
- **Rollback-ready design**: serializable intents, ticks instead of seconds,
  stable IDs instead of entity indices — the foundation for rollback netcode
  with a server as referee.
- **A documented vault** (`docs/`, published as a website with Quartz):
  specification, architecture, decisions (ADRs), open problems, and a
  handover page ([Status](docs/status.md)) that every session reads first.
- **AGENTS.md** with the workflow rules for humans and AI agents.

## What to do after forking

1. Replace `game-template` / `game_sim` / `game_client` names with your game's name.
2. Rewrite `docs/index.md`, `docs/specifications/spec-001-prototype.md` and
   `docs/status.md` for your game; keep the structure and conventions.
3. Adjust `site/quartz.config.ts` (title, `baseUrl`) and the deploy workflow to
   your repository name.
4. Delete the ADRs that do not apply to your game — but think twice before you
   do: they are the solid decisions (see `docs/decisions/decisions.md`).

## Repository layout

| Path | What it is |
| --- | --- |
| `AGENTS.md` | Start here: workflow rules for humans and AI agents |
| `crates/game_sim` | The deterministic, headless tick simulation (no rendering) |
| `crates/game_client` | The client: presentation, input, HUD, localisation |
| `docs/` | The vault: specification, architecture, decisions, problems, status |
| `site/` | Quartz build of the vault, published to GitHub Pages |
| `.github/workflows/` | CI (fmt, clippy, test, cross-platform checks) and vault deploy |

## Building

```sh
cargo run --release --package game_client
cargo test
```

The simulation runs headless and builds for `wasm32-unknown-unknown`, so it can
be tested without a window and embedded anywhere.

## License

MIT — see [LICENSE](LICENSE).
