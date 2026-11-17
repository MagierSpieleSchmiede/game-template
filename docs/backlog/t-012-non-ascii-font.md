---
title: "T-012 — A font asset for texts beyond ASCII"
---
# T-012 — A font asset for texts beyond ASCII

**Status:** done · **Scope:** template · **Depends on:** —

**Read for that:** ADR-0016

**What:** Bevy's built-in font covers ASCII only, so `·`, `—` or any non-English locale render as boxes. Bundle a font with a free licence and use it for all UI text.

**Done when:** A locale with non-ASCII characters renders correctly; the font's licence is in the repository.
