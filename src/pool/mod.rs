pub mod threadpool;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{process, thread, time::Duration};

    #[test]
    fn pool() {
        let pool = threadpool::ThreadPool::build(4).unwrap();

        let test = || {
            thread::sleep(Duration::from_secs(1));
            println!("Spawned worker with PID: {}", process::id());
        };

        pool.execute(test);
    }
}