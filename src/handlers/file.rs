use axum::{extract::Path, http::HeaderValue};
use hyper::{HeaderMap, StatusCode};

use crate::util::fsx;

#[debug_handler]
pub async fn handler(Path(hash): Path<String>) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    match fsx::get_from_hash(hash.as_ref()) {
        Ok((content, mime)) => {
            let mut headers = HeaderMap::new();
            headers.append("Content-Type", HeaderValue::from_str(mime).unwrap());
            Ok((headers, content))
        }
        Err(error) => {
            error!("{}", &error);
            Err(StatusCode::NOT_FOUND)
        }
    }
}
