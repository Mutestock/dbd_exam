use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

// Many of these should be implemented later on in the process..
// Need ways of differentiating where the problems stem from.
// Bad Request is too broad

#[derive(Error, Debug)]
pub enum Error {
    #[error("Incorrect credentials")]
    IncorrectCredentialsError,

    #[error("JWT token isn't valid")]
    InvalidToken,

    #[error("JTW token creation error")]
    JWTTokenCreationError,

    #[error("Missing authentication header in request")]
    NoAuthHeaderError,

    #[error("Invalid authentication header")]
    InvalidAuthHeaderError,

    #[error("Access denied")]
    NoPermissionError,

    #[error("Database couldn't retrieve any data from that query")]
    NoDataFromQueryError,

    #[error("Entry could not be created in database")]
    EntryCreationError,

    #[error("Entry could not be updated in database")]
    EntryUpdateError,

    #[error("Entry could not be deleted from database")]
    EntryDeletionError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::IncorrectCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
            Error::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::InvalidToken => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenCreationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Error::NoDataFromQueryError => (StatusCode::BAD_REQUEST, e.to_string()),
            Error::EntryCreationError => (StatusCode::BAD_REQUEST, e.to_string()),
            Error::EntryDeletionError => (StatusCode::BAD_REQUEST, e.to_string()),
            Error::EntryUpdateError => (StatusCode::BAD_REQUEST, e.to_string()),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
        )
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json_response = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(warp::reply::with_status(json_response, code))
}
