use std::{thread, time::Duration};

pub fn simulate(seconds: u64, task: &str) -> String {
    thread::sleep(Duration::from_secs(seconds));
    format!("Task {} was processed in {} seconds.", task, seconds)
}
