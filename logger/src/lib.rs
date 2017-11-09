extern crate log;
extern crate env_logger;
extern crate chrono;

use std::env;

use log::{ LogRecord, LogLevelFilter, SetLoggerError };
use env_logger::LogBuilder;
use chrono::{ Datelike, Timelike, Local };


pub fn init() -> Result<(), SetLoggerError> {

    let format = | record: &LogRecord | {
        let time = Local::now();
        format!("[{:04}-{:02}-{:02} {:02}:{:02}:{:02}] {} - {}", time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second(), record.level(), record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());   //TODO catch properly
    }

    builder.init()
}
