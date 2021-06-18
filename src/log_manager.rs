use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use super::manager::Manager;

const LOG_LEVEL: LogLevel = LogLevel::Debug;

impl Manager for LogManager {
    fn m_type(&self) -> &str {
        "log_manager"
    }
}

enum LogLevel {
    Error = 3,
    Warn = 2,
    Info = 1,
    Debug = 0,
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
        self.write_message(msg, LogLevel::Error)
    }
    pub fn warn(&self, msg: String) {
        self.write_message(msg, LogLevel::Warn)
    }
    pub fn info(&self, msg: String) {
        self.write_message(msg, LogLevel::Info)
    }
    pub fn debug(&self, msg: String) {
        self.write_message(msg, LogLevel::Debug)
    }
    fn write_message(&self, msg: String, level: LogLevel) {
        if level as i8 >= LOG_LEVEL as i8 {
            if let Err(e) = writeln!(&self.file_handle, "{}", msg) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}
