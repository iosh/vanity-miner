use crate::core::matcher::Matcher;

/// Matches addresses that end with a given suffix.
///
/// Same normalization assumption as `PrefixMatcher`.
#[derive(Debug, Clone)]
pub struct SuffixMatcher {
    pattern: String,
}

impl SuffixMatcher {
    pub fn new(pattern: impl Into<String>) -> Self {
        let mut pattern = pattern.into();
        pattern.make_ascii_lowercase();
        Self { pattern }
    }
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

impl Matcher for SuffixMatcher {
    fn matches(&self, address: &str) -> bool {
        address.ends_with(&self.pattern)
    }

    fn description(&self) -> String {
        format!("suffix:{}", self.pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::matcher::Matcher;

    #[test]
    fn matches_when_suffix_present() {
        let m = SuffixMatcher::new("xyz");
        assert!(m.matches("abcxyz"));
        assert!(m.matches("xyz"));
        assert!(!m.matches("abcxy"));
    }

    #[test]
    fn pattern_is_lowercased() {
        let m = SuffixMatcher::new("XyZ");
        assert_eq!(m.pattern(), "xyz");
    }
}
