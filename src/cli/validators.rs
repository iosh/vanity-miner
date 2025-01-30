const MIN_MNEMONIC_WORDS: usize = 12;
const MAX_MNEMONIC_WORDS: usize = 24;

/// Validates the mnemonic word count.
pub fn parse_mnemonic_word_count(s: &str) -> Result<usize, String> {
    let count: usize = s
        .parse()
        .map_err(|_| "Word count must be a number".to_string())?;

    if count >= MIN_MNEMONIC_WORDS && count <= MAX_MNEMONIC_WORDS && count % 3 == 0 {
        Ok(count)
    } else {
        Err(format!(
            "Word count must be 12, 15, 18, 21, or 24. Got {}",
            count
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mnemonic_word_counts() {
        assert!(parse_mnemonic_word_count("12").is_ok());
        assert!(parse_mnemonic_word_count("15").is_ok());
        assert!(parse_mnemonic_word_count("18").is_ok());
        assert!(parse_mnemonic_word_count("21").is_ok());
        assert!(parse_mnemonic_word_count("24").is_ok());
    }

    #[test]
    fn test_invalid_mnemonic_word_counts() {
        assert!(parse_mnemonic_word_count("13").is_err());
        assert!(parse_mnemonic_word_count("0").is_err());
        assert!(parse_mnemonic_word_count("25").is_err());
        assert!(parse_mnemonic_word_count("abc").is_err());
    }
}
