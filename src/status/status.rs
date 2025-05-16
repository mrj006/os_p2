use std::{collections::{hash_map::Entry, HashMap}, sync::{Arc, LazyLock, Mutex}, time::SystemTime};

use chrono::{self, DateTime, Utc};

static STATUS: LazyLock<Arc<Mutex<Status>>> = LazyLock::new(|| build());

pub struct Status {
    start_time: SystemTime,
    pid: u64,
    requests_handled: u128,
    workers: HashMap<u64, Worker>,
}

pub fn new(pid: u64) -> Arc<Mutex<Status>> {
    let status_mutex = Arc::clone(&*STATUS);
    
    // We update the main pid before returning the "new" instance only if
    // the pid = 0, meaning this is the first time the function is run
    update_pid(Arc::clone(&status_mutex), pid);

    status_mutex
}

fn build() -> Arc<Mutex<Status>> {
    Arc::new(Mutex::new(Status::new(0)))
}

fn update_pid(status: Arc<Mutex<Status>>, pid: u64) {
    let mut status = status.lock();

    // If a thread panic'd, we could 'unwrap' the error and re-acquire
    // the lock
    if let Err(error) = status {
        let data = error.into_inner();
        status = Ok(data);
    }

    // We can safely unwrap the guard as we already handled the poison
    let mut status = status.unwrap();

    if status.get_pid() == 0 {
        status.update_pid(pid);
    }
}

impl Status {
    fn new(pid: u64) -> Status {
        let start_time = SystemTime::now();
        let requests_handled: u128 = 0;
        let workers: HashMap<u64, Worker> = HashMap::new();

        Status { start_time, pid, requests_handled, workers }
    }

    fn get_pid(&self) -> u64 {
        self.pid
    }

    fn update_pid(&mut self, pid: u64) {
        self.pid = pid;
    }

    pub fn increase_requests_handled(&mut self) {
        self.requests_handled += 1;
    }

    pub fn update_worker(&mut self, pid: u64, busy: bool, command: String) {
        match self.workers.entry(pid) {
            Entry::Occupied(mut occupied_entry) => {
                let worker = occupied_entry.get_mut();
                worker.busy = busy;
                worker.command = command;
            },
            Entry::Vacant(vacant_entry) => {
                let worker = Worker { pid, busy, command };
                vacant_entry.insert(worker);
            },
        }
    }

    pub fn status(&self) -> String {
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

            // Given the last item will have a comma, and it's prohibited in a
            // JSON, we slice the string to trim it
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
