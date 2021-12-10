use axum::response::IntoResponse;
use hyper::StatusCode;

#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    BadCredentials,
    MissingAuthorizationHeader,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "User was not found"),
            AuthError::BadCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::MissingAuthorizationHeader => (
                StatusCode::UNAUTHORIZED,
                "`Authorization` header is missing from the request",
            ),
        };

        (status, message).into_response()
    }
}
