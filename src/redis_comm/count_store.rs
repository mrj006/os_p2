use redis::RedisResult;

use crate::models::count::CountJoinInput;

use super::connection;

pub fn add_count_part_res(file: &str, part: &str, count: usize) -> RedisResult<()> {
    let key = format!("count:{}:{}", file, part);
    connection::add_data_to_redis(key, count.to_string())
}

pub fn get_count_part_res(file: &str) -> Result<CountJoinInput, redis::RedisError> {
    let pattern = format!("count:{}:*", file);
    let values = connection::get_values_from_redis(pattern)?;
    let mut res = vec![];

    for value in values {
        let count = value.parse::<usize>().unwrap();
        res.push(count);
    }

    Ok(CountJoinInput { values: res })
}

pub fn remove_count_part_res(file: &str) -> RedisResult<()> {
    let pattern = format!("count:{}:*", file);
    connection::remove_keys_from_redis(pattern)
}

pub fn add_count_res(file: &str, count: usize) -> RedisResult<()> {
    let key = format!("count:{}", file);
    connection::add_data_to_redis(key, count.to_string())
}

pub fn get_count_res(file: &str) -> RedisResult<String> {
    let key = format!("count:{}", file);
    connection::get_value_from_redis(key)
}
