use std::net::{SocketAddr, ToSocketAddrs};
use tokio::{net::{TcpListener, TcpStream}};
use tokio::io::AsyncWriteExt;

use crate::client::client;
use crate::errors::{self, *};
use crate::models::request::HttpRequest;
use crate::models::response::{HttpResponse, Response};
use crate::status::status;
use crate::server_master;
use crate::server_slave;

use super::parser::parse;

pub async fn create_server(port: u16, role: String) {
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = match TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(e) => {
            log_error(Box::new(e));
            panic!("Unrecoverable error! Check logs.");
        },
    };

    loop {
        let stream = listener.accept().await;
        let role = role.clone();

        if let Ok((stream, remote)) = stream {
            tokio::spawn(async move {
                let _ = handle_requests(stream, remote, role).await;
            });
        }
    }
}

async fn handle_requests(req: TcpStream, remote: SocketAddr, role: String) -> Result<(), Box<dyn std::error::Error>> {
    // We duplicate the stream as tokio doesn't have a clone fn or copy trait
    let req_std = req.into_std()?;
    let parsing_req = TcpStream::from_std(req_std.try_clone()?)?;
    let req = TcpStream::from_std(req_std)?;

    let version = "HTTP/1.1".to_string();
    let message = parse(parsing_req).await;

    let message = match message {
        Ok(message) => message,
        // Error handling based on error type
        Err(e) => {
            if e.is::<errors::parse::ParseUriError>() {
            let res = HttpResponse::basic(400);
                return send_response(req, res).await;
            }
        
            if e.is::<errors::implement::ImplementationError>() {
            let res = HttpResponse::basic(501);
                return send_response(req, res).await;
            } else {
            let res = HttpResponse::basic(500);
                return send_response(req, res).await;
            }
        },
    };
    
    if message.version != version {
        let res = HttpResponse::basic(505);
            return send_response(req, res).await;
    }

    if role == "MASTER" {
        match server_master::routes::handle_route(message, remote).await {
            Response::HTTP(res) => send_response(req, res).await,
            Response::Buffer(buffer) => send_buffer(req, buffer).await,
        }
    } else {
        let res = server_slave::routes::handle_route(message, remote);
        
        send_response(req, res).await
    }

}

async fn send_response(mut req: TcpStream, res: HttpResponse) -> Result<(), Box<dyn std::error::Error>> {
    Ok(req.write_all(format!("{}", res).as_bytes()).await?)
}

async fn send_buffer(mut req: TcpStream, buffer: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(req.write_all(&buffer).await?)
}

pub fn report_to_master(port: u16, master_socket: String, slave_code: String) {
    // We call this function outside to get the slave's main PID
    status::update_main_pid(gettid::gettid());

    // We use a regular thread to have dedicated to the master's heartbeat, so
    // we make sure the slave can report to the master, even if all runtime's
    // threads are busy doing actual work
    std::thread::spawn(move || {
        // AS the master could take some time to initialize, we wait some time once
        std::thread::sleep(std::time::Duration::from_secs(10));
        loop {
            // We get only the first entry as there should be only 1 DNS result 
            let master_socket = master_socket.to_socket_addrs().unwrap().next().unwrap();
        
            let mut req = HttpRequest::default();
            req.method = "POST".to_string();
            req.params.insert("port".to_string(), port.to_string());
            req.params.insert("slave_code".to_string(), slave_code.clone());
            req.uri.push("slave".to_string());
            req.version = "HTTP/1.1".to_string();
        
            let master_res = match client::send_sync_request(master_socket, req) {
                Ok(res) => res,
                Err(_) => {
                    log_error("Master unreachable!".into());
                    panic!("Unrecoverable error! Check logs.");
                },
            };
        
            if master_res.len() == 0 {
                log_error("Master unreachable!".into());
                panic!("Unrecoverable error! Check logs.");
            }
        
            std::thread::sleep(std::time::Duration::from_secs(4));
        }
    });
}
