use std::{sync::{mpsc, Arc, Mutex}, thread};
use crate::errors::pool::PoolError;

use super::status::Status;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn build(size: usize, status: Arc<Mutex<Status>>) -> Result<ThreadPool, Box<dyn std::error::Error>> {
        if size < 1 {
            return Err(Box::new(PoolError));
        }

        // We use channels for message passing, implementing an internal queue
        let (sender, receiver) = mpsc::channel();

        // Given the channels are single-consumer, we wrap the receive in
        // Arc<Mutex<>> for multiple access.
        let receiver = Arc::new(Mutex::new(receiver));

        // Defines a amount of workers to have
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // We send a pointer clone for each worker to have access to the queue
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&status))?);
        }

        Ok(ThreadPool {workers, sender: Some(sender)})
    }
    
    // This helper function will pass the required work-to-do to each worker
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Implementing dropping trait for graceful shutdown, if required
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, status_clone: Arc<Mutex<Status>>) -> Result<Worker, Box<dyn std::error::Error>> {
        let builder = thread::Builder::new();
        let thread = builder.spawn(move || {
            // We add the thread to the status struct
            let pid = gettid::gettid();
            Self::status_add_worker(Arc::clone(&status_clone), pid);

            loop {
                let mut message = receiver.lock();
    
                // If a thread panic'd, we could 'unwrap' the error and re-acquire
                // the lock
                if let Err(error) = message {
                    let data = error.into_inner();
                    message = Ok(data);
                }
    
                // We can safely unwrap the guard as we already handled the poison
                let message = message.unwrap().recv();
    
                // The only error at this point is a sender dropped
                // The only way to receive that error is if the pool is closing
                if let Err(_) = message {
                    println!("Incoming channel closed, shutting down worker {id}");
                    break;
                }
                println!("Worker {id} working...");

                // We need access again to the status struct to update the
                // thread status
                Self::status_update_worker(Arc::clone(&status_clone), pid, true, "parsing".to_string());

                // The double parenthesis means we are callin the boxed function
                message.unwrap()();
                println!("Worker {id} finished!");
                Self::status_update_worker(Arc::clone(&status_clone), pid, false, "".to_string());
            }
        })?;
        Ok(Worker { id, thread })
    }

    fn status_add_worker(status_clone: Arc<Mutex<Status>>, pid: u64) {
        let mut status = status_clone.lock();

        // If a thread panic'd, we could 'unwrap' the error and re-acquire
        // the lock
        if let Err(error) = status {
            let data = error.into_inner();
            status = Ok(data);
        }

        // We can safely unwrap the guard as we already handled the poison
        let mut status = status.unwrap();

        status.add_worker(pid);
        drop(status);
    }

    fn status_update_worker(status_clone: Arc<Mutex<Status>>, pid: u64, busy: bool, command: String) {
        let mut status = status_clone.lock();

        // If a thread panic'd, we could 'unwrap' the error and re-acquire
        // the lock
        if let Err(error) = status {
            let data = error.into_inner();
            status = Ok(data);
        }

        // We can safely unwrap the guard as we already handled the poison
        let mut status = status.unwrap();
        status.update_worker(pid, busy, command);

        if busy {
            status.increase_requests_handled();
        }

        drop(status);
    }
}
