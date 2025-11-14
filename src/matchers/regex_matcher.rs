use crate::core::matcher::Matcher;
use regex::Regex;

/// Regex-based matcher.
///
/// The pattern syntax is that of the `regex` crate.
/// Case-sensitivity is controlled by the pattern itself, e.g. using `(?i)`
/// for case-insensitive matching. A higher-level builder can decide whether
/// to inject `(?i)` automatically for hex addresses.
#[derive(Debug, Clone)]
pub struct RegexMatcher {
    regex: Regex,
}

impl RegexMatcher {
    pub fn new(pattern: &str) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern)?;
        Ok(Self { regex })
    }

    pub fn as_str(&self) -> &str {
        self.regex.as_str()
    }
}

impl Matcher for RegexMatcher {
    fn matches(&self, address: &str) -> bool {
        self.regex.is_match(address)
    }

    fn description(&self) -> String {
        format!("regex:{}", self.regex.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::matcher::Matcher;

    #[test]
    fn regex_matcher_basic_match() {
        let m = RegexMatcher::new("^abc[0-9]+$").unwrap();
        assert!(m.matches("abc123"));
        assert!(!m.matches("xabc123"));
        assert!(!m.matches("abc"));
    }

    #[test]
    fn regex_matcher_case_insensitive_flag() {
        let m = RegexMatcher::new("(?i)^abc$").unwrap();
        assert!(m.matches("abc"));
        assert!(m.matches("AbC"));
        assert!(m.matches("ABC"));
    }
}
