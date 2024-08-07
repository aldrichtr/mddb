use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

impl Logger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(Logger))
            .map(|()| log::set_max_level(LevelFilter::Info))
    }
}
