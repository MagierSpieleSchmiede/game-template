---
title: "Platforms — targets and what excludes them"
---
# Platforms — targets and what excludes them

Targets: **Windows, Linux, macOS, Android, iOS** (tier 1: full gates), **Web (wasm32)** when feasible. The simulation crate must build for `wasm32-unknown-unknown` (ADR-0009): no threads, no file system, no system time, no clock.

| Tier | Platforms | CI |
| --- | --- | --- |
| 1 | Windows, Linux, macOS | `cargo fmt`, `cargo clippy --all-targets`, `cargo test` |
| 2 | Android, iOS | `cargo check -p game_sim` for the target |
| 3 | Web (wasm32) | `cargo check -p game_sim --target wasm32-unknown-unknown`; `cargo build -p game_client --target wasm32-unknown-unknown` |

The web build of the client only proves that it compiles and links for `wasm32-unknown-unknown` (a full build, because only linking finds a dependency without a wasm32 implementation). Nothing runs it: no `wasm-bindgen` step, no page, no browser test. Shipping to the web is a per-game task.

Rules:

- Nothing goes in that excludes a platform; every new dependency is checked for platform support before it enters `Cargo.toml`. Checked so far beyond Bevy and Avian: `serde`, and `postcard` (no_std with `alloc`; builds for every target, ADR-0017); in the client `base64` (pure Rust, every target) and, on the web only, `web-sys` and `wasm-bindgen` (already in the graph through Bevy).
- The simulation never touches a file system: saves and replays are bytes; the client stores them where the platform allows (`game_client/src/storage.rs`): a file in the user's data folder on Windows, Linux and macOS, in the app container's `Library/Application Support` on iOS and in the app's internal data folder on Android; the browser's `localStorage` on the web. The table is at the top of `storage.rs`.
- The client uses Bevy's default features (window, renderer, winit); the simulation does not (ADR-0005).
- On Android, Bevy links `android-activity`; the `android-check` feature of `game_sim` exists only so CI can check the crate alone.

Input: actions, not key codes — keyboard/mouse and gamepad are equal on every platform.
