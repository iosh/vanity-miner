use std::sync::{atomic::AtomicU64, Arc};

#[derive(Debug, Clone)]
pub struct GenerationStats {
    pub max_attempts: u64,
    pub max_matches: u64,
    pub found_count: Arc<AtomicU64>,
    pub attempt_count: Arc<AtomicU64>,
}

impl GenerationStats {
    pub fn new(max_attempts: u64, max_matches: u64) -> Self {
        GenerationStats {
            max_attempts,
            max_matches,
            found_count: Arc::new(AtomicU64::new(0)),
            attempt_count: Arc::new(AtomicU64::new(0)),
        }
    }
}
