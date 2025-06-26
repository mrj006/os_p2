pub mod parse;
pub mod server;
pub mod implement;
pub mod pool;
pub mod matrix;
pub mod slaves;

use std::io::Write;
use std::{fs, time::SystemTime};
use chrono::{self, DateTime, Utc};

pub fn log_error(error: Box<dyn std::error::Error>) {
    let kind = "error".to_string();
    let timestamp = get_timestamp();
    write_log(kind.clone(), error.to_string(), timestamp.clone(), "error".to_string());
    write_log(kind, error.to_string(), timestamp, "info".to_string());
}

pub fn log_info(message: String) {
    let kind = "info".to_string();
    let timestamp = get_timestamp();
    write_log(kind, message, timestamp, "info".to_string());
}

fn write_log(kind: String, message: String, timestamp: String, log: String) {
    fs::create_dir_all("./logs/").unwrap();
    let path = "logs/".to_string() + &log + ".log";
    let kind = kind.to_ascii_uppercase();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let message = kind + "\t" + &timestamp + "\t" + &message;
    
    writeln!(file, "{}", message).unwrap();
}

fn get_timestamp() -> String {
    let timestamp: DateTime<Utc> = SystemTime::now().into();
    format!("{}", timestamp.format("%+"))
}
