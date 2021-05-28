use futures::TryStreamExt;
use serde_json;
use warp::{
    multipart::{FormData, Part},
    Rejection, Reply,
};

use crate::entities::pg_entities::person::{
    Person,
    NewPerson,
    PeopleList
};

use crate::data_access::pg_connection::POOL;


pub async fn list() -> Result<impl warp::Reply, warp::Rejection>{
    let conn = POOL.get().unwrap();
    let response = PeopleList::list(&conn);
    Ok(warp::reply::json(&response))
}


pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Person::find(&id, &conn);

    let reply = match response {
        Ok(person) => {
            println!("{:#?}", &person);
            person
        }
        Err(e) => {
            println!("{:#?}", e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn create(new_person: NewPerson) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_person.create(&conn);

    let reply = match response {
        Ok(new_person) => {
            println!("{:#?}", &new_person);
        }
        Err(e) => {
            println!("{:#?}", &e);
            // Custom error recommended
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn update(id: i32, update_person: NewPerson) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Person::update(&id, &update_person, &conn);

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
    let response = Person::delete(&id, &conn);

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