pub mod threadpool;

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    fn pool() {
        let pool = threadpool::ThreadPool::build(4).unwrap();

        let test = || {
            thread::sleep(Duration::from_secs(1));
        };

        for _ in 0..10 {
            pool.execute(test);
        }

        drop(pool);

        assert!(true);
    }
}
