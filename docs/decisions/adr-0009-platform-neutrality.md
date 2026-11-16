---
title: "ADR-0009 — Platform neutrality: the simulation builds for wasm32"
---
# ADR-0009 — Platform neutrality: the simulation builds for wasm32

**Status:** accepted

**Context:** Targets are Windows, Linux, macOS, Android, iOS and, when feasible, the web. A dependency that pulls in threads, file system access or system time into the simulation excludes web — and usually breaks headless servers and RL training too.

**Decision:** `game_sim` must always build for `wasm32-unknown-unknown`: no threads, no file system, no system time, no clock. CI checks the wasm target (and Android, iOS) on every change. Every new dependency is checked for platform support before it enters `Cargo.toml`.

**Rejected alternative:** Deciding platforms per feature, late. Costs: rewrites, and features that cannot be undone once shipped.

**Consequences:** Web stays an option for free; the simulation stays embeddable everywhere. Costs: some convenient crates (threads, `std::time`, file logging) are off limits in the simulation; the client carries the platform-specific parts alone.
