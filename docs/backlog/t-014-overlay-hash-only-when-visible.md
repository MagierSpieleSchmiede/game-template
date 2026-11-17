---
title: "T-014 — Debug overlay: compute the state hash only while visible"
---
# T-014 — Debug overlay: compute the state hash only while visible

**Status:** done · **Scope:** template · **Depends on:** —

**Read for that:** ADR-0016

**What:** The client computes a full state hash after every tick just for the debug overlay. That is cheap for the example and expensive for large worlds. Compute it only while the overlay is visible.

**Done when:** With the overlay hidden, no state hash is computed per tick.

**Done:** `Frames.hash` is an `Option`, filled after a tick only while `Frames.hash_wanted` is set; the overlay's toggle sets it and computes the hash at once, so a shown overlay has it even while paused. Pinned by `the_state_hash_is_computed_only_while_the_overlay_is_visible` in `overlay.rs`.
