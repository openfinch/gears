use super::log_manager::LogManager;
use super::manager::Manager;

pub struct WorldManager<'a> {
    started: bool,
    logger: &'a LogManager,
}

impl Manager for WorldManager<'_> {
    fn m_type(&self) -> &str {
        "world_manager"
    }
}

impl WorldManager<'_> {
    pub fn new(log_manager: &LogManager) -> WorldManager {
        WorldManager {
            started: false,
            logger: &log_manager,
        }
    }
    pub fn startup(&mut self) {
        self.logger
            .info(String::from("WorldManager.startup(): World started"));
        self.started = true
    }
    pub fn shutdown(mut self) {
        self.started = false
    }
}
