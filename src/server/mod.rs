mod request;
mod response;
mod parser;
mod routes;

use std::{io::Write, net::{SocketAddr, TcpListener, TcpStream}, sync::{Arc, Mutex}};
use response::HttpResponse;

use crate::pool::{self, status::Status};
use crate::errors::{self, *};

use self::parser::parse;

pub fn create_server(port: u16) {
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(address);

    // At this point, we couldn't bind the address, so we panic the project
    if let Err(e) = listener {
        log_error(Box::new(e));
        panic!("Unrecoverable error! Check logs.");
    }

    let status = pool::status::Status::new(gettid::gettid());
    let status = Arc::new(Mutex::new(status));

    let pool = pool::threadpool::ThreadPool::build(4, Arc::clone(&status));

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
            let status = Arc::clone(&status);
            pool.execute(move || {
                if let Err(e) = handle_requests(stream.unwrap(), port, status) {
                    log_error(e);
                }
            });
        }
    }
}

fn handle_requests(req: TcpStream, port: u16, status: Arc<Mutex<Status>>) -> Result<(), Box<dyn std::error::Error>> {
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

    let res = routes::handle_route(message, port, status);

    if let Err(_) = res {
        let res = HttpResponse::basic(500);
        return send_response(req, res);
    }

    send_response(req, res.unwrap())
}

fn send_response(mut req: TcpStream, res: HttpResponse)
    -> Result<(), Box<dyn std::error::Error>> {

    Ok(req.write_all(format!("{res}").as_bytes())?)
}
