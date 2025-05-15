pub mod status;
pub mod threadpool;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::{Arc, Mutex}, thread, time::Duration};

    #[test]
    fn pool() {
        let status = status::Status::new(gettid::gettid());
        let status = Arc::new(Mutex::new(status));

        let pool = threadpool::ThreadPool::build(4, status).unwrap();

        let test = || {
            thread::sleep(Duration::from_secs(1));
        };

        for _ in 0..10 {
            pool.execute(test);
        }

    }

    #[test]
    fn json() {
        let mut status = status::Status::new(1000);
        status.increase_requests_handled();
        status.add_worker(1001);
        status.add_worker(1002);
        status.update_worker(1002, true, "test".to_string());

        let json = status.status();

        println!("{}", json);
    }
}