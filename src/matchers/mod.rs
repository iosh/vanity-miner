mod builder;
mod composite;
mod contains;
mod prefix;
mod regex_matcher;
mod suffix;

pub use builder::{build_matcher, MatcherConfig};
pub use composite::{CompositeLogic, CompositeMatcher};
pub use contains::ContainsMatcher;
pub use prefix::PrefixMatcher;
pub use regex_matcher::RegexMatcher;
pub use suffix::SuffixMatcher;
