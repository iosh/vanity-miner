use std::sync::Arc;

use super::config::MiningConfig;
use super::types::MiningResult;

/// Executors consume a `MiningConfig` and return aggregated stats.
pub trait Executor: Send + Sync {
    fn execute(&self, config: MiningConfig) -> MiningResult;
}

/// Shared executor handle.
pub type DynExecutor = Arc<dyn Executor>;
