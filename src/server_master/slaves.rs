use std::net::SocketAddr;
use std::sync::{Arc, LazyLock};
use indexmap::IndexMap;
use parking_lot::Mutex;
use tokio_util::sync::CancellationToken;

static SLAVES: LazyLock<Arc<Mutex<IndexMap<SocketAddr, CancellationToken>>>> = LazyLock::new(|| build());
static CURRENT_SLAVE: LazyLock<Arc<Mutex<usize>>> = LazyLock::new(|| Arc::new(Mutex::new(0)));

fn build() -> Arc<Mutex<IndexMap<SocketAddr, CancellationToken>>> {
    Arc::new(Mutex::new(IndexMap::new()))
}

pub fn add(socket: SocketAddr) {
    // We create the cancellation token for the slave. This token will be used
    // to cancel any ongoing task if the slave is no longer reachable
    let token = CancellationToken::new();

    // We need to first get a lock on the vector
    let slaves = Arc::clone(&*SLAVES);
    let mut slaves = slaves.lock();

    slaves.insert(socket, token);
}

pub fn remove(socket: SocketAddr) -> Option<CancellationToken> {
    // We need to first get a lock on the vector
    let slaves = Arc::clone(&*SLAVES);
    let mut slaves = slaves.lock();

    slaves.shift_remove(&socket)
}

pub fn get_current() -> Option<(SocketAddr, CancellationToken)> {
    // We need to first get a lock on the vector
    let slaves = Arc::clone(&*SLAVES);
    let slaves = slaves.lock();
    let length = slaves.len();

    // We prevent div by zero error
    if length > 0 {
        let current= Arc::clone(&*CURRENT_SLAVE);
        let mut current = current.lock();
        let previous = *current % length;
        *current = previous + 1;
    
        // We need to de-compose the enum to avoid returning references
        match slaves.get_index(previous) {
            Some((socket, token)) => Some((*socket, token.clone())),
            None => None,
        }
    } else {
        None
    }
}

pub fn get_specific(index: usize) -> Option<(SocketAddr, CancellationToken)> {
    // We need to first get a lock on the vector
    let slaves = Arc::clone(&*SLAVES);
    let slaves = slaves.lock();

    if !(index < slaves.len()) {
        return None;
    }

    let (socket, token) = slaves.get_index(index).unwrap();

    Some((*socket, token.clone()))
}

pub fn get_quantity() -> usize {
    let slaves = Arc::clone(&*SLAVES);
    let slaves = slaves.lock();
    slaves.len()
}
