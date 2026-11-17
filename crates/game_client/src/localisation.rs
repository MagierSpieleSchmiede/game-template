//! Localisation: every visible text is looked up by key (template default,
//! ADR-0016). The texts live in `data/locales/<locale>.txt`, compiled in, so
//! they work on every platform without a file system.

use std::collections::BTreeMap;

use bevy::prelude::*;

/// The texts of the active locale.
#[derive(Debug, Clone, Resource)]
pub struct Localisation {
    texts: BTreeMap<String, String>,
}

impl Localisation {
    /// Builds the table from a `key = value` file.
    pub fn from_text(text: &str) -> Result<Self, String> {
        let texts = crate::kv::parse(text)?
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();
        Ok(Self { texts })
    }

    /// The English texts shipped with the game.
    pub fn english() -> Self {
        Self::from_text(include_str!("../data/locales/en.txt")).expect("en.txt is valid")
    }

    /// The text for `key` with each `{name}` replaced by its value. A missing
    /// key shows as the key itself, so it is visible on screen, not hidden.
    pub fn text(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut text = self
            .texts
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_owned());
        for (name, value) in args {
            text = text.replace(&format!("{{{name}}}"), value);
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_arguments_and_shows_missing_keys() {
        let l = Localisation::from_text("hi = Hello {who}!").unwrap();
        assert_eq!(l.text("hi", &[("who", "you")]), "Hello you!");
        assert_eq!(l.text("missing-key", &[]), "missing-key");
    }

    #[test]
    fn english_has_every_key_the_client_uses() {
        let l = Localisation::english();
        for key in [
            "menu-title",
            "menu-start",
            "paused",
            "save-load-hint",
            "debug-overlay",
        ] {
            assert_ne!(l.text(key, &[]), key, "en.txt lacks {key}");
        }
    }
}
