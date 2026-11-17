//! Where the client keeps its files (saves today, settings later): named
//! slots of bytes behind [`SlotStore`], one backend per platform. The
//! simulation only produces bytes (ADR-0017); this module decides where they
//! go.
//!
//! | Platform | Backend |
//! | --- | --- |
//! | Linux, BSD | [`FileStore`] in `$XDG_DATA_HOME/<app>` or `~/.local/share/<app>` |
//! | macOS | [`FileStore`] in `~/Library/Application Support/<app>` |
//! | iOS | [`FileStore`] in the app container's `Library/Application Support/<app>` |
//! | Windows | [`FileStore`] in `%APPDATA%\<app>` |
//! | Android | [`FileStore`] in the app's internal data folder |
//! | Web | `WebStore`: browser `localStorage`, bytes as base64 |
//! | anything else | [`MemoryStore`]: gone when the app ends |
//!
//! A new backend implements [`SlotStore`] and is picked in [`platform_store`].

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;

use bevy::prelude::*;

/// The folder name under the user's data folder. The crate name, so renaming
/// the crate for a new game renames the folder too.
pub const APP_FOLDER: &str = env!("CARGO_PKG_NAME");

/// Named slots of bytes.
pub trait SlotStore: Send + Sync + 'static {
    /// The bytes in `slot`, or `None` if nothing was stored there yet.
    fn read(&self, slot: &str) -> io::Result<Option<Vec<u8>>>;
    /// Stores `bytes` in `slot`, replacing what was there.
    fn write(&mut self, slot: &str, bytes: &[u8]) -> io::Result<()>;
}

/// The client's store.
#[derive(Resource)]
pub struct Storage(pub Box<dyn SlotStore>);

/// Slot names are short identifiers, never paths: a slot can never point
/// outside the store's folder.
fn check_slot(slot: &str) -> io::Result<()> {
    let valid = !slot.is_empty()
        && slot.len() <= 64
        && slot
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
    if valid {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid slot name `{slot}`"),
        ))
    }
}

/// One file per slot, `<slot>.save`, in one folder.
#[derive(Debug, Clone)]
pub struct FileStore {
    dir: PathBuf,
}

impl FileStore {
    /// A store in `dir`; the folder is created on the first write.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    fn path(&self, slot: &str) -> io::Result<PathBuf> {
        check_slot(slot)?;
        Ok(self.dir.join(format!("{slot}.save")))
    }
}

impl SlotStore for FileStore {
    fn read(&self, slot: &str) -> io::Result<Option<Vec<u8>>> {
        match std::fs::read(self.path(slot)?) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Writes a temporary file and renames it over the old one, so a crash
    /// mid-write leaves the previous save intact.
    fn write(&mut self, slot: &str, bytes: &[u8]) -> io::Result<()> {
        let path = self.path(slot)?;
        std::fs::create_dir_all(&self.dir)?;
        let temporary = path.with_extension("save.tmp");
        std::fs::write(&temporary, bytes)?;
        std::fs::rename(&temporary, &path)
    }
}

/// Slots in memory: gone when the app ends. The fallback where a platform's
/// store is unavailable, and the store for tests.
#[derive(Debug, Clone, Default)]
pub struct MemoryStore(BTreeMap<String, Vec<u8>>);

impl SlotStore for MemoryStore {
    fn read(&self, slot: &str) -> io::Result<Option<Vec<u8>>> {
        check_slot(slot)?;
        Ok(self.0.get(slot).cloned())
    }

    fn write(&mut self, slot: &str, bytes: &[u8]) -> io::Result<()> {
        check_slot(slot)?;
        self.0.insert(slot.to_owned(), bytes.to_vec());
        Ok(())
    }
}

/// The user's data folder for this app on desktop platforms and iOS, from the
/// environment `var` (a parameter so tests can fake it); `None` elsewhere or
/// when the variables are missing. On iOS, `HOME` is the app's sandbox
/// container, so the folder lands in its `Library/Application Support`.
pub fn user_data_dir(var: impl Fn(&str) -> Option<OsString>) -> Option<PathBuf> {
    let absolute = |name: &str| var(name).map(PathBuf::from).filter(|p| p.is_absolute());
    let base = if cfg!(target_os = "windows") {
        absolute("APPDATA")
    } else if cfg!(any(target_os = "macos", target_os = "ios")) {
        absolute("HOME").map(|home| home.join("Library").join("Application Support"))
    } else if cfg!(all(
        unix,
        not(any(
            target_os = "android",
            target_os = "ios",
            target_os = "macos"
        ))
    )) {
        absolute("XDG_DATA_HOME")
            .or_else(|| absolute("HOME").map(|h| h.join(".local").join("share")))
    } else {
        None
    };
    base.map(|dir| dir.join(APP_FOLDER))
}

/// The key of `slot` in a store shared with other apps (the browser's
/// `localStorage` is shared by every page of the origin).
#[cfg(any(test, all(target_arch = "wasm32", target_os = "unknown")))]
fn web_key(slot: &str) -> io::Result<String> {
    check_slot(slot)?;
    Ok(format!("{APP_FOLDER}/{slot}"))
}

/// Bytes as text for a store that only holds strings.
#[cfg(any(test, all(target_arch = "wasm32", target_os = "unknown")))]
fn encode_text(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

/// The bytes [`encode_text`] turned into `text`.
#[cfg(any(test, all(target_arch = "wasm32", target_os = "unknown")))]
fn decode_text(text: &str) -> io::Result<Vec<u8>> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(text)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// The browser's `localStorage`, one key per slot (`<app>/<slot>`), bytes as
/// base64. Kept by the browser across reloads; most browsers allow about
/// 5 MB per origin, so about 3.7 MB of save data.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Debug, Clone, Default)]
pub struct WebStore;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl WebStore {
    /// The page's `localStorage`. Looked up on every call: it is a JS value,
    /// which a resource cannot hold (not `Send`).
    fn local_storage() -> io::Result<web_sys::Storage> {
        web_sys::window()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Unsupported, "no browser window"))?
            .local_storage()
            .map_err(js_error)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Unsupported, "no localStorage"))
    }

    /// Whether the browser lets this page use `localStorage` (it may not, in
    /// some private modes or with storage blocked).
    pub fn available() -> bool {
        Self::local_storage().is_ok()
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn js_error(value: wasm_bindgen::JsValue) -> io::Error {
    io::Error::other(format!("{value:?}"))
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl SlotStore for WebStore {
    fn read(&self, slot: &str) -> io::Result<Option<Vec<u8>>> {
        let key = web_key(slot)?;
        match Self::local_storage()?.get_item(&key).map_err(js_error)? {
            Some(text) => decode_text(&text).map(Some),
            None => Ok(None),
        }
    }

    /// Fails when the browser's quota is used up; the old save stays then.
    fn write(&mut self, slot: &str, bytes: &[u8]) -> io::Result<()> {
        let key = web_key(slot)?;
        Self::local_storage()?
            .set_item(&key, &encode_text(bytes))
            .map_err(js_error)
    }
}

/// The app's internal data folder on Android, from the `AndroidApp` Bevy
/// keeps; private to the app and kept across restarts.
#[cfg(target_os = "android")]
fn android_data_dir() -> Option<PathBuf> {
    bevy::android::ANDROID_APP
        .get()
        .and_then(|app| app.internal_data_path())
}

/// The store for this platform (see the table at the top); memory where the
/// platform's store is unavailable.
pub fn platform_store() -> Box<dyn SlotStore> {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    if WebStore::available() {
        info!("saves are stored in the browser's localStorage");
        return Box::new(WebStore);
    }
    #[cfg(target_os = "android")]
    let dir = android_data_dir();
    #[cfg(not(target_os = "android"))]
    let dir = user_data_dir(|name| std::env::var_os(name));
    match dir {
        Some(dir) => {
            info!("saves are stored in {}", dir.display());
            Box::new(FileStore::new(dir))
        }
        None => {
            warn!("no save folder on this platform: saves last until the app ends");
            Box::new(MemoryStore::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh, empty folder under the system's temporary folder.
    fn scratch_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("{APP_FOLDER}-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    fn round_trip(store: &mut dyn SlotStore) {
        assert_eq!(store.read("quicksave").unwrap(), None);
        store.write("quicksave", b"first").unwrap();
        store.write("quicksave", b"second").unwrap();
        store.write("other", b"third").unwrap();
        assert_eq!(
            store.read("quicksave").unwrap().as_deref(),
            Some(&b"second"[..])
        );
        assert_eq!(store.read("other").unwrap().as_deref(), Some(&b"third"[..]));
    }

    #[test]
    fn file_store_round_trips_and_replaces() {
        let dir = scratch_dir("round-trip");
        let nested = dir.join("nested");
        let mut store = FileStore::new(&nested);
        round_trip(&mut store);
        let mut names: Vec<_> = std::fs::read_dir(&nested)
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        names.sort();
        assert_eq!(
            names,
            ["other.save", "quicksave.save"],
            "no temporary file left"
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn memory_store_round_trips_and_replaces() {
        round_trip(&mut MemoryStore::default());
    }

    #[test]
    fn slot_names_cannot_leave_the_folder() {
        let mut store = FileStore::new(scratch_dir("slots"));
        for slot in ["", "../escape", "a/b", "a\\b", "C:", "."] {
            assert!(store.read(slot).is_err(), "`{slot}` was accepted");
            assert!(store.write(slot, b"x").is_err(), "`{slot}` was accepted");
        }
        assert!(MemoryStore::default().write("../escape", b"x").is_err());
    }

    #[test]
    fn web_keys_carry_the_app_and_refuse_bad_slots() {
        assert_eq!(
            web_key("quicksave").unwrap(),
            format!("{APP_FOLDER}/quicksave")
        );
        assert!(web_key("../escape").is_err());
        assert!(web_key("").is_err());
    }

    #[test]
    fn text_encoding_round_trips_every_byte() {
        let bytes: Vec<u8> = (0..=255).collect();
        let text = encode_text(&bytes);
        assert!(text.is_ascii(), "localStorage holds the text unchanged");
        assert_eq!(decode_text(&text).unwrap(), bytes);
        assert_eq!(decode_text(&encode_text(&[])).unwrap(), Vec::<u8>::new());
        let refused = decode_text("not base64!").unwrap_err();
        assert_eq!(refused.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn data_dir_follows_the_platform_convention() {
        let env = |pairs: &'static [(&str, &str)]| {
            move |name: &str| {
                pairs
                    .iter()
                    .find(|(k, _)| *k == name)
                    .map(|(_, v)| OsString::from(v))
            }
        };
        assert_eq!(user_data_dir(env(&[])), None);
        if cfg!(target_os = "linux") {
            assert_eq!(
                user_data_dir(env(&[("HOME", "/home/a")])),
                Some(PathBuf::from("/home/a/.local/share").join(APP_FOLDER))
            );
            assert_eq!(
                user_data_dir(env(&[("HOME", "/home/a"), ("XDG_DATA_HOME", "/data")])),
                Some(PathBuf::from("/data").join(APP_FOLDER))
            );
            // The XDG spec says to ignore relative paths.
            assert_eq!(
                user_data_dir(env(&[("HOME", "/home/a"), ("XDG_DATA_HOME", "rel")])),
                Some(PathBuf::from("/home/a/.local/share").join(APP_FOLDER))
            );
        }
        if cfg!(any(target_os = "macos", target_os = "ios")) {
            assert_eq!(
                user_data_dir(env(&[("HOME", "/Users/a")])),
                Some(PathBuf::from("/Users/a/Library/Application Support").join(APP_FOLDER))
            );
        }
        if cfg!(target_os = "windows") {
            assert_eq!(
                user_data_dir(env(&[("APPDATA", r"C:\Users\a\AppData\Roaming")])),
                Some(PathBuf::from(r"C:\Users\a\AppData\Roaming").join(APP_FOLDER))
            );
        }
    }
}
