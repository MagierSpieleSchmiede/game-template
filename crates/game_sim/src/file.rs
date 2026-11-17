//! Save and replay files (template default, ADR-0017): postcard behind a
//! small versioned header.
//!
//! ```text
//! magic "GSIM" | format version | kind | SIM_VERSION | payload
//! ```
//!
//! The simulation only turns values into bytes and back; it never touches a
//! file system (there is none on wasm). Where the bytes are stored — a file
//! on desktop, the app's data folder on mobile, browser storage on the web —
//! is the client's business.
//!
//! A replay or save only reproduces its run on the simulation behaviour that
//! recorded it, so the header carries [`crate::game::SIM_VERSION`] and a file
//! from any other version is refused with [`FileError::SimVersion`] instead of
//! silently diverging.

use std::fmt;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::game::SIM_VERSION;

/// The first bytes of every file.
pub const MAGIC: [u8; 4] = *b"GSIM";

/// The version of the container layout (header and encoding). Bump it when
/// the header or the encoding changes, not when gameplay changes.
pub const FORMAT_VERSION: u16 = 1;

/// What a file holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileKind {
    /// A [`crate::save::SaveGame`].
    Save,
    /// A [`crate::replay::Replay`].
    Replay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct Header {
    magic: [u8; 4],
    format: u16,
    kind: FileKind,
    sim_version: u32,
}

/// Why bytes could not be read as a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileError {
    /// The bytes do not start with the magic: not a file of this game.
    NotAGameFile,
    /// The container layout is from another version of the template.
    FormatVersion { expected: u16, found: u16 },
    /// A save where a replay was expected, or the other way round.
    WrongKind { expected: FileKind, found: FileKind },
    /// Recorded with another simulation behaviour; it would not reproduce.
    SimVersion { expected: u32, found: u32 },
    /// The payload is damaged or truncated.
    Corrupt(String),
    /// The payload decoded, but its state does not match the hash stored with
    /// it: damaged, or edited by hand.
    HashMismatch { expected: u64, found: u64 },
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAGameFile => write!(f, "not a file of this game"),
            Self::FormatVersion { expected, found } => {
                write!(f, "file format {found}, this build reads {expected}")
            }
            Self::WrongKind { expected, found } => {
                write!(f, "expected a {expected:?} file, found a {found:?} file")
            }
            Self::SimVersion { expected, found } => write!(
                f,
                "recorded with simulation version {found}, this build is {expected}"
            ),
            Self::Corrupt(why) => write!(f, "damaged file: {why}"),
            Self::HashMismatch { expected, found } => write!(
                f,
                "state hash {found:#018x} does not match the stored {expected:#018x}"
            ),
        }
    }
}

impl std::error::Error for FileError {}

/// Encodes `payload` as a file of `kind` for the current simulation version.
pub fn encode<T: Serialize>(kind: FileKind, payload: &T) -> Vec<u8> {
    encode_with(header(kind, SIM_VERSION), payload)
}

/// Decodes a file of `kind`, checking magic, format, kind and simulation
/// version before the payload.
pub fn decode<T: DeserializeOwned>(kind: FileKind, bytes: &[u8]) -> Result<T, FileError> {
    if !bytes.starts_with(&MAGIC) {
        return Err(FileError::NotAGameFile);
    }
    let (found, rest) = postcard::take_from_bytes::<Header>(bytes)
        .map_err(|e| FileError::Corrupt(format!("header: {e}")))?;
    if found.format != FORMAT_VERSION {
        return Err(FileError::FormatVersion {
            expected: FORMAT_VERSION,
            found: found.format,
        });
    }
    if found.kind != kind {
        return Err(FileError::WrongKind {
            expected: kind,
            found: found.kind,
        });
    }
    if found.sim_version != SIM_VERSION {
        return Err(FileError::SimVersion {
            expected: SIM_VERSION,
            found: found.sim_version,
        });
    }
    let (payload, rest) = postcard::take_from_bytes::<T>(rest)
        .map_err(|e| FileError::Corrupt(format!("payload: {e}")))?;
    if !rest.is_empty() {
        return Err(FileError::Corrupt(format!(
            "{} unexpected bytes after the payload",
            rest.len()
        )));
    }
    Ok(payload)
}

fn header(kind: FileKind, sim_version: u32) -> Header {
    Header {
        magic: MAGIC,
        format: FORMAT_VERSION,
        kind,
        sim_version,
    }
}

fn encode_with<T: Serialize>(header: Header, payload: &T) -> Vec<u8> {
    let mut bytes = postcard::to_allocvec(&header).expect("a header always encodes");
    bytes.extend(postcard::to_allocvec(payload).expect("simulation values always encode"));
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let bytes = encode(FileKind::Save, &(7u32, String::from("x")));
        assert!(bytes.starts_with(&MAGIC));
        assert_eq!(
            decode::<(u32, String)>(FileKind::Save, &bytes),
            Ok((7, "x".into()))
        );
    }

    #[test]
    fn refuses_foreign_bytes() {
        assert_eq!(
            decode::<u32>(FileKind::Save, b"PNG...."),
            Err(FileError::NotAGameFile)
        );
        assert_eq!(
            decode::<u32>(FileKind::Save, b""),
            Err(FileError::NotAGameFile)
        );
    }

    #[test]
    fn refuses_the_wrong_kind() {
        let bytes = encode(FileKind::Replay, &1u32);
        assert_eq!(
            decode::<u32>(FileKind::Save, &bytes),
            Err(FileError::WrongKind {
                expected: FileKind::Save,
                found: FileKind::Replay
            })
        );
    }

    #[test]
    fn refuses_other_versions() {
        let other_sim = encode_with(header(FileKind::Save, SIM_VERSION + 1), &1u32);
        assert_eq!(
            decode::<u32>(FileKind::Save, &other_sim),
            Err(FileError::SimVersion {
                expected: SIM_VERSION,
                found: SIM_VERSION + 1
            })
        );
        let mut other_format = header(FileKind::Save, SIM_VERSION);
        other_format.format = FORMAT_VERSION + 1;
        assert_eq!(
            decode::<u32>(FileKind::Save, &encode_with(other_format, &1u32)),
            Err(FileError::FormatVersion {
                expected: FORMAT_VERSION,
                found: FORMAT_VERSION + 1
            })
        );
    }

    #[test]
    fn refuses_truncated_and_padded_payloads() {
        let bytes = encode(FileKind::Save, &String::from("a longer payload"));
        assert!(matches!(
            decode::<String>(FileKind::Save, &bytes[..bytes.len() - 3]),
            Err(FileError::Corrupt(_))
        ));
        let mut padded = bytes.clone();
        padded.push(0);
        assert!(matches!(
            decode::<String>(FileKind::Save, &padded),
            Err(FileError::Corrupt(_))
        ));
    }
}
