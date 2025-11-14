use crate::core::matcher::Matcher;

/// Matches addresses that contain *any* of the configured substrings.
///
/// This is a local OR: if at least one pattern is found, it matches.
#[derive(Debug, Clone)]
pub struct ContainsMatcher {
    patterns: Vec<String>,
}

impl ContainsMatcher {
    pub fn new(patterns: Vec<String>) -> Self {
        let patterns = patterns
            .into_iter()
            .map(|mut p| {
                p.make_ascii_lowercase();
                p
            })
            .collect();

        Self { patterns }
    }

    pub fn patterns(&self) -> &Vec<String> {
        &self.patterns
    }

    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }
}

impl Matcher for ContainsMatcher {
    fn matches(&self, address: &str) -> bool {
        self.patterns.iter().any(|p| address.contains(p))
    }

    fn description(&self) -> String {
        if self.patterns.is_empty() {
            "contains:<empty>".to_string()
        } else {
            format!("contains:{}", self.patterns.join(","))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::matcher::Matcher;

    #[test]
    fn matches_when_any_pattern_is_present() {
        let m = ContainsMatcher::new(vec!["abc".into(), "xyz".into()]);
        assert!(m.matches("000abc000"));
        assert!(m.matches("000xyz000"));
        assert!(!m.matches("000def000"));
    }

    #[test]
    fn patterns_are_lowercased() {
        let m = ContainsMatcher::new(vec!["AbC".into(), "XyZ".into()]);
        assert_eq!(m.patterns(), &["abc".to_string(), "xyz".to_string()]);
    }

    #[test]
    fn empty_contains_never_matches() {
        let m = ContainsMatcher::new(vec![]);
        assert!(!m.matches("anything"));
        assert!(m.is_empty());
    }
}
