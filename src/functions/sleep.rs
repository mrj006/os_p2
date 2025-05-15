use std::{thread, time::Duration};

pub fn sleep(seconds: u64) -> String {
    thread::sleep(Duration::from_secs(seconds));
    format!("Thread slept for {} seconds.", seconds)
}
