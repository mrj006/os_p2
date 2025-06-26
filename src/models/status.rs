use std::collections::{hash_map::Entry, HashMap};
use std::time::SystemTime;

use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub start_time: SystemTime,
    pub pid: u64,
    pub requests_handled: u128,
    pub threads: HashMap<u64, Thread>,
    pub run_time: String
}

impl Status {
    pub fn new(pid: u64) -> Status {
        let start_time = SystemTime::now();
        let requests_handled: u128 = 0;
        let threads: HashMap<u64, Thread> = HashMap::new();
        let run_time = String::new();

        Status { start_time, pid, requests_handled, threads, run_time }
    }

    pub fn get_pid(&self) -> u64 {
        self.pid
    }

    pub fn update_pid(&mut self, pid: u64) {
        self.pid = pid;
    }

    pub fn increase_requests_handled(&mut self) {
        self.requests_handled += 1;
    }

    pub fn update_thread(&mut self, pid: u64, busy: bool, command: String) {
        match self.threads.entry(pid) {
            Entry::Occupied(mut occupied_entry) => {
                let thread = occupied_entry.get_mut();
                thread.busy = busy;
                thread.command = command;
            },
            Entry::Vacant(vacant_entry) => {
                let thread = Thread { pid, busy, command };
                vacant_entry.insert(thread);
            },
        }
    }

    pub fn status(&mut self) -> String {
        let start_time: DateTime<Utc> = self.start_time.into();
        let now: DateTime<Utc> = SystemTime::now().into();
        let run_time = now.signed_duration_since(start_time);

        // Given the functions don't check the "carry over points" for the time
        // measure, we need to use the remainder operator to avoid output like
        // 1 minutes 110 seconds
        let mut run_string = String::new();
        let days = run_time.num_days() % 7;
        let hours = run_time.num_hours() % 24;
        let minutes = run_time.num_minutes() % 60;
        let seconds = run_time.num_seconds() % 60;

        if run_time.num_weeks() > 0 {
            run_string += &format!("{} weeks ", run_time.num_weeks());
        }

        if days > 0 {
            run_string += &format!("{} days ", days);
        }

        if hours > 0 {
            run_string += &format!("{} hours ", hours);
        }

        if minutes > 0 {
            run_string += &format!("{} minutes ", minutes);
        }

        if seconds > 0 {
            run_string += &format!("{} seconds ", seconds);
        }

        self.run_time = run_string.trim().to_string();

        serde_json::to_string(self).unwrap()
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    pub pid: u64,
    pub busy: bool,
    pub command: String,
}
