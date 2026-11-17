---
title: "T-016 — Decide whether the template ships Avian while no code uses it"
---
# T-016 — Decide whether the template ships Avian while no code uses it

**Status:** open · **Scope:** template · **Depends on:** —

**Read for that:** ADR-0015, [[platforms|Platforms]]

**What:** `crates/game_sim/Cargo.toml` depends on `avian3d`, but no code in `game_sim` (or the client) uses it: the example game moves units without physics. Every game from the template pays its compile time and `target/` size, and the dependency suggests a 3D physics foundation the template does not have. Physics is already optional in code (ADR-0015 is a template default); the question is only whether the template should ship the dependency. Options: keep it as a ready-to-use default; drop it and leave *how to add Avian (2D or 3D)* in ADR-0015's *To replace*; or give the example game one real use so it pays its way. A change to ADR-0015 needs a new ADR (it supersedes the default).

**Done when:** the maintainer has decided; if dropped, `avian3d` is out of the workspace and `Cargo.lock`, ADR-0015 is superseded by a new ADR that says how to add it back, README and [[platforms|Platforms]] no longer list Avian as shipped, and the gates are green after one `cargo clean`.
