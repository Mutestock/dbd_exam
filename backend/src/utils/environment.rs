use dotenv;
use std::env;

pub fn load_variables() {
    match env::var_os("PRODUCTION_VARIABLES"){
        Some(_) => println!("Production environment variable found. .env.development not loaded"),
        None => {
            println!("No production environment variable. .env.development variables activated");
            dotenv::from_filename(".env.development").ok();
        }
    }
}
