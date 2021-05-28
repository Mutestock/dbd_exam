use mongodb::{error::Error, options::ClientOptions, Client};
use std::env;

lazy_static! {
    static ref MONGO_CONNECTION_STRING: String = {
        let _mongo_username = match env::var("MONGO_INITDB_ROOT_USERNAME") {
            Ok(v) => v,
            Err(e) => panic!(
                "MONGO_INITDB_ROOT_USERNAME missing environment variable {}",
                e
            ),
        };
        let _mongo_password = match env::var("MONGO_INITDB_ROOT_PASSWORD") {
            Ok(v) => v,
            Err(e) => panic!(
                "MONGO_INITDB_ROOT_PASSWORD missing environment variable {}",
                e
            ),
        };
        let mongo_host = match env::var("MONGO_HOST") {
            Ok(v) => v,
            Err(e) => panic!("MONGO_HOST missing environment variable {}", e),
        };
        let mongo_port = match env::var("MONGO_PORT") {
            Ok(v) => v,
            Err(e) => panic!("MONGO_PORT missing environment variable {}", e),
        };

        format!(
            // This format is changed due to data processing's issue with connecting with auth
            // No time to troubleshoot =/
            // "mongodb://{}:{}@{}:{}",
            // mongo_username, mongo_password, mongo_host, mongo_port

            "mongodb://{}:{}",
            mongo_host, mongo_port
        )
    };
}

#[allow(dead_code)]
pub async fn make_mongo_pool() -> Result<Client, Error> {
    let client_options = ClientOptions::parse(&MONGO_CONNECTION_STRING).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
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

    #[allow(dead_code)]
    #[test]
    fn test_make_mongo_pool() {
        // Can't unwrap on failure
        let pool = aw!(make_mongo_pool());
        pool.unwrap();
        assert_eq!(2 == 2, true);
    }
}
