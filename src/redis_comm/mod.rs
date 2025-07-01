pub mod connection;
pub mod count_store;
pub mod matrix_store;

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    fn set_variables() {
        let redis_uri = "redis://127.0.0.1:6379".to_string();
        // Not unsafe, as it would be set to the same value on all tests
        unsafe { env::set_var("REDIS_URI".to_string(), &redis_uri) };
    }

    #[test]
    fn add_data() {
        set_variables();
        connection::add_data_to_redis("test".to_string(), "value".to_string()).unwrap();
    }

    #[test]
    fn get_data() {
        set_variables();
        let key = format!("get_data_test");
        let value = format!("test");

        connection::add_data_to_redis(key.clone(), value.clone()).unwrap();
        let res = connection::get_value_from_redis(key).unwrap();

        assert_eq!(value, res);
    }

    #[test]
    fn remove_data() {
        set_variables();
        let key = format!("remove_data_test");
        connection::add_data_to_redis(key.clone(), "test".to_string()).unwrap();
        connection::remove_key_from_redis(key).unwrap();
    }

    #[test]
    fn remove_data_multiple() {
        set_variables();
        let key = "remove_multiple";

        for i in 0..5 {
            let key = format!("{}:{}", key, i);
            connection::add_data_to_redis(key, "test".to_string()).unwrap();
        }

        let pattern = format!("{}:*", key);
        connection::remove_keys_from_redis(pattern).unwrap();
    }

    #[test]
    fn countstore() {
        set_variables();
        let key = "countstore_test";
        let parts = 10;

        for i in 0..parts {
            count_store::add_count_part_res(key, &i.to_string(), i).unwrap();
        }

        let res = count_store::get_count_part_res(key).unwrap();
        let res: usize = res.values.iter().sum();
        let expected: usize = (0..parts).sum();
        assert_eq!(res, expected);
    }
}