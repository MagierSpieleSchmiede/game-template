---
title: "T-015 — Client storage backends for Android, iOS and the web"
---
# T-015 — Client storage backends for Android, iOS and the web

**Status:** open · **Scope:** template · **Depends on:** T-008

**Read for that:** [[platforms|Platforms]], ADR-0016, `crates/game_client/src/storage.rs`

**What:** Saves are files on desktop, but on Android, iOS and the web the client falls back to `MemoryStore`, so a save is gone when the app ends. Add one `SlotStore` backend per platform and pick it in `platform_store`: the app's internal data folder on Android (from the `AndroidApp` Bevy holds), the app container's `Library/Application Support` on iOS, browser `localStorage` (bytes as base64) on the web. Check every new dependency for the platforms first.

**Done when:** each platform keeps a save across an app restart (checked on a device or emulator, or in the browser); desktop behaviour is unchanged.
