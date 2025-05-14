use std::{thread, time::Duration};

pub fn sleep(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}
