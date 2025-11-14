mod console;
mod csv;
mod result;
mod stats_console;

pub use console::ConsoleResultSink;
pub use csv::CsvResultSink;
pub use result::{CombinedResultSink, DynResultSink, ResultSink};
pub use stats_console::ConsoleStatsSink;
