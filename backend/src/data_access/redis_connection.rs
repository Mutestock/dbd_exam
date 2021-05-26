use crate::utils::environment::load_variables;
use redis::{self, AsyncCommands, Commands};
use std::env;

lazy_static! {
    static ref REDIS_CONNECTION_STRING: String = {
        load_variables();
        let redis_host = match env::var("REDIS_HOST") {
            Ok(v) => v,
            Err(e) => panic!("REDIS_HOST missing environment variable {}", e),
        };
        let redis_port = match env::var("REDIS_PORT") {
            Ok(v) => v,
            Err(e) => panic!("REDIS_PORT missing environment variable {}", e),
        };

        format!("redis://{}:{}", redis_host, redis_port)
    };
}

#[allow(dead_code)]
pub async fn make_async_redis_connection() -> redis::RedisResult<redis::aio::Connection> {
    let client = redis::Client::open(REDIS_CONNECTION_STRING.to_string())?;
    let con = client.get_async_connection().await?;
    Ok(con)
}

#[allow(dead_code)]
pub fn make_redis_connection() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open(REDIS_CONNECTION_STRING.to_string())?;
    let conn = client.get_connection()?;

    Ok(conn)
}

#[allow(dead_code)]
pub fn health_check() -> redis::RedisResult<()> {
    let _: () = make_redis_connection()?.set("health", "ok")?;
    Ok(())
}

#[allow(dead_code)]
pub fn get_health_check() -> redis::RedisResult<String> {
    let health_check = make_redis_connection()?.get("health")?;
    Ok(health_check)
}

#[allow(dead_code)]
pub async fn get_async_health_check() -> redis::RedisResult<String> {
    let health_check = make_async_redis_connection()
        .await?
        .get("health")
        .await?;
    Ok(health_check)
}

#[allow(dead_code)]
pub async fn async_health_check() -> redis::RedisResult<()> {
    make_async_redis_connection()
        .await?
        .set("health_async", "ok")
        .await?;
    Ok(())
}

//  ##############################
//  ########### Tests ############
//  ##############################

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_normal_connection() {
        make_redis_connection().unwrap();
        assert_eq!(2 == 2, true);
    }

    #[test]
    fn test_async_connection() {
        aw!(make_async_redis_connection()).unwrap();
        assert_eq!(2 == 2, true);
    }

    #[test]
    fn test_async_health_check() {
        aw!(async_health_check()).unwrap();
        assert_eq!(2 == 2, true);
    }

    #[test]
    fn test_normal_health_check() {
        health_check().unwrap();
        assert_eq!(2 == 2, true);
    }

    #[test]
    fn test_get_health_check() {
        match health_check() {
            Ok(_) => (),
            Err(e) => println!("Error in async_health_check: {}", e),
        };
        let health: String = match get_health_check() {
            Ok(v) => v,
            Err(e) => panic!("Error in get normal health check: {}", e),
        };
        assert_eq!(health, "ok");
    }

    #[test]
    fn test_get_async_health_check() {
        let conn = aw!(async_health_check());
        match conn {
            Ok(_) => (),
            Err(e) => println!("Error in async_health_check: {}", e),
        };
        let health =  aw!(get_async_health_check());
        let health: String = match health {
            Ok(v) => v,
            Err(e) => panic!("Error in get normal health check: {}", e),
        };
        assert_eq!(health, "ok");
    }
}
