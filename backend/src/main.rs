use warp::Filter;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate lazy_static;

mod error;
mod data_access;
mod entities;
mod logic;
pub mod schema;
mod service;
mod utils;

use self::{
    logic::{
        caching,
        entity_handlers::{location_handler, person_handler, university_handler},
    },
    service::routes::pg_routes::{location_routes, person_routes, university_routes},
    utils::environment::{load_variables, RESOURCES_DIR},
};

#[tokio::main]
async fn main() {
    // Runs container/local switch
    load_variables();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "content-type",
            "Access-Control-Allow-Origin",
        ])
        .allow_methods(vec!["POST", "GET", "PUT", "DELETE"])
        .build();

    let log = warp::log("api::request");

    println!("Backend: Establishing Warp routes...");
    let location_routes = list_locations!()
        .or(get_location!())
        .or(create_location!())
        .or(update_location!())
        .or(delete_location!());

    let person_routes = list_people!()
        .or(get_person!())
        .or(create_person!())
        .or(update_person!())
        .or(delete_person!());

    let university_routes = list_universities!()
        .or(get_university!())
        .or(create_university!())
        .or(update_university!())
        .or(delete_university!());

    let router = location_routes
        .or(person_routes)
        .or(university_routes)
        .recover(error::handle_rejection)
        .with(log)
        .with(cors);

    caching::scheduled_cache_clear().await;
    println!("{}", RESOURCES_DIR.display());
    println!("Backend: Starting Warp backend...");
    warp::serve(router).run(([0, 0, 0, 0], 3030)).await;
    println!("Backend: Exiting Warp backend")
}
