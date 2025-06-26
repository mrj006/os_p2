use std::sync::{Arc, LazyLock};
use parking_lot::Mutex;

use crate::models::status::Status;

static STATUS: LazyLock<Arc<Mutex<Status>>> = LazyLock::new(|| build());

fn build() -> Arc<Mutex<Status>> {
    Arc::new(Mutex::new(Status::new(0)))
}

pub fn update_main_pid(pid: u64) {
    // We need to first get a lock on the vector
    let status = Arc::clone(&*STATUS);
    let mut status = status.lock();

    if status.get_pid() == 0 {
        status.update_pid(pid);
    }
}

pub fn update_thread(pid: u64, busy: bool, command: String) {
    // We need to first get a lock on the vector
    let status = Arc::clone(&*STATUS);
    let mut status = status.lock();

    status.update_thread(pid, busy, command);
}

pub fn increase_requests_handled() {
    // We need to first get a lock on the vector
    let status = Arc::clone(&*STATUS);
    let mut status = status.lock();

    status.increase_requests_handled();
}

pub fn status() -> String {
    // We need to first get a lock on the vector
    let status = Arc::clone(&*STATUS);
    let mut status = status.lock();

    status.status()
}
