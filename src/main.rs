use std::env;

use crate::errors::{log_error, log_info};

mod distributed;
mod errors;
mod functions;
mod models;
mod pool;
mod server;
mod server_master;
mod status;
mod redis_comm;

fn main() {
    let port = check_env_var("SERVER_PORT", false);
    
    // We set a default port number if case of error reading or parsing
    let port = match port.parse::<u16>() {
        Ok(port) => port,
        Err(_) => {
            log_error(format!("Unable to read 'SERVER_PORT' from env vars, defaulting to 7878!").into());
            unsafe { env::set_var("SERVER_PORT", "7878") };
            7878
        },
    };

    // We panic if any of these vars are missing or cant be read
    let _ = check_env_var("REDIS_URI", true);
    let slave_code = check_env_var("SLAVE_CODE", true);
    let role = check_env_var("SERVER_ROLE", false);
    
    // We default to slave role in case of error reading or matching the value
    if role == "MASTER" {
        log_info(format!("Starting server as master on port {}", port));
        server_master::server::create_server(port);
    } else {
        let master_socket = check_env_var("MASTER_SOCKET", true);
        log_info(format!("Starting server as slave on port {}", port));
        server::server::create_server(port, master_socket, slave_code);
    }
}

fn check_env_var(key: &str, should_panic: bool) -> String {
    let Ok(value) = env::var(key) else {
        if should_panic {
            log_error(format!("Unable to read '{}' from env vars!", key).into());
            panic!("Unrecoverable error! Check logs.");
        } else {
            return "".to_string();
        }
    };

    value
}
