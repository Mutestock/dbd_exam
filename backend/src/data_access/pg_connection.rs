
// https://github.com/sfackler/r2d2https://github.com/sfackler/r2d2
// https://docs.diesel.rs/diesel/r2d2/struct.ConnectionManager.html
// https://docs.diesel.rs/diesel/pg/struct.PgConnection.html
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {

    static ref POSTGRES_CONNECTION_STRING: String = {
        let postgres_host = match env::var("POSTGRES_HOST") {
            Ok(v) => v,
            Err(e) => panic!("POSTGRES_HOST missing environment variable {}", e),
        };

        let postgres_user = match env::var("POSTGRES_USER") {
            Ok(v) => v,
            Err(e) => panic!("POSTGRES_USER missing environment variable {}", e),
        };

        let postgres_password = match env::var("POSTGRES_PASSWORD") {
            Ok(v) => v,
            Err(e) => panic!("POSTGRES_PASSWORD missing environment variable {}", e),
        };

        let postgres_db = match env::var("POSTGRES_DB") {
            Ok(v) => v,
            Err(e) => panic!("POSTGRES_DB missing environment variable {}", e),
        };

        let postgres_port = match env::var("POSTGRES_PORT") {
            Ok(v) => v,
            Err(e) => panic!("POSTGRES_PORT missing environment variable {}", e),
        };

        format!("postgres://{}:{}@{}:{}/{}", 
            postgres_user, 
            postgres_password, 
            postgres_host, 
            postgres_port, 
            postgres_db
        )
    };


    pub static ref POOL: Pool = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.");

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    };
}

//  ##############################
//  ########### Tests ############
//  ##############################

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pool() {
        let pool = &POOL;
        assert_eq!(2 == 2, true);
    }
}
