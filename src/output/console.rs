use std::io::{self, Write};

use indicatif::ProgressBar;

use crate::core::types::{FoundAddress, SecretInfo};

use super::ResultSink;
/// Simple console sink that prints found address to stdout
pub struct ConsoleResultSink {
    writer: Option<Box<dyn Write + Send>>,
    progress_bar: Option<ProgressBar>,
}

impl ConsoleResultSink {
    pub fn new() -> Self {
        Self {
            writer: Some(Box::new(io::stdout())),
            progress_bar: None,
        }
    }

    pub fn with_progress_bar(pb: ProgressBar) -> Self {
        Self {
            writer: None,
            progress_bar: Some(pb),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_writer<W>(writer: W) -> Self
    where
        W: Write + Send + 'static,
    {
        Self {
            writer: Some(Box::new(writer)),
            progress_bar: None,
        }
    }
}

impl ResultSink for ConsoleResultSink {
    fn handle(&mut self, found: &FoundAddress) -> io::Result<()> {
        let secret = format_secret(&found.secret);
        let line = format!("{} | {}", found.address, secret);

        if let Some(pb) = &self.progress_bar {
            pb.println(line);
            Ok(())
        } else if let Some(writer) = &mut self.writer {
            writeln!(writer, "{line}")
        } else {
            // Nothing configured; do nothing.
            Ok(())
        }
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
