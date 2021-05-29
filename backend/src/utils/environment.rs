use dotenv;
use std::{env, path::PathBuf};

lazy_static! {
    pub static ref RESOURCES_DIR: PathBuf =
        env::current_dir().expect("Could not find current directory for some reason");
}

pub fn load_variables() {
    match env::var_os("PRODUCTION_VARIABLES") {
        Some(_) => {
            println!("Backend: Production environment variable found. .env.development not loaded")
        }
        None => {
            println!(
                "Backend: No production environment variable. .env.development variables activated"
            );
            dotenv::from_filename(".env.development").ok();
        }
    }
}
