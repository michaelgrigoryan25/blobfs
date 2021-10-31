use crate::{handlers::Response, util::fsx};
use axum::Json;
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Remove {
    hashes: Vec<String>,
}

#[debug_handler]
pub async fn handler(form: Json<Remove>) -> Result<Json<Response>, StatusCode> {
    // Vector of removed hashes
    let mut removed: Vec<String> = vec![];

    for hash in &form.hashes {
        // Removing the file by hash
        match fsx::remove_file(&hash).await {
            Ok(_) => {
                removed.push(hash.clone());
                continue;
            }
            Err(error) => {
                eprintln!("{}", &error);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }

    // Sending a JSON response
    Ok(Json::from(Response::new(removed)))
}
