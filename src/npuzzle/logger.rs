use chrono::{DateTime, Local};
use log::*;

const LOGGER: Logger = Logger;

struct Logger;
impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let now: DateTime<Local> = Local::now();
        println!(
            "{} [{}] {}",
            now.format("%Y-%m-%d %H:%M:%S%.6f"),
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    if set_logger(&LOGGER).is_ok() {
        log::set_max_level(LevelFilter::Debug);
    }
}
