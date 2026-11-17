---
title: "T-004 — The game's actions and their input actions"
---
# T-004 — The game's actions and their input actions

**Status:** open · **Scope:** game · **Depends on:** [[t-003-replace-the-example-game|T-003]]

**Read for that:** [[intent-and-controls|Intent & controls]]

**What:** Define the game's `Action` variants and `validate`, the input actions that produce them, and their default bindings in `data/input/bindings.txt`.

**Done when:** Every action can be triggered on keyboard/mouse and on gamepad; malformed actions are rejected by `validate`, with tests.
