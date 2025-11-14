use std::{
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
};

use csv::Writer;

use crate::core::types::{FoundAddress, SecretInfo};

use super::ResultSink;

/// CSV sink that appends found addresses to a CSV file
pub struct CsvResultSink {
    path: PathBuf,
    writer: Writer<std::fs::File>,
}

impl CsvResultSink {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path_buf = path.as_ref().to_path_buf();

        let file_exists_and_non_empty =
            path_buf.exists() && path_buf.metadata().map(|m| m.len() > 0).unwrap_or(false);

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&path_buf)?;

        let mut writer = Writer::from_writer(file);

        if !file_exists_and_non_empty {
            writer
                .write_record(&["address", "secret"])
                .map_err(csv_to_io_error)?;
            writer.flush()?;
        }

        Ok(Self {
            path: path_buf,
            writer,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl ResultSink for CsvResultSink {
    fn handle(&mut self, found: &FoundAddress) -> io::Result<()> {
        let secret = format_secret(&found.secret);

        self.writer
            .write_record(&[found.address.as_str(), secret.as_str()])
            .map_err(csv_to_io_error)?;
        self.writer.flush()?;

        Ok(())
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

fn csv_to_io_error(err: csv::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{FoundAddress, SecretInfo};

    #[test]
    fn csv_sink_writes_header_and_record() {
        let mut path = std::env::temp_dir();
        path.push("vanity_miner_csv_test_output.csv");

        let _ = std::fs::remove_file(&path);

        let mut sink = CsvResultSink::new(&path).expect("create csv sink");

        let found = FoundAddress {
            address: "0xaddr".into(),
            secret: SecretInfo::PrivateKey("deadbeef".into()),
        };

        sink.handle(&found).unwrap();

        let content = std::fs::read_to_string(&path).expect("read csv file");

        assert!(content.contains("address,secret"));
        assert!(content.contains("0xaddr"));
        assert!(content.contains("deadbeef"));
    }
}
