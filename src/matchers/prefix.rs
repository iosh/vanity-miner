use crate::core::matcher::Matcher;

/// Matches addresses that start with a given prefix
///
/// The address passed to `matches` is expected to be *normalized*:
/// - For hex address: lowercased and without `0x` prefix
/// - For other encodings: whatever with how the pattern was constructed.
#[derive(Debug, Clone)]
pub struct PrefixMatcher {
    pattern: String,
}

impl PrefixMatcher {
    pub fn new(pattern: impl Into<String>) -> Self {
        let mut pattern = pattern.into();
        pattern.make_ascii_lowercase();
        Self { pattern }
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

impl Matcher for PrefixMatcher {
    fn matches(&self, address: &str) -> bool {
        address.starts_with(&self.pattern)
    }

    fn description(&self) -> String {
        format!("prefix:{}", self.pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::matcher::Matcher;

    #[test]
    fn matches_when_prefix_present() {
        let m = PrefixMatcher::new("abc");
        assert!(m.matches("abcdef"));
        assert!(m.matches("abc"));
        assert!(!m.matches("xabcdef"));
    }

    #[test]
    fn pattern_is_lowercased() {
        let m = PrefixMatcher::new("AbC");
        assert_eq!(m.pattern(), "abc");
    }
}
