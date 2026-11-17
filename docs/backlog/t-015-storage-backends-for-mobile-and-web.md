---
title: "T-015 — Client storage backends for Android, iOS and the web"
---
# T-015 — Client storage backends for Android, iOS and the web

**Status:** in progress · **Scope:** template · **Depends on:** T-008

**Read for that:** [[platforms|Platforms]], ADR-0016, `crates/game_client/src/storage.rs`

**What:** Saves are files on desktop, but on Android, iOS and the web the client falls back to `MemoryStore`, so a save is gone when the app ends. Add one `SlotStore` backend per platform and pick it in `platform_store`: the app's internal data folder on Android (from the `AndroidApp` Bevy holds), the app container's `Library/Application Support` on iOS, browser `localStorage` (bytes as base64) on the web. Check every new dependency for the platforms first.

**Done when:** each platform keeps a save across an app restart (checked on a device or emulator, or in the browser); desktop behaviour is unchanged.

**Built:** `storage.rs` picks per platform: `FileStore` in the app's internal data folder on Android (`bevy::android::ANDROID_APP`), in `$HOME/Library/Application Support/<app>` on iOS (`HOME` is the app container there), `WebStore` on the web (`localStorage`, key `<app>/<slot>`, base64); `MemoryStore` stays the fallback when the platform's store is unavailable. Pinned natively by `web_keys_carry_the_app_and_refuse_bad_slots` and `text_encoding_round_trips_every_byte`.

**Checked:** web in headless Chromium (wasm-bindgen 0.2.130, debug build): F5 in a run wrote `game_client/quicksave`; after a page reload, F9 on the menu started the saved run. Desktop unchanged (tests green).

**Still open:** Android and iOS are not compiled or run anywhere yet: CI checks only `game_sim` for them (no NDK or Apple SDK here), and the client has no Android entry point (`android_main`/cdylib), so `ANDROID_APP` is only set once a game packages for Android. Check on a device or emulator when that packaging exists; then set this task to done.
