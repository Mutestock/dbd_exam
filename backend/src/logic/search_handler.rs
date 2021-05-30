use crate::data_access::meili_connection::make_meili_pool;
use crate::entities::pg_entities::location::SearchLocation;
use warp;

pub async fn search(search_str: String) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = make_meili_pool();
    let search_results = SearchLocation::search(search_str, &conn)
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
