use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

use crate::errors::{self, *};
use crate::models::request::HttpRequest;
use crate::pool;
use crate::models::response::HttpResponse;
use crate::status::status;

use super::parser::parse;
use super::routes;

pub fn create_server(port: u16, master_socket: String, slave_code: String) {
    report_to_master(port, master_socket, slave_code);
    
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(e) => {
            log_error(Box::new(e));
            panic!("Unrecoverable error! Check logs.");
        },
    };

    match listener.set_nonblocking(true) {
        Ok(_) => (),
        Err(_) => log_error("Unable to set TCP listener to non-blocking mode!".into()),
    }

    status::update_main_pid(gettid::gettid());

    let pool = match pool::threadpool::ThreadPool::build(4) {
        Ok(pool) => pool,
        Err(e) => {
            log_error(e);
            panic!("Unrecoverable error! Check logs.");
        },
    };

    for stream in listener.incoming() {
        // We can ignore stream errors as we wouldn't be able to do anything
        if let Ok(stream) = stream {
            pool.execute(move || {
                let Ok(remote) = stream.peer_addr() else {
                    return;
                };

                if let Err(e) = handle_requests(stream, remote) {
                    log_error(e);
                }
            });
        }
    }
}

fn handle_requests(req: TcpStream, remote: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let version = "HTTP/1.1".to_string();
    let message = parse(&req);

    let message = match message {
        Ok(message) => message,
        // Error handling based on error type
        Err(e) => {
            if e.is::<errors::parse::ParseUriError>() {
            let res = HttpResponse::basic(400);
                return send_response(req, res);
            }
        
            if e.is::<errors::implement::ImplementationError>() {
            let res = HttpResponse::basic(501);
                return send_response(req, res);
            } else {
            let res = HttpResponse::basic(500);
                return send_response(req, res);
            }
        },
    };

    if message.version != version {
        let res = HttpResponse::basic(505);
            return send_response(req, res);
    }

    let res = routes::handle_route(message, remote);
    
    send_response(req, res)
}

fn send_response(mut req: TcpStream, res: HttpResponse)
    -> Result<(), Box<dyn std::error::Error>> {

    Ok(req.write_all(format!("{res}").as_bytes())?)
}

fn report_to_master(port: u16, master_socket: String, slave_code: String) {
    // We get only the first entry as there should be only 1 DNS result 
    let master_socket = master_socket.to_socket_addrs().unwrap().next().unwrap();

    let timeout = std::time::Duration::from_secs(30);
    let Ok(mut stream) = TcpStream::connect_timeout(&master_socket, timeout) else {
        log_error("Master unreachable!".into());
        panic!("Unrecoverable error! Check logs.");
    };

    let mut req = HttpRequest::default();
    req.method = "POST".to_string();
    req.params.insert("port".to_string(), port.to_string());
    req.params.insert("slave_code".to_string(), slave_code);
    req.uri.push("slave".to_string());
    req.version = "HTTP/1.1".to_string();

    if let Err(_) = stream.write_all(format!("{}", req).as_bytes()) {
        log_error("Master unreachable!".into());
        panic!("Unrecoverable error! Check logs.");
    }
}
