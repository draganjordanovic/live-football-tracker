use redis::{aio::ConnectionManager, AsyncCommands, Client as RedisClient};
use serde::{de::DeserializeOwned, Serialize};

pub async fn get_connection(
    redis_client: &RedisClient,
) -> Result<ConnectionManager, redis::RedisError> {
    redis_client.get_connection_manager().await
}

pub async fn get_json<T: DeserializeOwned>(
    redis_client: &RedisClient,
    key: &str,
) -> Result<Option<T>, redis::RedisError> {
    let mut conn = get_connection(redis_client).await?;
    let value: Option<String> = conn.get(key).await?;

    match value {
        Some(json) => {
            let parsed = serde_json::from_str::<T>(&json).ok();
            Ok(parsed)
        }
        None => Ok(None),
    }
}

pub async fn set_json<T: Serialize>(
    redis_client: &RedisClient,
    key: &str,
    ttl_seconds: u64,
    value: &T,
) -> Result<(), redis::RedisError> {
    let mut conn = get_connection(redis_client).await?;
    let json = serde_json::to_string(value).unwrap();

    let _: () = conn.set_ex(key, json, ttl_seconds).await?;
    Ok(())
}