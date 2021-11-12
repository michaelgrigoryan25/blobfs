use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::{self, StatusCode},
};

// An extractor that performs authorization.
struct Authorization;

#[async_trait]
impl<B> FromRequest<B> for Authorization
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let auth_header = req
            .headers()
            .and_then(|headers| headers.get(http::header::AUTHORIZATION))
            .and_then(|value| value.to_str().ok());

        if let Some(value) = auth_header {
            if value == "test" {
                return Ok(Self);
            }
        }

        Err(StatusCode::UNAUTHORIZED)
    }
}
