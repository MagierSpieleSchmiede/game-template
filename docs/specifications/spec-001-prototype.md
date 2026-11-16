---
title: "Spec 001 — the fundamental prototype"
---
# Spec 001 — the fundamental prototype

> [!tip] Template page
> Rewrite this spec for your game: scope, requirements, acceptance criteria. Keep the shape.

## Goal

A playable prototype that proves the architecture: units in an arena, movement, one ability, a win condition — fully deterministic, headless-testable, on every platform.

## Toolchain

| What | Choice |
| --- | --- |
| Language | Rust (ADR-0011) |
| Engine | Bevy (ADR-0011); Avian for physics is a template default (ADR-0015) |
| Simulation | `game_sim`, deterministic ticks in `FixedUpdate` (ADR-0001) |
| Tests | `cargo test`, headless app, determinism test |
| CI | fmt, clippy, test on desktop; `cargo check` of the sim for wasm32, Android, iOS |
| Docs | This vault, published with Quartz |

## Scope

In scope: tick chain, intents, one arena, units, movement, one ability, health, a win condition, minimal presentation (camera, HUD), localisation-ready text.

Out of scope (see [[extension-points|Extension points]]): network transport, AI/RL, add-ons, audio, save files, animation from asset files. Already in the template: menu/pause screens, input actions, localisation, snapshots, replays, headless rollback.

## Requirements

1. Same scenario, same inputs, same seed → same end state (tested, [[determinism-test|Determinism test]]).
2. The simulation builds for `wasm32-unknown-unknown` (CI).
3. No gameplay code outside the tick chain.
4. All gameplay constants in the balance resource (ADR-0010).
5. Input through actions; text through localisation keys.

## Acceptance criteria

- [ ] `cargo test` green, including the determinism test
- [ ] The prototype runs on Windows, Linux, macOS
- [ ] A headless run of a scripted match produces the documented result
- [ ] CI green on all tiers ([[platforms|Platforms]])
