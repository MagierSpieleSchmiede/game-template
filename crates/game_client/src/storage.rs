//! Where the client keeps its files (saves today, settings later): named
//! slots of bytes behind [`SlotStore`], one backend per platform. The
//! simulation only produces bytes (ADR-0017); this module decides where they
//! go.
//!
//! | Platform | Backend |
//! | --- | --- |
//! | Linux, BSD | [`FileStore`] in `$XDG_DATA_HOME/<app>` or `~/.local/share/<app>` |
//! | macOS | [`FileStore`] in `~/Library/Application Support/<app>` |
//! | Windows | [`FileStore`] in `%APPDATA%\<app>` |
//! | Android, iOS, web | [`MemoryStore`] until they get their own backend |
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

/// Slots in memory: gone when the app ends. The stand-in on platforms without
/// a backend yet, and the store for tests.
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

/// The user's data folder for this app on desktop platforms, from the
/// environment `var` (a parameter so tests can fake it); `None` elsewhere or
/// when the variables are missing.
pub fn user_data_dir(var: impl Fn(&str) -> Option<OsString>) -> Option<PathBuf> {
    let absolute = |name: &str| var(name).map(PathBuf::from).filter(|p| p.is_absolute());
    let base = if cfg!(target_os = "windows") {
        absolute("APPDATA")
    } else if cfg!(target_os = "macos") {
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

/// The store for this platform: files in the user's data folder where there
/// is one, memory otherwise.
pub fn platform_store() -> Box<dyn SlotStore> {
    match user_data_dir(|name| std::env::var_os(name)) {
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
        if cfg!(target_os = "macos") {
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
