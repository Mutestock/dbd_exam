use crate::data_access::meili_connection::make_meili_pool;
use crate::data_access::pg_connection::POOL;
use crate::entities::pg_entities::university::{
    CachedUniversitiesList, CachedUniversity, NewUniversity, University, SearchUniversity
};
use crate::error::Error;
use crate::logic::caching;
use crate::entities::shared_behaviour::CacheAble;
use warp;

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let reply = match caching::check(caching::CACHE_UNIVERSITY_FORMAT).await {
        Ok(v) => CachedUniversitiesList::deserialize(&v),
        Err(_) => {
            let conn = POOL.get().unwrap();
            let res = CachedUniversitiesList::list(&conn);
            caching::set(caching::CACHE_UNIVERSITY_FORMAT, &res.serialize())
                .await
                .expect("Could not set university key, value pair");
            res
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let key_string = format!("{}{}", caching::CACHE_UNIVERSITY_FORMAT, id.to_string());

    // Tries to fetch from the cache first.
    // If it fails, it tries from the primary database
    // If that succeeds, the object gets cached and returned
    // If everything fails, warp returns an error message

    let reply = match caching::check(&key_string).await {
        Ok(v) => CachedUniversity::deserialize(&v),
        Err(_) => {
            let conn = POOL.get().unwrap();
            let response = University::find(&id, &conn);
            match response {
                Ok(university) => {
                    println!("{:#?}", &university);
                    caching::set(&key_string, &university.serialize())
                        .await
                        .expect("Could not set university key, value pair");
                    university
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

pub async fn create(new_university: NewUniversity) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_university.create(&conn);

    let reply = match response {
        Ok(new_university) => {
            println!("{:#?}", &new_university);
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
    update_university: NewUniversity,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = University::update(&id, &update_university, &conn);

    let reply = match response {
        Ok(null) => {
            caching::remove(&format!("{}{}", caching::CACHE_UNIVERSITY_FORMAT, id))
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
    let response = University::delete(&id, &conn);

    let reply = match response {
        Ok(null) => {
            caching::remove(&format!("{}{}", caching::CACHE_UNIVERSITY_FORMAT, id))
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
    let search_results = SearchUniversity::search(&search_str, &conn)
        .await;
    let reply = match search_results {
        Ok(results) => results,
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::custom(Error::SearchError));
        }
    };
    let res_str = format!("{:?}", reply);
    Ok(warp::reply::json(&res_str))
}
