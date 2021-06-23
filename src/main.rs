#![allow(unused_variables, dead_code)]

use display_manager::DisplayManager;
use game_manager::GameManager;
use legion::*;
use log_manager::LogManager;
use std::time::Instant;

mod component;
mod display_manager;
mod game_manager;
mod log_manager;
mod manager;
mod quaternion;
mod vector;

fn main() {
    let mut log_manager: LogManager = LogManager::new();
    log_manager.startup();
    log_manager.error(String::from("Gears.main(): This is an error"));
    log_manager.warn(String::from("Gears.main(): This is a warning"));
    log_manager.info(String::from("Gears.main(): This is an info"));
    log_manager.debug(String::from("Gears.main(): This is a debug"));

    let mut display_manager: DisplayManager = DisplayManager::new(&log_manager);
    display_manager.startup();

    let mut world = World::default();

    let mut game_manager: GameManager = GameManager::new(&log_manager, &mut world);
    game_manager.startup();

    let time = Instant::now();
    game_manager.run();
    log_manager.debug(format!("{}ms", time.elapsed().as_millis()));

    game_manager.shutdown();
    display_manager.shutdown();
    log_manager.shutdown();
}
