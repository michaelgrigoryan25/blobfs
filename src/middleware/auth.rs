use axum::response::IntoResponse;
use hyper::StatusCode;

// Authentication errors
static ERR_USER_NOT_FOUND: &str = "User was not found";
static ERR_INVALID_CREDENTIALS: &str = "Invalid credentials";
static ERR_MISSING_HEADER: &str = "`Authorization` header is missing from the request";

#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    BadCredentials,
    MissingAuthorizationHeader,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, ERR_USER_NOT_FOUND),
            AuthError::BadCredentials => (StatusCode::UNAUTHORIZED, ERR_INVALID_CREDENTIALS),
            AuthError::MissingAuthorizationHeader => (StatusCode::UNAUTHORIZED, ERR_MISSING_HEADER),
        };

        (status, message).into_response()
    }
}
