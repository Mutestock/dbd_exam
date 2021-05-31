use warp;

use crate::data_access::meili_connection::make_meili_pool;
use crate::data_access::pg_connection::POOL;
use crate::entities::pg_entities::location::SearchLocation;
use crate::entities::pg_entities::location::{
    CachedLocation, CachedLocationsList, Location, NewLocation,
};
use crate::entities::shared_behaviour::CacheAble;
use crate::error::Error;
use crate::logic::caching;

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let reply = match caching::check(caching::CACHE_LOCATION_FORMAT).await {
        Ok(v) => CachedLocationsList::deserialize(&v),
        Err(_) => {
            let conn = POOL.get().unwrap();
            let res = CachedLocationsList::list(&conn);
            caching::set(caching::CACHE_LOCATION_FORMAT, &res.serialize())
                .await
                .expect("Could not set university key, value pair");
            res
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let key_string = format!("{}{}", caching::CACHE_LOCATION_FORMAT, id.to_string());

    // Tries to fetch from the cache first.
    // If it fails, it tries from the primary database
    // If that succeeds, the object gets cached and returned
    // If everything fails, warp returns an error message

    let reply = match caching::check(&key_string).await {
        Ok(v) => CachedLocation::deserialize(&v),
        Err(_) => {
            let conn = POOL.get().unwrap();
            let response = Location::find(&id, &conn);
            match response {
                Ok(location) => {
                    println!("{:#?}", &location);
                    caching::set(&key_string, &location.serialize())
                        .await
                        .expect("Could not set location key, value pair");
                    location
                }
                Err(e) => {
                    println!("{:#?}", e);
                    return Err(warp::reject::custom(Error::NoDataFromQueryError));
                }
            }
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
            println!("{:#?}", e);
            return Err(warp::reject::custom(Error::EntryCreationError));
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn update(
    id: i32,
    update_location: NewLocation,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Location::update(&id, &update_location, &conn);

    let reply = match response {
        Ok(null) => {
            caching::remove(&format!("{}{}", caching::CACHE_LOCATION_FORMAT, id))
                .await
                .expect("Could not remove key from cached_location_{}");
            println!("{:#?}", &null);
            null
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::custom(Error::EntryUpdateError));
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn delete(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Location::delete(&id, &conn);

    let reply = match response {
        Ok(null) => {
            caching::remove(&format!("{}{}", caching::CACHE_LOCATION_FORMAT, id))
                .await
                .expect("Could not remove key from cached_location_{}");
            println!("{:#?}", &null);
            null
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::custom(Error::EntryDeletionError));
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn search(search_str: String) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = make_meili_pool();
    let search_results = SearchLocation::search(&search_str, &conn)
        .await;
    let reply = match search_results {
        Ok(results) => results,
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    let res_str = format!("{:?}", reply);
    Ok(warp::reply::json(&res_str))
}
