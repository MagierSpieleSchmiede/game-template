//! The tiny `key = value` text format of the client's data files
//! (localisation, bindings): `#` starts a comment line, blank lines are
//! skipped, the first `=` separates key and value, both are trimmed.

/// The entries of a `key = value` file, in file order, or the first bad line.
pub fn parse(text: &str) -> Result<Vec<(&str, &str)>, String> {
    let mut entries = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("line {}: expected `key = value`", number + 1));
        };
        entries.push((key.trim(), value.trim()));
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_entries_and_skips_comments() {
        let text = "# comment\n\na = 1\n b =  two words = x \n";
        assert_eq!(
            parse(text).unwrap(),
            vec![("a", "1"), ("b", "two words = x")]
        );
    }

    #[test]
    fn reports_bad_lines() {
        assert_eq!(
            parse("a = 1\noops").unwrap_err(),
            "line 2: expected `key = value`"
        );
    }
}
