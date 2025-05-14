mod request;
mod response;
mod parser;

use std::{collections::HashMap, io::Write, net::{SocketAddr, TcpListener, TcpStream}};

use response::HttpResponse;

use crate::errors::{self, *};

use self::parser::parse;

pub fn create_server(port: u16) {
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(address);

    if let Err(e) = listener {
        log_error(Box::new(e));
        panic!("Unrecoverable error! Check logs.");
    }

    for stream in listener.unwrap().incoming() {
        if stream.is_ok() {
            if let Err(e) = handle_requests(stream.unwrap()) {
                log_error(e);
            }
        }
    }
}

fn handle_requests(req: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let version = "HTTP/1.1".to_string();
    let message = parse(&req);

    // Error handling based on error type
    if let Err(e) = &message {
        if e.is::<errors::parse::ParseUriError>() {
            return send_response(req, version, 400, HashMap::new(), "".to_string());
        }

        if e.is::<errors::implement::ImplementationError>() {
            return send_response(req, version, 501, HashMap::new(), "".to_string());
        } else {
            return send_response(req, version, 500, HashMap::new(), "".to_string());
        }
    }

    //Call method handler and check for errors
    send_response(req, version, 200, HashMap::new(), "".to_string())
}

fn send_response(mut req: TcpStream, version: String, status: u16, headers: HashMap<String, String>, contents: String)
    -> Result<(), Box<dyn std::error::Error>> {
    let reason = (match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        408 => "Request Timeout",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        _ => "Internal Server Error"
    }).to_string();

    let response = HttpResponse {
        version,
        status,
        reason,
        headers,
        contents,
    };

    Ok(req.write_all(format!("{response}").as_bytes())?)
}