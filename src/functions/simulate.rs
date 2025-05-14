use std::{thread, time::Duration};

pub fn simulate(seconds: u64, task: &str) {
    println!("Task {} in process...", task);
    thread::sleep(Duration::from_secs(seconds));
}
