use std::{io::{Read, Write}, net::SocketAddr};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::models::request::HttpRequest;

pub async fn send_async_request(socket: SocketAddr, req: HttpRequest) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let message = format!("{}", req);

    let mut stream = TcpStream::connect(socket).await?;
    let _ = stream.write_all(message.as_bytes()).await;
    
    loop {
        // Wait for the socket to be readable
        let _ = stream.readable().await;

        let mut buf = vec![0; 4096];

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut buf) {
            Ok(0) => return Ok(vec![0; 0]),
            Ok(n) => {
                buf.truncate(n);
                return Ok(buf);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
}

pub fn send_sync_request(socket: SocketAddr, req: HttpRequest) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut stream = std::net::TcpStream::connect(socket)?;
    stream.write_all(format!("{}", req).as_bytes())?;

    let mut buf = vec![0u8; 4096];
    stream.read(&mut buf)?;
    Ok(buf)
}