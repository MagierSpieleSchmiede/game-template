---
title: "T-008 — Client: save and load, stored per platform"
---
# T-008 — Client: save and load, stored per platform

**Status:** done · **Scope:** template · **Depends on:** —

**Read for that:** ADR-0017, [[platforms|Platforms]]

**What:** The simulation produces save bytes (`SaveGame::to_bytes`) but the client stores none. Add save and load actions and a storage per platform: a file in the user's data folder on desktop, the app's data folder on mobile, browser storage on the web. Loading restores the snapshot into the running world; the presentation follows through the observation.

**Done when:** Saving and loading works on desktop; storage is abstracted so mobile and web need only their own backend; a refused file (other version, damaged) shows a localised message.

**Result:** `saving.rs` (actions `save` F5/North, `load` F9/West; notices) and `storage.rs` (`SlotStore`, `FileStore`, `MemoryStore`); mobile and web backends follow in [[t-015-storage-backends-for-mobile-and-web|T-015]].
