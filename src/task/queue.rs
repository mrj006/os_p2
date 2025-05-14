use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use crate::request::HttpRequest;

#[derive(Clone)]
pub struct RequestQueue {
    pub queue: Arc<(Mutex<VecDeque<HttpRequest>>, Condvar)>,
}

impl RequestQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    pub fn enqueue(&self, req: HttpRequest) {
        let (lock, cvar) = &*self.queue;
        let mut guard = lock.lock().unwrap();
        guard.push_back(req);
        cvar.notify_one();
    }

    pub fn dequeue(&self) -> HttpRequest {
        let (lock, cvar) = &*self.queue;
        let mut guard = lock.lock().unwrap();
        while guard.is_empty() {
            guard = cvar.wait(guard).unwrap();
        }
        guard.pop_front().unwrap()
    }
}



    

