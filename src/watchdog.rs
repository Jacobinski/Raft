use std::time::{Duration, Instant};

struct Watchdog {
    timeout: Duration,
    running: bool,
    last_kicked: Instant,
}

impl Watchdog {
    fn new(timeout: Duration) -> Self {
        Watchdog {
            timeout,
            running: false,
            last_kicked: Instant::now(),
        }
    }

    // fn run(&mut self, callback: impl Fn()) {
    //     self.running = true;
    //     self.last_kicked = Instant::now();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let timeout = Duration::from_millis(200);
        let wd = Watchdog::new(timeout);
        assert_eq!(timeout, wd.timeout);
    }
}
