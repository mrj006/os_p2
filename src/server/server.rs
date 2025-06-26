use std::{env, io::Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

use crate::errors::{self, *};
use crate::models::request::HttpRequest;
use crate::pool;
use crate::models::response::HttpResponse;
use crate::status::status;

use super::parser::parse;
use super::routes;

pub fn create_server(port: u16) {
    report_to_master();
    
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(address);

    // At this point, we couldn't bind the address, so we panic the project
    if let Err(e) = listener {
        log_error(Box::new(e));
        panic!("Unrecoverable error! Check logs.");
    }

    status::update_main_pid(gettid::gettid());

    let pool = pool::threadpool::ThreadPool::build(4);

    // At this point, we couldn't start the thread pool, so we panic the project
    if let Err(e) = pool {
        log_error(e);
        panic!("Unrecoverable error! Check logs.");
    }

    let pool = pool.unwrap();

    // WE can safely unwrap the listener as the errors are already handled
    for stream in listener.unwrap().incoming() {
        // We can ignore stream errors as we wouldn't be able to do anything
        if stream.is_ok() {
            pool.execute(move || {
                let stream = stream.unwrap();
                let remote = stream.peer_addr().unwrap();
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

    // Error handling based on error type
    if let Err(e) = &message {
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
    }

    let message = message.unwrap();

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

fn report_to_master() {
    let Ok(master_socket) = env::var("MASTER_SOCKET") else {
        log_error("Unable to read 'MASTER_SOCKET' from env variable!".into());
        panic!("Unrecoverable error! Check logs.");
    };

    // We get only the first entry as there should be only 1 DNS result 
    let master_socket = master_socket.to_socket_addrs().unwrap().next().unwrap();

    let timeout = std::time::Duration::from_secs(30);
    let Ok(mut stream) = TcpStream::connect_timeout(&master_socket, timeout) else {
        log_error("Master unreachable!".into());
        panic!("Unrecoverable error! Check logs.");
    };

    // We can safely unwrap these vars as they were checked before starting the
    // server
    let port = env::var("SERVER_PORT").unwrap();
    let slave_code = env::var("SLAVE_CODE").unwrap();

    let mut req = HttpRequest::default();
    req.method = "POST".to_string();
    req.params.insert("port".to_string(), port);
    req.params.insert("slave_code".to_string(), slave_code);
    req.uri.push("slave".to_string());
    req.version = "HTTP/1.1".to_string();

    if let Err(_) = stream.write_all(format!("{}", req).as_bytes()) {
        log_error("Master unreachable!".into());
        panic!("Unrecoverable error! Check logs.");
    }
}
