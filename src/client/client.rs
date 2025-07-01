use std::net::SocketAddr;

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::models::request::HttpRequest;

pub async fn send_request(socket: SocketAddr, req: HttpRequest) -> Vec<u8> {
    let message = format!("{}", req);

    let mut stream = TcpStream::connect(socket).await.unwrap();
    let _ = stream.write_all(message.as_bytes()).await;
    
    loop {
        // Wait for the socket to be readable
        let _ = stream.readable().await;

        let mut buf = vec![0; 4096];

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut buf) {
            Ok(0) => return vec![0; 0],
            Ok(n) => {
                buf.truncate(n);
                return buf;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(_) => {
                return vec![0; 0];
            }
        }
    }
}
