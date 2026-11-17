---
title: "T-014 — Debug overlay: compute the state hash only while visible"
---
# T-014 — Debug overlay: compute the state hash only while visible

**Status:** open · **Scope:** template · **Depends on:** —

**Read for that:** ADR-0016

**What:** The client computes a full state hash after every tick just for the debug overlay. That is cheap for the example and expensive for large worlds. Compute it only while the overlay is visible.

**Done when:** With the overlay hidden, no state hash is computed per tick.
