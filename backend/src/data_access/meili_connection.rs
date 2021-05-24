use futures::executor::block_on;
use meilisearch_sdk::{client::*, document::*, indexes::*, progress::*, search::*, settings::*};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::prelude::*};
use crate::utils::environment::load_variables;

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

        format!("http://{}:{}",host,port)
    };

    pub static ref POOL: Client<'static> = {
        let client = Client::new(&MEILI_URL, &MEILI_MASTER_KEY);
        client
    };

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
    fn test_sqrt() {
        load_variables();
        let client = &POOL;
        aw!(client.health()).unwrap();
        assert_eq!(2 == 2, true);
    }
}
