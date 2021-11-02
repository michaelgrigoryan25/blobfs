use axum::response::IntoResponse;
use hyper::StatusCode;

pub async fn handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nope. There is nothing in here")
}
