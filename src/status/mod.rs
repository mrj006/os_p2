pub mod status;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json() {
        status::update_main_pid(1000);
        status::increase_requests_handled();
        status::update_worker(1001, false, "".to_string());
        status::update_worker(1002, true, "test".to_string());

        let json = status::status();
        let res: Vec<&str> = json.split(&['\"'][..]).collect();

        assert_eq!(39, res.len());
    }
}
