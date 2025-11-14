mod console;
mod csv;
mod result;

pub use console::ConsoleResultSink;
pub use csv::CsvResultSink;
pub use result::{CombinedResultSink, DynResultSink, ResultSink};
