pub mod parse;
pub mod server;
pub mod implement;
pub mod pool;
pub mod matrix;

use std::io::Write;
use std::{fs, time::SystemTime};
use chrono::{self, DateTime, Utc};

pub fn log_error(error: Box<dyn std::error::Error>) {
    write_log(error.to_string(), "error".to_string());
}

fn write_log(message: String, log: String) {
    fs::create_dir_all("./logs/").unwrap();
    let path = "logs/".to_string() + &log + ".log";
    let log = log.to_ascii_uppercase();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let timestamp: DateTime<Utc> = SystemTime::now().into();
    let timestamp = format!("{}", timestamp.format("%+"));
    let message = log + "\t" + &timestamp + "\t" + &message;
    
    writeln!(file, "{}", message).unwrap();

}
