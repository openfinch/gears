use super::log_manager::LogManager;
use super::manager::Manager;

pub struct DisplayManager<'a> {
    started: bool,
    max_x: i8,
    max_y: i8,
    logger: &'a LogManager,
}

impl Manager for DisplayManager<'_> {
    fn m_type(&self) -> &str {
        "display_manager"
    }
}

impl DisplayManager<'_> {
    pub fn new(log_manager: &LogManager) -> DisplayManager {
        DisplayManager {
            started: false,
            max_x: 8,
            max_y: 8,
            logger: &log_manager,
        }
    }
    pub fn startup(&mut self) {
        self.logger
            .info(String::from("DisplayManager.startup(): Current window set"));
        self.logger.info(format!(
            "DisplayManager.startup(): max X is {}, max Y is {}",
            self.max_x, self.max_y
        ));
        self.started = true
    }
    pub fn shutdown(mut self) {
        self.started = false
    }
}
