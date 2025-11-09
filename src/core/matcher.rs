use std::sync::Arc;

/// Address matcher trait used by the executor.
pub trait Matcher: Send + Sync {
    fn matches(&self, address: &str) -> bool;
    fn description(&self) -> String;
}

/// Convenient alias for sharing matcher instances.
pub type DynMatcher = Arc<dyn Matcher>;

#[cfg(test)]

mod tests {
    use std::sync::Arc;

    use crate::core::matcher::{DynMatcher, Matcher};

    struct AlwaysMatch;

    impl Matcher for AlwaysMatch {
        fn matches(&self, _address: &str) -> bool {
            true
        }
        fn description(&self) -> String {
            "always".into()
        }
    }

    #[test]
    fn matcher_trait_object() {
        let matcher: DynMatcher = Arc::new(AlwaysMatch);
        assert!(matcher.matches("anything"));
        assert_eq!(matcher.description(), "always");
    }
}
