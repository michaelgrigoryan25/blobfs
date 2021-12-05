use crate::util::fsx;
use crate::{handlers::Response, util::partial_infer};
use axum::{extract::Multipart, http::StatusCode, Json};

#[debug_handler]
pub async fn handler(mut multipart: Multipart) -> Result<Json<Response>, StatusCode> {
    let mut hashes: Vec<String> = vec![];
    // Vector for keeping hashes that were skipped for some reason
    let mut skipped: Vec<String> = vec![];

    // Looping through all fields
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().trim().to_string();
        let data = &field.bytes().await.unwrap();
        let mime = partial_infer(&data);

        // If the mimetype of the file was predicted
        if !mime.is_empty() {
            // Writing the file to the disk
            match fsx::write_file(data, &mime) {
                Ok(hash) => hashes.push(hash),
                Err(error) => {
                    // Breaking on single error
                    eprintln!("{}", &error);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
        } else if name.is_empty() {
            skipped.push("[unnamed field]".to_string());
        } else {
            skipped.push(name)
        }
    }

    // Sending a JSON response
    Ok(Json::from(Response::new(hashes, skipped)))
}
