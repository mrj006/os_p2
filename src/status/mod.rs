pub mod status;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json() {
        let status = status::new(1000);

        let mut status = status.lock();

        // If a thread panic'd, we could 'unwrap' the error and re-acquire
        // the lock
        if let Err(error) = status {
            let data = error.into_inner();
            status = Ok(data);
        }

        // We can safely unwrap the guard as we already handled the poison
        let mut status = status.unwrap();
        
        status.increase_requests_handled();
        status.update_worker(1001, false, "".to_string());
        status.update_worker(1002, true, "test".to_string());

        let json = status.status();
        let res: Vec<&str> = json.split(&['\"'][..]).collect();

        assert_eq!(39, res.len());
    }
}
