use std::time;

struct Timer {
    prev_time: time::Instant,
}

impl Timer {
    fn new() -> Self {
        Self {
            prev_time: time::Instant::now(),
        }
    }
    fn elapsed_since_last_call(&mut self) -> time::Duration {
        let elapsed = self.prev_time.elapsed();
        println!("{:?}", elapsed);
        self.prev_time = time::Instant::now();
        elapsed
    }
}

fn main() {
    let mut timer = Timer::new();
    loop {
        let a = timer.elapsed_since_last_call();
    }
}
