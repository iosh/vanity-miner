use std::io::{self, Write};

use crate::core::types::{FoundAddress, SecretInfo};

use super::ResultSink;

/// Simple console sink that prints found address to stdout
pub struct ConsoleResultSink {
    writer: Box<dyn Write + Send>,
}

impl ConsoleResultSink {
    pub fn new() -> Self {
        Self {
            writer: Box::new(io::stdout()),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_writer<W>(writer: W) -> Self
    where
        W: Write + Send + 'static,
    {
        Self {
            writer: Box::new(writer),
        }
    }
}

impl ResultSink for ConsoleResultSink {
    fn handle(&mut self, found: &FoundAddress) -> io::Result<()> {
        let secret = format_secret(&found.secret);
        writeln!(self.writer, "{} | {}", found.address, secret)
    }
}

fn format_secret(secret: &SecretInfo) -> String {
    match secret {
        SecretInfo::PrivateKey(s) => s.clone(),
        SecretInfo::Mnemonic {
            phrase,
            derivation_path,
        } => format!("mnemonic:{}|path:{}", phrase, derivation_path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::SecretInfo;

    #[test]
    fn console_sink_writes_without_error() {
        let buffer: Vec<u8> = Vec::new();
        let mut sink = ConsoleResultSink::with_writer(buffer);

        let found = FoundAddress {
            address: "0xaddr".into(),
            secret: SecretInfo::PrivateKey("deadbeef".into()),
        };

        sink.handle(&found).unwrap();
    }
}
