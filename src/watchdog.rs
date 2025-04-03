use std::fmt;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const SLEEP_DURATION: Duration = Duration::from_millis(100);

struct Watchdog {
    timeout: Duration,
    running: Arc<Mutex<bool>>,
    last_kicked: Arc<Mutex<Instant>>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>
}

#[derive(Debug)]
enum WatchdogError {
    AlreadyStarted,
    NotStarted,
}

impl fmt::Display for WatchdogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchdogError::AlreadyStarted => write!(f, "Watchdog already running"),
            WatchdogError::NotStarted => write!(f, "Watchdog not running"),
        }
    }
}

impl Error for WatchdogError {}

impl Watchdog {
    /// Creates a new watchdog
    pub fn new(timeout: Duration) -> Self {
        Watchdog {
            timeout,
            running: Arc::new(Mutex::new(false)),
            last_kicked: Arc::new(Mutex::new(Instant::now())),
            handle: Arc::new(Mutex::new(Option::None))
        }
    }

    /// Starts the watchdog with a callback function.
    /// If the watchdog fails to be periodically kicked, it will trigger the callback and stop.
    /// If the watchdog is already running, an error is returned.
    pub fn start(&mut self, callback: impl Fn() + Send + 'static) -> Result<(), WatchdogError> {
        let mut running = self.running.lock().unwrap();
        let mut handle = self.handle.lock().unwrap();

        // If the watchdog is already running, return an error.
        // Otherwise, wait for the previous thread to end before starting a new one.
        if *running {
            return Result::Err(WatchdogError::AlreadyStarted)
        }
        if let Some(handle) = handle.take() {
            handle.join().unwrap();
        }

        let to = self.timeout;
        let r = Arc::clone(&self.running);
        let lk = Arc::clone(&self.last_kicked);

        let spawn_handle = thread::spawn(move ||{
            while *r.lock().unwrap() {
                let time_since_last_kick = (*lk.lock().unwrap()).elapsed();
                if time_since_last_kick > to {
                    callback();
                    break;
                }
                thread::sleep(SLEEP_DURATION);
            }
        });

        *running = true;
        *handle = Option::Some(spawn_handle);

        Result::Ok(())
    }

    /// Stops the watchdog.
    /// If the watchdog is already stopped, returns an error.
    pub fn stop(&mut self) -> Result<(), WatchdogError>{
        let mut running = self.running.lock().unwrap();
        if *running == false {
            Result::Err(WatchdogError::NotStarted)
        } else {
            *running = false;
            Result::Ok(())
        }
    }

    /// Kicks the watchdog.
    /// If the watchdog is not running, returns an error.
    pub fn kick(&mut self) -> Result<(), WatchdogError>{
        if *self.running.lock().unwrap() == false {
            Result::Err(WatchdogError::NotStarted)
        } else {
            *self.last_kicked.lock().unwrap() = Instant::now();
            Result::Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_can_set_timeout() {
        let timeout = Duration::from_millis(200);
        let wd = Watchdog::new(timeout);
        assert_eq!(timeout, wd.timeout);
    }

    // TODO: Add more tests to cover API
    // TODO: Make DURATION_SLEEP shorter for tests
}
