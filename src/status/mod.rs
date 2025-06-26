pub mod status;

#[cfg(test)]
mod tests {
    use crate::models::status::Status;

    use super::*;

    #[test]
    fn json() {
        status::update_main_pid(1000);
        status::increase_requests_handled();
        status::update_thread(1001, false, "".to_string());
        status::update_thread(1002, true, "test".to_string());

        let json = status::status();
        let res = serde_json::from_str::<Status>(&json).unwrap();

        assert_eq!(res.pid, 1000);
        assert_eq!(res.requests_handled, 1);
        assert_eq!(res.threads.get(&1001).unwrap().busy, false);
        assert_eq!(res.threads.get(&1002).unwrap().command, "test".to_string());
    }
}
