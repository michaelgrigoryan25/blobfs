use crate::util::config::{ConfigSingletonReader, ConfigTrait};
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use hyper::StatusCode;

#[derive(Debug, Clone, Copy)]
pub struct Authentication;

#[async_trait]
impl<B> FromRequest<B> for Authentication
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Getting the authorization header
        let authorization = &req
            .headers()
            .expect("Cannot get headers")
            .get("Authorization");

        // Validating the header
        if let Some(authorization) = authorization {
            let bytes = base64::decode(&authorization).unwrap();
            let decoded = String::from_utf8_lossy(&bytes);

            // Reading config from singleton
            let config = ConfigSingletonReader::singleton()
                .inner
                .try_lock()
                .expect("Thread failed to unwrap `ConfigSingletonReader`");
            // Getting all users
            let users = &config.get_users();

            // Splitting the base64 decoded string and getting the username and password from it
            let mut decoded_split = decoded.split(':');
            let username = &decoded_split.next();
            let password = &decoded_split.next();

            if let (Some(username), Some(password)) = (*username, *password) {
                // Checking whether the user is valid
                let user = users
                    .iter()
                    .find(|it| it.username == username && it.password == password);

                // TODO: Map the value from `user` to the actual route from the request
                // More info: https://docs.rs/axum/latest/axum/#commonly-used-middleware
                if let Some(_) = user {
                    Ok(Self)
                } else {
                    Err(StatusCode::UNAUTHORIZED)
                }
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
