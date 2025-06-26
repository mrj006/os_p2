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
    let port = match env::var("SERVER_PORT") {
        Ok(value) => value.parse::<u16>().unwrap(),
        Err(_) => {
            log_error(format!("Unable to read port from env vars, defaulting to 7878!").into());
            unsafe { env::set_var("SERVER_PORT", "7878") };
            7878
        }
    };

    let role = env::var("SERVER_ROLE");
    
    if let Err(_) = role {
        log_info(format!("Starting server as slave on port {}", port));
        server::server::create_server(port);
    }

    if role.unwrap() == "MASTER" {
        log_info(format!("Starting server as master on port {}", port));
        server_master::server::create_server(port);
    } else {
        log_info(format!("Starting server as slave on port {}", port));
        server::server::create_server(port);
    }
}
