use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;

pub mod v1;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not Found")]
    NotFound,
    #[error("Conflict")]
    Conflict,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        let status = match self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Conflict => StatusCode::CONFLICT,
        };
        Response::builder()
            .status(status)
            .body(Body::from(self.to_string()))
            .unwrap()
    }
}
