use serde::Serialize;

pub mod remove;
pub mod upload;

#[derive(Serialize)]
pub struct Response {
    hashes: Vec<String>,
    skipped: Vec<String>,
}

impl Response {
    pub fn new(hashes: Vec<String>, skipped: Vec<String>) -> Response {
        Response { hashes, skipped }
    }
}
