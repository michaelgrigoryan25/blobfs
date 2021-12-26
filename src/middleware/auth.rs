use super::AuthError;
use crate::config::{ConfigSingletonReader, ConfigTrait, User};
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};

#[async_trait]
/// Authentication implementation for `User` struct
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let authorization = req
            .headers()
            .expect("Cannot get headers")
            .get("authorization");

        if let Some(authorization) = authorization {
            let bytes = base64::decode(&authorization).unwrap();
            let decoded = String::from_utf8_lossy(&bytes);

            // Reading config from singleton
            let config = ConfigSingletonReader::singleton()
                .inner
                .lock()
                .expect("Thread failed to lock `ConfigSingletonReader`");

            // Splitting the base64 decoded string and getting the username and password from it
            let mut decoded_split = decoded.split(':');
            let (username, password) = (&decoded_split.next(), &decoded_split.next());

            if let (Some(username), Some(password)) = (*username, *password) {
                // Checking whether the user is valid
                let user = config.verify_user(username, password);

                // Checking whether the user is valid
                if let Some(user) = user {
                    return Ok(user.clone());
                }

                return Err(AuthError::UserNotFound);
            }

            return Err(AuthError::BadCredentials);
        }

        return Err(AuthError::MissingAuthorizationHeader);
    }
}
