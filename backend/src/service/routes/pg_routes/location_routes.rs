use warp::{filters::BoxedFilter, path, Filter};

use crate::entities::pg_entities::location::NewLocation;

// Courtesy of steadylearner
// https://github.com/steadylearner/Rust-Full-Stack/blob/master/warp/database/2.%20with_db_pool/src/routes/post_route_without_reusable.rs

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "location" / ..).boxed()
}

pub fn list() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .boxed()
}

pub fn get() -> BoxedFilter<(i32,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn create() -> BoxedFilter<(NewLocation,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn update() -> BoxedFilter<(i32, NewLocation)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::put()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .and(json_body)
        .boxed()
}

pub fn delete() -> BoxedFilter<(i32,)> {
    warp::delete()
        .and(path_prefix())
        .and(warp::path::param::<i32>())
        .boxed()
}

pub fn search() -> BoxedFilter<(String, )>{
    warp::get()
        .and(path_prefix())
        .and(warp::path("search"))
        .and(warp::path::param::<String>())
        .boxed()
}