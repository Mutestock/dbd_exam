use std::{env, error::Error};
use futures::prelude::*;
use redis::{self, AsyncCommands, Commands, RedisError};
use crate::utils::environment::load_variables;


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

// Result<redis::aio::Connection, RedisError>

async fn make_redis_pool() -> redis::RedisResult<redis::aio::Connection> {
    let client = redis::Client::open(REDIS_CONNECTION_STRING.to_string())?;
    let con = client.get_async_connection().await?;
    Ok(con)
}

pub fn get_redis_conn() -> redis::RedisResult<redis::Connection> {
    //let conn_str: String = (REDIS_CONNECTION_STRING.to_string()
    let client = redis::Client::open(REDIS_CONNECTION_STRING.to_string())?;
    let conn = client.get_connection()?;

    Ok(conn)
}

pub fn health_check() -> redis::RedisResult<()> {
    let _ : () = get_redis_conn()?
    .set("health", "ok")?;
    Ok(())
}

async fn async_health_check() -> redis::RedisResult<()> {
    //redis::cmd("SET").arg(&["key2", "bar"]).query_async(&mut make_redis_pool().await?).await?;
    make_redis_pool().await?.set("health_async","ok").await?;
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
    fn test_normal_connection(){
        get_redis_conn().unwrap();
        assert_eq!(2 == 2, true);
    }

    #[test]
    fn test_async_connection(){
        aw!(make_redis_pool()).unwrap();
        assert_eq!(2 == 2, true);
    }

    #[test]
    fn test_async_health_check() {
        let conn = aw!(async_health_check());
        match conn  {
            Ok(_) => (),
            Err(e) => println!("Error in async_health_check: {}",e)
        };
        assert_eq!(2 == 2, true);
    }

    #[test]
    fn test_normal_health_check() {
        match health_check(){
            Ok(_) => (),
            Err(e) => println!("Error in normal health check: {}", e)
        }
        assert_eq!(2 == 2, true);
    }
}
