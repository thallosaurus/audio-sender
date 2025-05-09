use log::{Record, Level, Metadata};

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{} {}] - {}", record.target(), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}