use std::net::SocketAddr;

use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Slave {
    pub socket: SocketAddr,
    pub token: CancellationToken,
}
