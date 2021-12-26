use crate::{
    config::{Permission, User},
    util::{fsx, has_permission},
};
use axum::{extract::Path, http::HeaderValue};
use hyper::{HeaderMap, StatusCode};

#[debug_handler]
#[rustfmt::skip]
pub async fn handler(user: User, Path(hash): Path<String>) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    // TODO: Somehow get rid of the boilerplate
    if has_permission(&user, Permission::Read) {
        return match fsx::get_from_hash(hash.as_ref()) {
            Ok((content, mime)) => {
                let mut headers = HeaderMap::new();
                headers.append("Content-Type", HeaderValue::from_str(&mime).unwrap());

                Ok((headers, content))
            }
            _ => Err(StatusCode::NOT_FOUND),
        };
    }

    Err(StatusCode::FORBIDDEN)
}
