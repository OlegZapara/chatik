use redis::Client;

pub async fn connect_redis() -> Result<Client, redis::RedisError> {
    let redis_url = dotenvy::var("REDIS_URL").expect("`REDIS_URL` not in .env");
    redis::Client::open(redis_url)
}
