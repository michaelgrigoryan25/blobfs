use crate::util::config::{Permission, User};
use crate::util::{fsx, has_permission};
use crate::{handlers::Response, util::partial_infer};
use axum::{extract::Multipart, http::StatusCode, Json};

#[debug_handler]
pub async fn handler(user: User, mut multipart: Multipart) -> Result<Json<Response>, StatusCode> {
    // TODO: Somehow get rid of the boilerplate
    if has_permission(&user, Permission::Write) {
        let (mut hashes, mut skipped) = (vec![], vec![]);

        // Looping through all fields
        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap().trim().to_string();
            let data = field.bytes().await.unwrap();
            let mime = partial_infer(&data);

            // If the mimetype of the file was predicted
            if !mime.is_empty() {
                // Writing the file to the disk
                match fsx::write_file(&data, &mime) {
                    Ok(hash) => hashes.push(hash),
                    _ => {
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
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
