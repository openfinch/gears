use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use super::manager::Manager;

const LOG_LEVEL: LogLevel = LogLevel::DEBUG;

impl Manager for LogManager {
    fn m_type(&self) -> &str {
        "log_manager"
    }
}

enum LogLevel {
    ERROR = 3,
    WARN = 2,
    INFO = 1,
    DEBUG = 0,
}

pub struct LogManager {
    started: bool,
    file_handle: File,
}

impl LogManager {
    pub fn new() -> LogManager {
        LogManager {
            started: false,
            file_handle: OpenOptions::new()
                .append(true)
                .create(true)
                .open("gears.log")
                .unwrap(),
        }
    }
    pub fn startup(&mut self) {
        self.info(String::from("Starting..."));
        self.started = true;
    }
    pub fn shutdown(mut self) {
        self.started = false
    }
    pub fn error(&self, msg: String) {
        self.write_message(msg, LogLevel::ERROR)
    }
    pub fn warn(&self, msg: String) {
        self.write_message(msg, LogLevel::WARN)
    }
    pub fn info(&self, msg: String) {
        self.write_message(msg, LogLevel::INFO)
    }
    pub fn debug(&self, msg: String) {
        self.write_message(msg, LogLevel::DEBUG)
    }
    fn write_message(&self, msg: String, level: LogLevel) {
        if level as i8 >= LOG_LEVEL as i8 {
            if let Err(e) = writeln!(&self.file_handle, "{}", msg) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}
