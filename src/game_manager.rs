use std::thread::sleep;
use std::time::{Duration, Instant};

use super::log_manager::LogManager;
use super::manager::Manager;
use super::world::World;
use crate::component::{Transform, Velocity};
use crate::quaternion::Quaternion;
use crate::vector::Vector3;
use legion::*;

enum GameState {
    PreStart = 0,
    Running = 1,
}

pub struct Time {
    _delta: Instant,
}

impl Time {
    pub fn delta(&mut self) -> Duration {
        let elapsed = self._delta.elapsed();
        self._delta = Instant::now();
        return elapsed;
    }

    pub fn split(&self) -> Duration {
        self._delta.elapsed()
    }
}

#[system(for_each)]
fn update_positions(transform: &mut Transform, velocity: &Velocity, #[resource] time: &Time) {
    let split = time.split().as_secs_f32();
    transform.position.x += velocity.dx * split;
    transform.position.y += velocity.dy * split;
    transform.position.z += velocity.dz * split;
}

pub struct GameManager<'a> {
    started: bool,
    logger: &'a LogManager,
    state: GameState,
    schedule: Schedule,
    resources: Resources,
    pub target_time: Duration,
    pub world: &'a mut World,
}

impl Manager for GameManager<'_> {
    fn m_type(&self) -> &str {
        "game_manager"
    }
}

impl GameManager<'_> {
    pub fn new<'a>(log_manager: &'a LogManager, world: &'a mut World) -> GameManager<'a> {
        GameManager {
            started: false,
            logger: &log_manager,
            state: GameState::PreStart,
            schedule: Schedule::builder()
                .add_system(update_positions_system())
                .build(),
            resources: Resources::default(),
            target_time: Duration::new(0, 16666666_u32),
            world,
        }
    }
    pub fn startup(&mut self) {
        self.logger
            .info(String::from("GameManager.startup(): Game started"));
        self.started = true;
        let entity: Entity = self.world.push((
            Transform {
                position: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                rotation: Quaternion {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                scale: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Velocity {
                dx: 1.0,
                dy: 1.0,
                dz: 1.0,
            },
        ));

        self.resources.insert(Time {
            _delta: Instant::now(),
        });
    }
    pub fn shutdown(mut self) {
        self.started = false
    }
    pub fn run(&mut self) {
        self.state = GameState::Running;

        let mut e = 0;
        loop {
            e += 1;
            match self.state {
                GameState::Running => {
                    // self.logger.info(String::from("Running loop"));
                    // Reset DeltaTime
                    self.resources.get_mut::<Time>().map(|mut d| d.delta());

                    // Get input // e.g., keyboard/mouse

                    // Update game world state
                    self.schedule.execute(&mut self.world, &mut self.resources);

                    let mut query = <&Transform>::query();

                    // you can then iterate through the components found in the world
                    for position in query.iter(self.world) {
                        println!("{:?}", position);
                    }

                    // Draw current scene to back buffer
                    // self.logger.debug(String::from("Draw current scene to back buffer"));

                    // Swap back buffer to current buffer
                    // self.logger.debug(String::from("Swap back buffer to current buffer"));

                    // Measure loop_time // i.e., how long above steps took
                    let loop_time = self
                        .resources
                        .get::<Time>()
                        .map(|d| d.split())
                        .unwrap_or(Duration::new(0, 0));
                    // Sleep for (target_time - loop_time)
                    sleep(self.target_time - loop_time);

                    if e == 60 {
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
