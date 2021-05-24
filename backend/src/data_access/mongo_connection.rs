use mongodb::{Client, error::Error, options::ClientOptions};
use std::env;

lazy_static! {
    static ref MONGO_CONNECTION_STRING: String = {
        let mongo_username = match env::var("MONGO_INITDB_ROOT_USERNAME") {
            Ok(v) => v,
            Err(e) => panic!("Meili host missing environment variable {}", e),
        };
        let mongo_password = match env::var("MONGO_INITDB_ROOT_PASSWORD") {
            Ok(v) => v,
            Err(e) => panic!("Meili host missing environment variable {}", e),
        };
        let mongo_host = match env::var("MEILI_HOST") {
            Ok(v) => v,
            Err(e) => panic!("Meili host missing environment variable {}", e),
        };
        let mongo_port = match env::var("MONGO_PORT") {
            Ok(v) => v,
            Err(e) => panic!("Meili host missing environment variable {}", e),
        };

        format!(
            "mongodb://{}:{}@{}:{}",
            mongo_username, mongo_password, mongo_host, mongo_port
        )
    };
}

async fn make_mongo_pool() -> Result<Client, Error> {
    let mut client_options = ClientOptions::parse(&MONGO_CONNECTION_STRING).await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_sqrt(){
        let pool = aw!(make_mongo_pool()).unwrap();

        assert_eq!(2==2, true);
    }
}
