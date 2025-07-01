use std::{net::SocketAddr, sync::Arc};

use parking_lot::Mutex;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Slave {
    pub socket: SocketAddr,
    pub token: CancellationToken,
    pub is_active: Arc<Mutex<bool>>,
}
