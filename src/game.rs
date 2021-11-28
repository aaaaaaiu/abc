use crate::timer;
use std::{thread, time};

#[derive(PartialEq, Eq)]
enum State {
    Gameplay,
    Paused,
    Quit,
}

impl Default for State {
    fn default() -> State {
        State::Gameplay
    }
}

#[derive(Default)]
pub struct Game {
    state: State,
    fps_fixed: u32,
    fps: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            fps_fixed: 120,
            fps: 60,
            ..Default::default()
        }
    }

    pub fn run(&mut self) {
        let mut elapsed_since_last_fixed_update = time::Duration::ZERO;
        let mut elapsed_since_last_update = time::Duration::ZERO;
        let mut update_timer = timer::Timer::new();
        let mut fixed_update_timer = timer::Timer::new();

        while self.state != State::Quit {
            elapsed_since_last_fixed_update += fixed_update_timer.elapsed_since_last_call();

            let duration_per_fixed_update = time::Duration::from_secs(1) / self.fps_fixed;

            while elapsed_since_last_fixed_update >= duration_per_fixed_update {
                self.fixed_update(duration_per_fixed_update.as_secs_f64());
                elapsed_since_last_fixed_update -= duration_per_fixed_update;
            }

            elapsed_since_last_update += update_timer.elapsed_since_last_call();

            let duration_per_update = time::Duration::from_secs(1) / self.fps;

            if elapsed_since_last_update >= duration_per_update {
                let mut delta = time::Duration::ZERO;
                while elapsed_since_last_update >= duration_per_update {
                    elapsed_since_last_update -= duration_per_update;
                    delta += duration_per_update
                }
                self.update(delta.as_secs_f64());
            }

            elapsed_since_last_fixed_update += fixed_update_timer.elapsed_since_last_call();
            elapsed_since_last_update += update_timer.elapsed_since_last_call();

            let time_until_next_fixed_update =
                if elapsed_since_last_fixed_update < duration_per_fixed_update {
                    duration_per_fixed_update - elapsed_since_last_fixed_update
                } else {
                    time::Duration::ZERO
                };
            let time_until_next_update = if elapsed_since_last_update < duration_per_update {
                duration_per_update - elapsed_since_last_update
            } else {
                time::Duration::ZERO
            };

            let wait_time = time_until_next_fixed_update.min(time_until_next_update);
            if wait_time == time::Duration::ZERO {
                continue;
            }

            let next_loop_start_time = time::Instant::now() + wait_time;

            let sleep_time = wait_time - time::Duration::from_millis(1);
            if sleep_time > time::Duration::ZERO {
                thread::sleep(sleep_time);
            }

            while time::Instant::now() < next_loop_start_time {
                std::thread::yield_now();
            }
        }
    }
    fn fixed_update(&mut self, delta_secs: f64) {
        //println!("fixed_update");
    }
    fn update(&mut self, delta_secs: f64) {
        //println!("update");
    }
}
