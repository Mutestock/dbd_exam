use futures::TryStreamExt;
use serde_json;
use warp::{
    multipart::{FormData, Part},
    Rejection, Reply,
};

use crate::entities::pg_entities::location::{
    Location,
    NewLocation,
    LocationsList
};

use crate::data_access::pg_connection::POOL;


pub async fn list() -> Result<impl warp::Reply, warp::Rejection>{
    let conn = POOL.get().unwrap();
    let response = LocationsList::list(&conn);
    Ok(warp::reply::json(&response))
}


pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Location::find(&id, &conn);

    let reply = match response {
        Ok(location) => {
            println!("{:#?}", &location);
            location
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn create(new_location: NewLocation) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_location.create(&conn);

    let reply = match response {
        Ok(new_location) => {
            println!("{:#?}", &new_location);
        }
        Err(e) => {
            println!("{:#?}", &e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn update(id: i32, update_location: NewLocation) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Location::update(&id, &update_location, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn delete(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Location::delete(&id, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}