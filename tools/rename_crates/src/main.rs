//! Renames the template's crates `game_sim` and `game_client` for a new game, in
//! one step (T-011):
//!
//! ```text
//! cargo rename-crates <sim-name> <client-name>
//! ```
//!
//! 1. checks both names and that the working tree has no uncommitted changes,
//!    so `git diff` shows exactly what the tool did and `git checkout` undoes it;
//! 2. replaces both names in every tracked text file (Cargo files, workflows,
//!    code, docs), as whole identifiers only: `game_simulation` stays;
//! 3. moves `crates/game_sim` and `crates/game_client` to the new names;
//! 4. lets cargo rewrite `Cargo.lock` and rustfmt re-sort the imports, so the
//!    gates pass afterwards.
//!
//! The tool leaves its own folder alone: its defaults are the template's names.
//! Its messages are for developers in a terminal, not game text, so they are
//! not localised.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

/// The template's crate names, the ones this tool replaces.
const OLD_SIM: &str = "game_sim";
const OLD_CLIENT: &str = "game_client";

/// The tool's own folder, relative to the workspace root; never rewritten.
const OWN_DIR: &str = "tools/rename_crates/";

const USAGE: &str = "usage: cargo rename-crates <sim-name> <client-name>
  e.g. cargo rename-crates dragon_sim dragon_client
Renames the crates game_sim and game_client everywhere in the repository.";

/// Names that are not free for a crate of the workspace: Rust keywords that
/// can stand in a path, and the crates every Rust program can name.
const RESERVED: &[&str] = &[
    "alloc",
    "core",
    "crate",
    "proc_macro",
    "self",
    "std",
    "super",
    "test",
];

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let [sim, client] = args else {
        return Err(USAGE.to_owned());
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = root
        .canonicalize()
        .map_err(|e| format!("cannot find the workspace root: {e}"))?;

    let lock = fs::read_to_string(root.join("Cargo.lock"))
        .map_err(|e| format!("cannot read Cargo.lock: {e}"))?;
    check_names(sim, client, &lock_packages(&lock))?;
    for old in [OLD_SIM, OLD_CLIENT] {
        if !root.join("crates").join(old).is_dir() {
            return Err(format!(
                "crates/{old} does not exist: the crates were renamed already?"
            ));
        }
    }
    for new in [sim, client] {
        if root.join("crates").join(new).exists() {
            return Err(format!("crates/{new} exists already"));
        }
    }
    if !git(&root, &["status", "--porcelain"])?.is_empty() {
        return Err(
            "the working tree has uncommitted changes; commit or stash them first, \
             so the rename can be reviewed and undone with git"
                .to_owned(),
        );
    }
    let files: Vec<String> = git(&root, &["ls-files", "-z"])?
        .split('\0')
        .filter(|f| !f.is_empty())
        .map(str::to_owned)
        .collect();

    let pairs = [(OLD_SIM, sim.as_str()), (OLD_CLIENT, client.as_str())];
    let report = rewrite_files(&root, &files, &pairs)?;
    move_crates(&root, &pairs)?;
    println!(
        "renamed {OLD_SIM} -> {sim} and {OLD_CLIENT} -> {client} in {} files",
        report.changed.len()
    );
    for file in &report.skipped {
        println!("warning: {file} is not UTF-8 text but mentions an old name; check it by hand");
    }

    // The package entries in Cargo.lock are sorted by name, and imports by
    // crate name: let the tools that own those orders restore them. Each step
    // lists its attempts; `update --workspace` touches only the workspace's own
    // entries, so no dependency changes version. It needs the registry index
    // only when the local copy is missing, hence offline first.
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let steps: [(&str, &[&[&str]]); 2] = [
        (
            "update Cargo.lock",
            &[
                &["update", "--workspace", "--offline"],
                &["update", "--workspace"],
            ],
        ),
        ("format the code", &[&["fmt", "--all"]]),
    ];
    for (what, attempts) in steps {
        let ok = attempts.iter().any(|args| {
            Command::new(&cargo)
                .args(*args)
                .current_dir(&root)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .is_ok_and(|s| s.success())
        });
        if !ok {
            let last = attempts[attempts.len() - 1].join(" ");
            println!("warning: could not {what}; run `cargo {last}` yourself");
        }
    }
    println!(
        "next: run the gates from AGENTS.md, review `git diff`, then go on with T-001 \
         (repository name, site config, spec, README)"
    );
    Ok(())
}

/// Checks that the new names can be crate names of this workspace.
fn check_names(sim: &str, client: &str, taken: &[&str]) -> Result<(), String> {
    for name in [sim, client] {
        check_name(name)?;
        if name == OLD_SIM || name == OLD_CLIENT {
            return Err(format!("`{name}` is one of the template's names already"));
        }
        if taken.contains(&name) {
            return Err(format!(
                "`{name}` is already a package in Cargo.lock; pick another name"
            ));
        }
    }
    if sim == client {
        return Err("the two crates need different names".to_owned());
    }
    Ok(())
}

/// A crate name the same in Cargo and in code: a lowercase ASCII letter, then
/// lowercase letters, digits and underscores.
fn check_name(name: &str) -> Result<(), String> {
    let mut chars = name.chars();
    let first_ok = chars.next().is_some_and(|c| c.is_ascii_lowercase());
    let rest_ok = chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
    if !first_ok || !rest_ok || name.len() > 64 {
        return Err(format!(
            "`{name}` is not a crate name: start with a lowercase letter, then use \
             lowercase letters, digits and underscores (no hyphens)"
        ));
    }
    if RESERVED.contains(&name) {
        return Err(format!("`{name}` is reserved in Rust"));
    }
    Ok(())
}

/// The package names listed in a `Cargo.lock`.
fn lock_packages(lock: &str) -> Vec<&str> {
    lock.lines()
        .filter_map(|line| line.strip_prefix("name = \""))
        .filter_map(|rest| rest.strip_suffix('"'))
        .collect()
}

/// `text` with every whole-identifier occurrence of an old name replaced by
/// its new one, in one pass; `None` if nothing changed.
fn replace_names(text: &str, pairs: &[(&str, &str)]) -> Option<String> {
    let bytes = text.as_bytes();
    let is_ident = |b: Option<&u8>| b.is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_');
    let mut out = String::with_capacity(text.len());
    let (mut copied, mut i) = (0, 0);
    while i < bytes.len() {
        let hit = pairs.iter().find(|(old, _)| {
            bytes[i..].starts_with(old.as_bytes())
                && !is_ident(i.checked_sub(1).and_then(|j| bytes.get(j)))
                && !is_ident(bytes.get(i + old.len()))
        });
        match hit {
            // An old name starts with an ASCII letter, so `i` is a char boundary.
            Some((old, new)) => {
                out.push_str(&text[copied..i]);
                out.push_str(new);
                i += old.len();
                copied = i;
            }
            None => i += 1,
        }
    }
    if copied == 0 {
        return None;
    }
    out.push_str(&text[copied..]);
    Some(out)
}

#[derive(Debug, Default)]
struct Report {
    /// Files whose text changed.
    changed: Vec<String>,
    /// Files that mention an old name but are not UTF-8, so were left alone.
    skipped: Vec<String>,
}

/// Replaces the names in `files` (paths relative to `root`), except the tool's own.
fn rewrite_files(root: &Path, files: &[String], pairs: &[(&str, &str)]) -> Result<Report, String> {
    let mut report = Report::default();
    for file in files.iter().filter(|f| !f.starts_with(OWN_DIR)) {
        let path = root.join(file);
        let bytes = fs::read(&path).map_err(|e| format!("cannot read {file}: {e}"))?;
        match String::from_utf8(bytes) {
            Ok(text) => {
                if let Some(new) = replace_names(&text, pairs) {
                    fs::write(&path, new).map_err(|e| format!("cannot write {file}: {e}"))?;
                    report.changed.push(file.clone());
                }
            }
            Err(e) => {
                let bytes = e.as_bytes();
                if pairs
                    .iter()
                    .any(|(old, _)| bytes.windows(old.len()).any(|w| w == old.as_bytes()))
                {
                    report.skipped.push(file.clone());
                }
            }
        }
    }
    Ok(report)
}

/// Moves `crates/<old>` to `crates/<new>` for each pair.
fn move_crates(root: &Path, pairs: &[(&str, &str)]) -> Result<(), String> {
    for (old, new) in pairs {
        let from: PathBuf = root.join("crates").join(old);
        fs::rename(&from, root.join("crates").join(new))
            .map_err(|e| format!("cannot move crates/{old} to crates/{new}: {e}"))?;
    }
    Ok(())
}

/// Runs git in `root` and returns its output.
fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map_err(|_| "git printed something that is not UTF-8".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAIRS: [(&str, &str); 2] = [(OLD_SIM, "dragon_sim"), (OLD_CLIENT, "dragon_client")];

    #[test]
    fn replaces_whole_identifiers_only() {
        let text = "use game_sim::SimPlugin;\nmembers = [\"crates/game_sim\", \"crates/game_client\"]\n\
                    `game_sim`/`game_client` game_simulation my_game_sim game_client2 -p game_client ä game_sim";
        let expected = "use dragon_sim::SimPlugin;\nmembers = [\"crates/dragon_sim\", \"crates/dragon_client\"]\n\
                    `dragon_sim`/`dragon_client` game_simulation my_game_sim game_client2 -p dragon_client ä dragon_sim";
        assert_eq!(replace_names(text, &PAIRS).as_deref(), Some(expected));
    }

    #[test]
    fn unchanged_text_reports_none() {
        assert_eq!(
            replace_names("game_simulation, nothing to do: äöü", &PAIRS),
            None
        );
        assert_eq!(replace_names("", &PAIRS), None);
    }

    #[test]
    fn names_must_be_crate_identifiers() {
        assert!(check_name("dragon_sim").is_ok());
        assert!(check_name("d2").is_ok());
        for bad in [
            "",
            "Dragon",
            "dragon-sim",
            "2d",
            "_d",
            "dräche",
            "std",
            "self",
        ] {
            assert!(check_name(bad).is_err(), "{bad:?} must be refused");
        }
    }

    #[test]
    fn names_must_be_new_and_distinct() {
        let lock = "[[package]]\nname = \"bevy\"\nversion = \"0.20.1\"\n\n[[package]]\nname = \"game_sim\"\n";
        let taken = lock_packages(lock);
        assert_eq!(taken, ["bevy", "game_sim"]);
        assert!(check_names("dragon_sim", "dragon_client", &taken).is_ok());
        assert!(check_names("bevy", "dragon_client", &taken).is_err());
        assert!(check_names("dragon", "dragon", &taken).is_err());
        assert!(check_names("game_client", "dragon_client", &taken).is_err());
    }

    #[test]
    fn rewrites_tracked_files_and_moves_the_crates() {
        let root = env::temp_dir().join(format!("rename_crates-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let write = |file: &str, bytes: &[u8]| {
            let path = root.join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, bytes).unwrap();
        };
        write(
            "Cargo.toml",
            b"members = [\"crates/game_sim\", \"crates/game_client\"]\n",
        );
        write("crates/game_sim/Cargo.toml", b"name = \"game_sim\"\n");
        write(
            "crates/game_client/src/main.rs",
            b"use game_sim::SimPlugin;\n",
        );
        write("docs/plain.md", b"nothing here\n");
        write("data/run.replay", b"\xff\x00game_sim\xfe");
        write(
            "tools/rename_crates/src/main.rs",
            b"const OLD_SIM: &str = \"game_sim\";\n",
        );
        let files: Vec<String> = [
            "Cargo.toml",
            "crates/game_sim/Cargo.toml",
            "crates/game_client/src/main.rs",
            "docs/plain.md",
            "data/run.replay",
            "tools/rename_crates/src/main.rs",
        ]
        .map(String::from)
        .to_vec();

        let report = rewrite_files(&root, &files, &PAIRS).unwrap();
        move_crates(&root, &PAIRS).unwrap();

        assert_eq!(report.changed, files[..3]);
        assert_eq!(report.skipped, ["data/run.replay"]);
        let read = |file: &str| fs::read_to_string(root.join(file)).unwrap();
        assert_eq!(
            read("Cargo.toml"),
            "members = [\"crates/dragon_sim\", \"crates/dragon_client\"]\n"
        );
        assert_eq!(
            read("crates/dragon_sim/Cargo.toml"),
            "name = \"dragon_sim\"\n"
        );
        assert_eq!(
            read("crates/dragon_client/src/main.rs"),
            "use dragon_sim::SimPlugin;\n"
        );
        assert!(!root.join("crates/game_sim").exists());
        assert!(!root.join("crates/game_client").exists());
        assert_eq!(
            read("tools/rename_crates/src/main.rs"),
            "const OLD_SIM: &str = \"game_sim\";\n"
        );
        assert_eq!(
            fs::read(root.join("data/run.replay")).unwrap(),
            b"\xff\x00game_sim\xfe"
        );
        fs::remove_dir_all(&root).unwrap();
    }
}
