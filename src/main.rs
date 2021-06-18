#![allow(used_unused_variable)]

use display_manager::DisplayManager;
use game_manager::GameManager;
use log_manager::LogManager;
use world_manager::WorldManager;

mod display_manager;
mod game_manager;
mod game_object;
mod log_manager;
mod manager;
mod quaternion;
mod transform;
mod vector;
mod world_manager;

fn main() {
    let mut log_manager: LogManager = LogManager::new();
    log_manager.startup();
    log_manager.error(String::from("Gears.main(): This is an error"));
    log_manager.warn(String::from("Gears.main(): This is a warning"));
    log_manager.info(String::from("Gears.main(): This is an info"));
    log_manager.debug(String::from("Gears.main(): This is a debug"));

    let mut display_manager: DisplayManager = DisplayManager::new(&log_manager);
    display_manager.startup();

    let mut world_manager: WorldManager = WorldManager::new(&log_manager);
    world_manager.startup();

    let mut game_manager: GameManager = GameManager::new(&log_manager);
    game_manager.startup();

    game_manager.run();

    game_manager.shutdown();
    world_manager.shutdown();
    display_manager.shutdown();
    log_manager.shutdown();
}
