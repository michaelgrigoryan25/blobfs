use crate::handlers::Response;
use crate::util::fsx;
use axum::{extract::Multipart, http::StatusCode, Json};

#[debug_handler]
pub async fn handler(multipart: Multipart) -> Result<Json<Response>, StatusCode> {
    let mut hashes: Vec<String> = vec![];
    let mut multipart = multipart;

    // Looping through all fields
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = &field.bytes().await.unwrap();
        let mime = infer::get(&data);

        // Writing a
        match fsx::write_file(&data, &mime) {
            Ok(hash) => hashes.push(hash),
            Err(error) => {
                eprintln!("{}", &error);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }

    // Sending a JSON response
    Ok(Json::from(Response::new(hashes)))
}
