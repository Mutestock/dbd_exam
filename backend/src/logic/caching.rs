use crate::data_access::redis_connection::make_async_redis_connection;
use redis::{AsyncCommands, RedisError};


pub const CACHE_LOCATION_FORMAT: &str = "cached_location_";
pub const CACHE_PERSON_FORMAT: &str = "cached_person_";
pub const CACHE_UNIVERSITY_FORMAT: &str = "cached_university_";

pub async fn check(key: &str) -> redis::RedisResult<String> {
    let mut conn = make_async_redis_connection()
        .await
        .expect("Could not create an async redis connection");

    conn.get(key).await
}

pub async fn set(key: &str, value: &str) -> Result<(),RedisError>{
    let mut conn = make_async_redis_connection()
        .await
        .expect("Could not create an async redis connection");

    //conn.set(key, value)
    //    .await
    //    .expect(&format!("Could not set {} to {}", key, value));
    conn.set(key, value)
        .await?;
    Ok(())
}


pub async fn remove(key: &str) -> Result<(),RedisError>{
    let mut conn = make_async_redis_connection()
        .await
        .expect("Could not create an async redis connection"); 

    let _ : () = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
    Ok(())
}