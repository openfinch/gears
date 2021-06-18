use std::thread::sleep;
use std::time::{Duration, Instant};

use super::game_object::GameObject;
use super::log_manager::LogManager;
use super::manager::Manager;

enum GameState {
    PreStart = 0,
    Running = 1,
}

pub struct TickTime {
    pub before_ts: Instant,
    pub after_ts: Instant,
}

impl TickTime {
    pub fn elapsed_time(&self) -> Duration {
        self.after_ts - self.before_ts
    }
}

pub struct GameManager<'a> {
    started: bool,
    logger: &'a LogManager,
    state: GameState,
    pub frame_time: Duration,
    pub object_list: Vec<GameObject<'a>>,
}

impl Manager for GameManager<'_> {
    fn m_type(&self) -> &str {
        "game_manager"
    }
}

impl GameManager<'_> {
    pub fn new(log_manager: &LogManager) -> GameManager {
        GameManager {
            started: false,
            logger: &log_manager,
            state: GameState::PreStart,
            frame_time: Duration::new(0, 16666666_u32),
            object_list: vec![],
        }
    }
    pub fn startup(&mut self) {
        self.logger
            .info(String::from("GameManager.startup(): Game started"));
        self.started = true
    }
    pub fn shutdown(mut self) {
        self.started = false
    }
    pub fn run(&mut self) {
        self.state = GameState::Running;

        let mut timer = TickTime {
            before_ts: Instant::now(),
            after_ts: Instant::now(),
        };

        let mut e = 0;
        loop {
            e += 1;
            match self.state {
                GameState::Running => {
                    self.logger.info(String::from("Running loop"));
                    // Get start time
                    timer.before_ts = Instant::now();
                    // Get input // e.g., keyboard/mouse

                    // Update game world state
                    for gameobject in &mut self.object_list {
                        gameobject.update();
                    }

                    // Draw current scene to back buffer
                    // Swap back buffer to current buffer
                    // Measure loop_time // i.e., how long above steps took
                    timer.after_ts = Instant::now();
                    // Sleep for (TARGET_TIME - loop_time)
                    sleep(self.frame_time - timer.elapsed_time());

                    if e == 10 {
                        self.state = GameState::PreStart;
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }
}
