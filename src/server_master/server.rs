use std::{net::SocketAddr, sync::atomic::{AtomicUsize, Ordering}};
use tokio::{net::{TcpListener, TcpStream}};
use tokio::io::AsyncWriteExt;
use tokio::runtime::Builder;

use crate::{errors::{self, *}, models::response::Response};
use crate::models::response::HttpResponse;

use super::parser::parse;
use super::routes;

pub fn create_server(port: u16) {
    // We create an asynchronous runtime, replacing the synchronous thread pool
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("Master-thread-{}", id)
        })
        .build();

    let runtime = match runtime {
        Ok(rt) => rt,
        Err(e) => {
            log_error(Box::new(e));
            panic!("Unrecoverable error! Check logs.");
        },
    };

    // We initiate the async execution
    runtime.block_on(async {
        let address = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = TcpListener::bind(address).await;
    
        let listener = match listener {
            Ok(listener) => listener,
            Err(e) => {
                log_error(Box::new(e));
                panic!("Unrecoverable error! Check logs.");
            },
        };
    
        loop {
            let stream = listener.accept().await;

            //TODO we can check socket address for slaves to match master's IP and report IP on log
            if let Ok((stream, remote)) = stream {
                tokio::spawn(async move {
                    handle_requests(stream, remote).await;
                });
            }
        }
    });
}

async fn handle_requests(req: TcpStream, remote: SocketAddr) {
    // We duplicate the stream as tokio doesn't have a clone fn or copy trait
    let req_std = req.into_std().unwrap();
    let parsing_req = TcpStream::from_std(req_std.try_clone().unwrap()).unwrap();
    let req = TcpStream::from_std(req_std).unwrap();

    let version = "HTTP/1.1".to_string();
    let message = parse(parsing_req).await;

    // Error handling based on error type
    if let Err(e) = &message {
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
    }

    let message = message.unwrap();

    if message.version != version {
        let res = HttpResponse::basic(505);
            return send_response(req, res).await;
    }

    match routes::handle_route(message, remote).await {
        Response::HTTP(res) => send_response(req, res).await,
        Response::Buffer(buffer) => send_buffer(req, buffer).await,
    };
}

async fn send_response(mut req: TcpStream, res: HttpResponse) {
    req.write_all(format!("{res}").as_bytes()).await.unwrap();
}

async fn send_buffer(mut req: TcpStream, buffer: Vec<u8>) {
    req.write_all(&buffer).await.unwrap();
}