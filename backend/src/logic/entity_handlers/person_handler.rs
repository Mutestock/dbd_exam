use warp;

use crate::data_access::pg_connection::POOL;
use crate::entities::pg_entities::person::{CachedPerson, NewPerson, CachedPeopleList, Person};
use crate::entities::shared_behaviour::CacheAble;
use crate::logic::caching;

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let reply = match caching::check(caching::CACHE_PERSON_FORMAT).await {
        Ok(v) => CachedPeopleList::deserialize(&v),
        Err(_) => {
            let conn = POOL.get().unwrap();
            let res = CachedPeopleList::list(&conn);
            caching::set(caching::CACHE_PERSON_FORMAT, &res.serialize())
                .await
                .expect("Could not set university key, value pair");
            res
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let key_string = format!("{}{}", caching::CACHE_PERSON_FORMAT, id.to_string());

    // Tries to fetch from the cache first.
    // If it fails, it tries from the primary database
    // If that succeeds, the object gets cached and returned
    // If everything fails, warp returns an error message

    let reply = match caching::check(&key_string).await {
        Ok(v) => CachedPerson::deserialize(&v),
        Err(_) => {
            let conn = POOL.get().unwrap();
            let response = Person::find(&id, &conn);
            match response {
                Ok(person) => {
                    println!("{:#?}", &person);
                    caching::set(&key_string, &person.serialize())
                        .await
                        .expect("Could not set person key, value pair");
                    person
                }
                Err(e) => {
                    println!("{:#?}", e);
                    // Custom error recommended
                    return Err(warp::reject::not_found());
                }
            }
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

pub async fn update(
    id: i32,
    update_person: NewPerson,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Person::update(&id, &update_person, &conn);

    let reply = match response {
        Ok(null) => {
            caching::remove(&format!("{}{}", caching::CACHE_PERSON_FORMAT, id))
                .await
                .expect("Could not remove key from cached_location_{}");
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
            caching::remove(&format!("{}{}", caching::CACHE_PERSON_FORMAT, id))
                .await
                .expect("Could not remove key from cached_location_{}");
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
