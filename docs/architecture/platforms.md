---
title: "Platforms — targets and what excludes them"
---
# Platforms — targets and what excludes them

Targets: **Windows, Linux, macOS, Android, iOS** (tier 1: full gates), **Web (wasm32)** when feasible. The simulation crate must build for `wasm32-unknown-unknown` (ADR-0009): no threads, no file system, no system time, no clock.

| Tier | Platforms | CI |
| --- | --- | --- |
| 1 | Windows, Linux, macOS | `cargo fmt`, `cargo clippy --all-targets`, `cargo test` |
| 2 | Android, iOS | `cargo check -p game_sim` for the target |
| 3 | Web (wasm32) | `cargo check -p game_sim --target wasm32-unknown-unknown` |

Rules:

- Nothing goes in that excludes a platform; every new dependency is checked for platform support before it enters `Cargo.toml`.
- The client uses Bevy's default features (window, renderer, winit); the simulation does not (ADR-0005).
- On Android, Bevy links `android-activity`; the `android-check` feature of `game_sim` exists only so CI can check the crate alone.

Input: actions, not key codes — keyboard/mouse and gamepad are equal on every platform.
