use crate::core::matcher::{DynMatcher, Matcher};
use std::sync::Arc;

/// Global combination logic for composite matchers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeLogic {
    /// All inner matchers must match (logical AND).
    All,
    /// At least one inner matcher must match (logical OR).
    Any,
}

/// A matcher that combines multiple matchers with a chosen logic.
///
/// - For `CompositeLogic::All`:
///   - `matches` returns `true` only if *all* inner matchers return `true`.
///   - If there are no inner matchers, it returns `true` (vacuous truth).
///
/// - For `CompositeLogic::Any`:
///   - `matches` returns `true` if *any* inner matcher returns `true`.
///   - If there are no inner matchers, it returns `false`.
#[derive(Clone)]
pub struct CompositeMatcher {
    logic: CompositeLogic,
    matchers: Vec<DynMatcher>,
}

impl CompositeMatcher {
    pub fn new(logic: CompositeLogic, matchers: Vec<DynMatcher>) -> Self {
        Self { logic, matchers }
    }

    pub fn logic(&self) -> CompositeLogic {
        self.logic
    }

    pub fn len(&self) -> usize {
        self.matchers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.matchers.is_empty()
    }

    pub fn matchers(&self) -> &Vec<DynMatcher> {
        &self.matchers
    }
}

impl Matcher for CompositeMatcher {
    fn matches(&self, address: &str) -> bool {
        match self.logic {
            CompositeLogic::All => self.matchers.iter().all(|m| m.matches(address)),
            CompositeLogic::Any => self.matchers.iter().any(|m| m.matches(address)),
        }
    }

    fn description(&self) -> String {
        let inner_decs: Vec<String> = self.matchers.iter().map(|m| m.description()).collect();

        match self.logic {
            CompositeLogic::All => format!("all({})", inner_decs.join(",")),
            CompositeLogic::Any => format!("any({})", inner_decs.join(",")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::matcher::Matcher;

    #[derive(Debug)]
    struct Always(bool);

    impl Matcher for Always {
        fn matches(&self, _address: &str) -> bool {
            self.0
        }
        fn description(&self) -> String {
            format!("always({})", self.0)
        }
    }

    #[test]
    fn all_logic_short_circuits_on_false() {
        let m = CompositeMatcher::new(
            CompositeLogic::All,
            vec![
                Arc::new(Always(true)) as DynMatcher,
                Arc::new(Always(false)) as DynMatcher,
                Arc::new(Always(true)) as DynMatcher,
            ],
        );

        assert!(!m.matches("anything"));
    }

    #[test]
    fn any_logic_short_circuits_on_true() {
        let m = CompositeMatcher::new(
            CompositeLogic::Any,
            vec![
                Arc::new(Always(false)) as DynMatcher,
                Arc::new(Always(true)) as DynMatcher,
                Arc::new(Always(false)) as DynMatcher,
            ],
        );

        assert!(m.matches("anything"));
    }

    #[test]
    fn empty_all_is_true_empty_any_is_false() {
        let all = CompositeMatcher::new(CompositeLogic::All, Vec::new());
        let any = CompositeMatcher::new(CompositeLogic::Any, Vec::new());

        assert!(all.matches("anything"));
        assert!(!any.matches("anything"));
    }

    #[test]
    fn description_includes_inner_descriptions() {
        let m = CompositeMatcher::new(
            CompositeLogic::All,
            vec![
                Arc::new(Always(true)) as DynMatcher,
                Arc::new(Always(false)) as DynMatcher,
            ],
        );

        let desc = m.description();
        assert!(desc.starts_with("all("));
        assert!(desc.contains("always(true)"));
        assert!(desc.contains("always(false)"));
    }
}
