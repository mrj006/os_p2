use std::env;
use redis::{Client, Commands, Connection, RedisResult};

fn connect_to_redis() -> RedisResult<Connection> {
    // We can safely unwrap this call as we checked for this env var before
    // starting the server
    let redis_uri = env::var("REDIS_URI").unwrap();
    let client = Client::open(redis_uri)?;
    client.get_connection()
}

pub fn add_data_to_redis(key: String, value: String) -> RedisResult<()> {
    // We attempt to connect to the DB
    let mut connection = connect_to_redis()?;
    let redis = &mut connection;

    redis.set(key, value)
}

pub fn get_value_from_redis(key: String) -> Result<String, redis::RedisError> {
    // We attempt to connect to the DB
    let mut connection = connect_to_redis()?;
    let redis = &mut connection;
    
    let value: String = redis.get(key)?;
    Ok(value)
}

pub fn get_values_from_redis(pattern: String) -> Result<Vec<String>, redis::RedisError> {
    // We attempt to connect to the DB
    let mut connection = connect_to_redis()?;
    let redis = &mut connection;

    let keys: Vec<String> = redis.keys(pattern)?;
    let mut values: Vec<String> = vec![];

    for key in keys {
        values.push(redis.get(key)?);
    }
    
    Ok(values)
}

// Removing will result in race conditions due to having multiple individual 
// containers, should be avoided
pub fn remove_key_from_redis(key: String) -> RedisResult<()> {
    // We attempt to connect to the DB
    let mut connection = connect_to_redis()?;
    let redis = &mut connection;

    redis.del(key)
}

// Removing will result in race conditions due to having multiple individual 
// containers, should be avoided
pub fn remove_keys_from_redis(pattern: String) -> RedisResult<()> {
    // We attempt to connect to the DB
    let mut connection = connect_to_redis()?;
    let redis = &mut connection;

    let keys: Vec<String> = redis.keys(pattern)?;
    
    for key in keys {
        let _ = redis.del::<_, ()>(key)?;
    }

    Ok(())
}
