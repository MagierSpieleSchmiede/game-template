//! The UI font (template default, ADR-0016): Noto Sans Regular, compiled in
//! and put in place of Bevy's default font, so every text that does not name
//! a font of its own — all of them today — uses it. Bevy's built-in default
//! covers ASCII only; Noto Sans covers Latin, Greek and Cyrillic.
//!
//! The font is under the SIL Open Font License 1.1, see `data/fonts/OFL.txt`.
//! To change it, replace the file and `UI_FONT`; the test below checks that it
//! has a glyph for every character in every locale file.

use bevy::asset::AssetId;
use bevy::prelude::*;

/// The bytes of the UI font.
const UI_FONT: &[u8] = include_bytes!("../data/fonts/NotoSans-Regular.ttf");

/// Replaces the default font with the UI font. Add it after `DefaultPlugins`.
pub fn plugin(app: &mut App) {
    app.world_mut()
        .resource_mut::<Assets<Font>>()
        .insert(AssetId::default(), Font::from_bytes(UI_FONT.to_vec()))
        .expect("the default font id is a UUID and always valid");
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use bevy::text::TextPlugin;
    use skrifa::{FontRef, MetadataProvider};

    use super::*;

    fn charmap_covers(ch: char) -> bool {
        let font = FontRef::new(UI_FONT).expect("the UI font parses");
        font.charmap().map(ch).is_some()
    }

    #[test]
    fn covers_every_character_of_every_locale() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/locales");
        let mut locales = 0;
        for entry in std::fs::read_dir(&dir).expect("data/locales exists") {
            let path = entry.expect("readable entry").path();
            let text = std::fs::read_to_string(&path).expect("locale is UTF-8");
            for (key, value) in crate::kv::parse(&text).expect("locale parses") {
                for ch in value.chars() {
                    assert!(
                        charmap_covers(ch),
                        "{}: the UI font has no glyph for {ch:?} (U+{:04X}) in {key}",
                        path.display(),
                        u32::from(ch),
                    );
                }
            }
            locales += 1;
        }
        assert!(locales > 0, "no locale files in {}", dir.display());
    }

    #[test]
    fn covers_latin_greek_and_cyrillic_beyond_ascii() {
        for ch in "äöüßéñçøłő—–·…€ΑΩαωДЖдж«»„“”".chars() {
            assert!(charmap_covers(ch), "no glyph for {ch:?}");
        }
    }

    #[test]
    fn replaces_the_default_font() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), TextPlugin))
            .add_plugins(plugin);
        let fonts = app.world().resource::<Assets<Font>>();
        let font = fonts.get(AssetId::default()).expect("a default font");
        assert_eq!(font.data.as_ref(), UI_FONT);
    }
}
