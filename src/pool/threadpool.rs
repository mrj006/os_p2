use std::{sync::{mpsc, Arc}, thread};
use parking_lot::Mutex;

use crate::errors::pool::PoolError;
use crate::status::status;

pub struct ThreadPool {
    threads: Vec<Thread>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, Box<dyn std::error::Error>> {
        if size < 1 {
            return Err(Box::new(PoolError));
        }

        // We use channels for message passing, implementing an internal queue
        let (sender, receiver) = mpsc::channel();

        // Given the channels are single-consumer, we wrap the receive in
        // Arc<Mutex<>> for multiple access.
        let receiver = Arc::new(Mutex::new(receiver));

        // Defines a amount of threads to have
        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            // We send a pointer clone for each thread to have access to the queue
            threads.push(Thread::new(id, Arc::clone(&receiver))?);
        }

        Ok(ThreadPool {threads, sender: Some(sender)})
    }
    
    // This helper function will pass the required work-to-do to each thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Implementing dropping trait for graceful shutdown, if required
// This makes sure the on-going job is completed before dropping the thread
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for thread in self.threads.drain(..) {
            println!("Shutting down thread {}", thread.id);

            thread.thread.join().unwrap();
        }
    }
}

struct Thread {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Thread {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Thread, Box<dyn std::error::Error>> {
        let builder = thread::Builder::new();
        let thread = builder.spawn(move || {
            let pid = gettid::gettid();
            status::update_thread(pid, false, "".to_string());

            loop {
                // We need to first get a lock on the vector
                let message = receiver.lock();
        
                // We can safely unwrap the guard as we already handled the poison
                let message = message.recv();
    
                // The only error at this point is a sender dropped
                // The only way to receive that error is if the pool is closing
                if let Err(_) = message {
                    println!("Incoming channel closed, shutting down thread {id}");
                    break;
                }
                println!("Thread {id} working...");

                // The double parenthesis means we are calling the boxed function
                message.unwrap()();
                println!("Thread {id} finished!");
                status::update_thread(pid, false, "".to_string());
            }
        })?;
        Ok(Thread { id, thread })
    }
}
