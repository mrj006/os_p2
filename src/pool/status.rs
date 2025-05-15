use std::{collections::HashMap, time::SystemTime};

use chrono::{self, DateTime, Utc};

pub struct Status {
    start_time: SystemTime,
    pid: u64,
    requests_handled: u128,
    workers: HashMap<u64, Worker>,
}

impl Status {
    pub fn new(pid: u64) -> Status {
        let start_time = SystemTime::now();
        let requests_handled: u128 = 0;
        let workers: HashMap<u64, Worker> = HashMap::new();

        Status { start_time, pid, requests_handled, workers }
    }

    pub fn increase_requests_handled(&mut self) {
        self.requests_handled += 1;
    }

    pub fn add_worker(&mut self, pid: u64) {
        if !self.workers.contains_key(&pid) {
            let worker = Worker {pid, busy: false, command: "".to_string() };
            self.workers.insert(pid, worker);
        }
    }

    pub fn update_worker(&mut self, pid: u64, busy: bool, command: String) {
        let worker = self.workers.get_mut(&pid).unwrap();
        worker.busy = busy;
        worker.command = command;
    }

    pub fn status(&self) -> String {
        let start_time: DateTime<Utc> = self.start_time.into();
        let now: DateTime<Utc> = SystemTime::now().into();
        let run_time = now.signed_duration_since(start_time);

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

        Self::to_json(self,run_string.trim().to_string())
    }

    fn to_json(&self, run_time: String) -> String {
        let mut json = "{".to_string();
        json += &format!(
            "\"Run time\": \"{}\",\"Main PID\": \"{}\",\"Number of handled requests\": \"{}\"",
            run_time,
            self.pid,
            self.requests_handled
        );

        if self.workers.len() > 0 {
            let mut workers = String::new();

            for (_, worker) in &self.workers {
                workers += &worker.to_json();
                workers += ",";
            }

            let workers = workers[0..workers.len() - 1].to_string();

            let workers = format!(
                "\"Active workers\": [{}]",
                workers
            );

            json += ",";
            json += &workers;
        }

        json += "}";
        json
    }
}

struct Worker {
    pid: u64,
    busy: bool,
    command: String,
}

impl Worker {
    fn to_json(&self) -> String {
        let mut json = "{".to_string();

        json += &format!(
            "\"PID\": \"{}\",\"Busy\": \"{}\",\"Command\": \"{}\"",
            self.pid,
            self.busy,
            self.command
        );

        json += "}";
        json
    }
}
