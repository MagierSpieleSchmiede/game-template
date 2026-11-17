---
title: "ADR-0016 — The client shell: screens, input actions, localisation, debug overlay"
---
# ADR-0016 — The client shell: screens, input actions, localisation, debug overlay

**Status:** default (ADR-0012)

**Context:** Every game needs a screen flow, device-independent input, translated texts and a way to see the simulation's state while playing — and the rules forbid visible text and key bindings in code. Mature crates exist for each (Fluent for texts, input-manager crates for actions), but each is a dependency with its own release cycle against Bevy's, and the right choice depends on the game's size.

**Decision:** Small, dependency-free implementations in `crates/game_client`: **screens** (menu → playing ⇄ paused, pausing virtual time pauses the simulation); **input actions** (keyboard and gamepad equal, default bindings in `data/input/bindings.txt` with Bevy's own key and button names, resolved by reflection); **localisation** (`key = value` files in `data/locales/`, compiled in, `{name}` placeholders, a missing key shows as itself); a **debug overlay** (tick, state hash, frame rate; `toggle-debug` action); **save and load** (`save`/`load` actions on one quicksave slot, a localised notice for every outcome including each reason a file is refused; loading from the menu starts the loaded run) over a **storage** of named byte slots with one backend per platform (files in the user's data folder on desktop, found from the environment without a dependency).

**Rejected alternatives:** *Fluent and an input-manager crate now* (dependencies chosen before anyone knows what the game needs), *nothing* (every game would start by writing the same scaffolding, and the template's client could not be played).

**Consequences:** The client is playable on keyboard and gamepad, and every text and binding is data. Costs: no plural rules or grammar in localisation, no rebinding UI, no saved settings yet, one save slot only; Bevy's built-in font covers ASCII only.

**To replace:** each part is one module (`screens.rs`, `input.rs`, `localisation.rs` with `kv.rs`, `overlay.rs`, `saving.rs`, `storage.rs`) plus its data file; replace a module and keep its resource names (`ActionState`, `Localisation`), or change their users.
