use redis::{Client, Connection, RedisResult};

pub fn conectar_redis() -> RedisResult<Connection> {
    let client = Client::open("redis://127.0.0.1/")?;
    client.get_connection()
}
