// use futures::executor::block_on;
use meilisearch_sdk::client::*;
use std::env;

lazy_static! {
    static ref MEILI_MASTER_KEY: String = match env::var("MEILI_MASTER_KEY") {
        Ok(v) => v,
        Err(e) => panic!("Meili host missing environment variable {}", e),
    };
    static ref MEILI_URL: String = {
        let host = match env::var("MEILI_HOST") {
            Ok(v) => v,
            Err(e) => panic!("Meili host missing environment variable {}", e),
        };

        let port = match env::var("MEILI_PORT") {
            Ok(v) => v,
            Err(e) => panic!("Meili host missing environment variable {}", e),
        };

        format!("http://{}:{}", host, port)
    };
    //pub static ref POOL: Client<'static> = {
    //    let client = Client::new(&MEILI_URL, &MEILI_MASTER_KEY);
    //    client
    //};
}

#[allow(dead_code)]
pub fn make_meili_pool() -> Client<'static> {
    let client = Client::new(&MEILI_URL, &MEILI_MASTER_KEY);
    client
}

//  ##############################
//  ########### Tests ############
//  ##############################

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::environment::load_variables;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_meili_health() {
        load_variables();
        let client = make_meili_pool();
        // Can't unwrap on failure
        aw!(client.health()).unwrap();
        assert_eq!(2 == 2, true);
    }
}
