use warp::Filter;

#[macro_use]
extern crate diesel;
pub mod schema;


#[macro_use]
extern crate lazy_static;

mod data_access;
mod utils;
mod entities;

#[tokio::main]
async fn main() {
    println!("Starting Warp backend");
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030))
        .await;
    println!("Exiting Warp backend")
}

