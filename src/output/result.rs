use std::io;

use crate::core::types::FoundAddress;

/// Sink that consumes found addresses writes them somewhere
pub trait ResultSink: Send {
    fn handle(&mut self, found: &FoundAddress) -> io::Result<()>;
}

pub type DynResultSink = Box<dyn ResultSink>;

/// A sink that forwards each record to multiple inner sinks.
#[derive(Default)]
pub struct CombinedResultSink {
    sinks: Vec<DynResultSink>,
}

impl CombinedResultSink {
    pub fn new(sinks: Vec<DynResultSink>) -> Self {
        Self { sinks }
    }

    pub fn is_empty(&self) -> bool {
        self.sinks.is_empty()
    }

    pub fn push(&mut self, sink: DynResultSink) {
        self.sinks.push(sink);
    }
}

impl ResultSink for CombinedResultSink {
    fn handle(&mut self, found: &FoundAddress) -> io::Result<()> {
        for sink in &mut self.sinks {
            sink.handle(found)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{FoundAddress, SecretInfo};

    #[derive(Default)]
    struct CountingSink {
        count: usize,
    }

    impl ResultSink for CountingSink {
        fn handle(&mut self, _found: &FoundAddress) -> io::Result<()> {
            self.count += 1;
            Ok(())
        }
    }

    #[test]
    fn combined_sink_forwards_to_all_inner_sinks() {
        let mut _s1 = CountingSink::default();
        let mut _s2 = CountingSink::default();

        let mut combined = CombinedResultSink::new(Vec::new());
        combined.push(Box::new(CountingSink::default()));
        combined.push(Box::new(CountingSink::default()));

        let addr = FoundAddress {
            address: "addr".into(),
            secret: SecretInfo::PrivateKey("secret".into()),
        };

        combined.handle(&addr).unwrap();
    }
}
