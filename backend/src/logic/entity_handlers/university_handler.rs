use futures::TryStreamExt;
use serde_json;
use warp::{
    multipart::{FormData, Part},
    Rejection, Reply,
};

use crate::entities::pg_entities::university::{NewUniversity, UniversitiesList, University};

use crate::data_access::pg_connection::POOL;

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = UniversitiesList::list(&conn);
    Ok(warp::reply::json(&response))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = University::find(&id, &conn);

    let reply = match response {
        Ok(university) => {
            println!("{:#?}", &university);
            university
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn create(new_university: NewUniversity) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_university.create(&conn);

    let reply = match response {
        Ok(new_university) => {
            println!("{:#?}", &new_university);
        }
        Err(e) => {
            println!("{:#?}", &e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn update(
    id: i32,
    update_university: NewUniversity,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = University::update(&id, &update_university, &conn);

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
    let response = University::delete(&id, &conn);

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
