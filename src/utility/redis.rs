use redis::{Client, Connection};

pub fn get_redis_connection() -> Connection {
    let client = Client::open("redis://127.0.0.1").expect("Invalid Redis URL");
    client.get_connection().expect("Failed to connect Redis")
}