use std::sync::Arc;

use crate::core::matcher::{DynMatcher, Matcher};

use super::{
    CompositeLogic, CompositeMatcher, ContainsMatcher, PrefixMatcher, RegexMatcher, SuffixMatcher,
};

/// High-level configuration for building a matcher.
///
/// This is the shape we expect from CLI/config layers.
/// All fields are optional; if nothing is set, the resulting matcher
/// will match any address.
#[derive(Debug, Clone, Default)]
pub struct MatcherConfig {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub contains: Vec<String>,
    pub regex: Option<String>,
}

/// Build a matcher from the provided configuration.
///
/// Rules:
/// - If no constraints are set, returns an "always match" matcher.
/// - If exactly one matcher is created, it is returned directly.
/// - If multiple matchers are created, they are combined with `CompositeLogic::All`.
///
/// Only regex construction can fail, so the error type is `regex::Error`.
pub fn build_matcher(config: MatcherConfig) -> Result<DynMatcher, regex::Error> {
    let mut matchers: Vec<DynMatcher> = vec![];

    if let Some(prefix) = config.prefix {
        matchers.push(Arc::new(PrefixMatcher::new(prefix)) as DynMatcher);
    }

    if let Some(suffix) = config.suffix {
        matchers.push(Arc::new(SuffixMatcher::new(suffix)) as DynMatcher);
    }

    if !config.contains.is_empty() {
        matchers.push(Arc::new(ContainsMatcher::new(config.contains)) as DynMatcher);
    }

    if let Some(pattern) = config.regex {
        matchers.push(Arc::new(RegexMatcher::new(&pattern)?) as DynMatcher);
    }

    let result: DynMatcher = match matchers.len() {
        0 => Arc::new(AlwaysMatch) as DynMatcher,
        1 => matchers.into_iter().next().unwrap(),
        _ => Arc::new(CompositeMatcher::new(CompositeLogic::All, matchers)) as DynMatcher,
    };

    Ok(result)
}

/// A matcher that always returns `true`.
///
/// Used as a fallback when no constraints are configured.
#[derive(Debug)]
struct AlwaysMatch;

impl Matcher for AlwaysMatch {
    fn matches(&self, _address: &str) -> bool {
        true
    }

    fn description(&self) -> String {
        "always".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_config_yields_always_matcher() {
        let cfg = MatcherConfig::default();
        let matcher = build_matcher(cfg).expect("build matcher");
        assert!(matcher.matches("anything"));
        assert_eq!(matcher.description(), "always");
    }

    #[test]
    fn prefix_only_config_builds_prefix_matcher() {
        let cfg = MatcherConfig {
            prefix: Some("abc".into()),
            ..Default::default()
        };
        let matcher = build_matcher(cfg).expect("build matcher");
        assert!(matcher.matches("abcdef"));
        assert!(!matcher.matches("xabcdef"));
        assert!(matcher.description().starts_with("prefix:"));
    }

    #[test]
    fn multiple_fields_build_composite_all() {
        let cfg = MatcherConfig {
            prefix: Some("abc".into()),
            suffix: Some("xyz".into()),
            ..Default::default()
        };

        let matcher = build_matcher(cfg).expect("build matcher");

        assert!(matcher.matches("abc000xyz"));
        assert!(!matcher.matches("abc000000"));
        assert!(!matcher.matches("000000xyz"));

        let desc = matcher.description();
        assert!(desc.starts_with("all("));
        assert!(desc.contains("prefix:"));
        assert!(desc.contains("suffix:"));
    }

    #[test]
    fn regex_errors_are_propagated() {
        let cfg = MatcherConfig {
            regex: Some("(".into()), // invalid regex
            ..Default::default()
        };

        let err = build_matcher(cfg).err().expect("expected regex error");
        assert!(err.to_string().contains("unclosed"));
    }
}
