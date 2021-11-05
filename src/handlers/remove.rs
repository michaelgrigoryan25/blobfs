use crate::{handlers::Response, util::fsx};
use axum::Json;
use hyper::StatusCode;
use serde::Deserialize;

// The JSON form which is going to hold all the hashes that the client wants to delete
#[derive(Deserialize)]
pub struct Remove {
    hashes: Vec<String>,
}

#[debug_handler]
pub async fn handler(form: Json<Remove>) -> Result<Json<Response>, StatusCode> {
    let mut removed: Vec<String> = vec![];
    // Vector of skipped hashes
    let mut skipped: Vec<String> = vec![];

    for hash in &form.hashes {
        // Skipping the hashes that do not meet minimum length requirements
        if hash.len() != 24 {
            skipped.push(hash.to_string());
        } else {
            // Removing the file by hash
            if fsx::remove_file(hash).is_ok() {
                removed.push(hash.clone());
            } else {
                skipped.push(hash.to_string());
            }
        }
    }

    // Sending a JSON response
    Ok(Json::from(Response::new(removed, skipped)))
}
