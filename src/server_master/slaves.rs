use std::net::SocketAddr;
use std::sync::{Arc, LazyLock};
use indexmap::IndexMap;
use parking_lot::Mutex;
use tokio::select;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::client::client;
use crate::models::request::HttpRequest;
use crate::models::slave::Slave;

static SLAVES_INDEX: LazyLock<Arc<Mutex<IndexMap<SocketAddr, Slave>>>> = LazyLock::new(|| build_index());
static CURRENT_SLAVE: LazyLock<Arc<Mutex<usize>>> = LazyLock::new(|| Arc::new(Mutex::new(0)));
static SLAVES: LazyLock<Arc<Mutex<JoinSet<()>>>> = LazyLock::new(|| build_slaves());

fn build_index() -> Arc<Mutex<IndexMap<SocketAddr, Slave>>> {
    Arc::new(Mutex::new(IndexMap::new()))
}

fn build_slaves() -> Arc<Mutex<JoinSet<()>>> {
    Arc::new(Mutex::new(JoinSet::<()>::new()))
}

async fn monitor_slave(socket: SocketAddr, token: CancellationToken) -> bool {
    let mut ping = HttpRequest::default();
    ping.method = "GET".to_string();
    ping.uri.push("ping".to_string());
    ping.version = "HTTP/1.1".to_string();

    select! {
        _ = client::send_request(socket, ping) => {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            true
        }
        
        // If this branch completes, we need to cancel all slave-related tasks
        // and remove it from the map to avoid further assignments
        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
            token.cancel();            
            let _ = remove(socket);
            false
        }
    }
}

pub async fn add(socket: SocketAddr) {
    // We create the cancellation token for the slave. This token will be used
    // to cancel any ongoing task if the slave is no longer reachable
    let token = CancellationToken::new();
    let token_clone = token.clone();

    // We spawn the thread that will continue to monitor the slave
    let slaves = Arc::clone(&*SLAVES);
    let mut slaves = slaves.lock();
    slaves.spawn(async move {        
        // As long as the slave is responsive, the task would be looping
        while monitor_slave(socket, token_clone.clone()).await {}
    });

    let slave = Slave { socket, token };

    // We need to first get a lock on the vector
    let slaves_index = Arc::clone(&*SLAVES_INDEX);
    let mut slaves_index = slaves_index.lock();
    slaves_index.insert(socket, slave);
}

pub fn remove(socket: SocketAddr) {
    // We need to first get a lock on the vector
    let slaves_index = Arc::clone(&*SLAVES_INDEX);
    let mut slaves_index = slaves_index.lock();
    let _ = slaves_index.shift_remove(&socket);
}

pub fn get_current() -> Option<Slave> {
    // We need to first get a lock on the vector
    let slaves_index = Arc::clone(&*SLAVES_INDEX);
    let slaves_index = slaves_index.lock();
    let length = slaves_index.len();

    // We prevent div by zero error
    if length > 0 {
        let current= Arc::clone(&*CURRENT_SLAVE);
        let mut current = current.lock();
        let previous = *current % length;
        *current = previous + 1;
    
        // We need to de-compose the enum to avoid returning references
        match slaves_index.get_index(previous) {
            Some((_, slave)) => Some(slave.clone()),
            None => None,
        }
    } else {
        None
    }
}

pub fn get_specific(index: usize) -> Option<Slave> {
    // We need to first get a lock on the vector
    let slaves_index = Arc::clone(&*SLAVES_INDEX);
    let slaves_index = slaves_index.lock();

    if !(index < slaves_index.len()) {
        return None;
    }

    let Some((_, slave)) = slaves_index.get_index(index) else {
        return None;
    };

    Some(slave.clone())
}

pub fn get_quantity() -> usize {
    let slaves_index = Arc::clone(&*SLAVES_INDEX);
    let slaves_index = slaves_index.lock();
    slaves_index.len()
}
