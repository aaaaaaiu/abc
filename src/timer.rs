use std::time;

pub struct Timer {
    prev_time: time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            prev_time: time::Instant::now(),
        }
    }
    pub fn elapsed_since_last_call(&mut self) -> time::Duration {
        let elapsed = self.prev_time.elapsed();
        println!("{:?}", elapsed);
        self.prev_time = time::Instant::now();
        elapsed
    }
}
