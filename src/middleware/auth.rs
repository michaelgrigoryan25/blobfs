use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::{self, StatusCode},
};
use serde::Deserialize;
use std::{fs, process::exit};

static mut MASTER_KEY: &str = "";

#[derive(Deserialize, Clone)]
struct Config {
    key: String,
}

// An extractor that performs authorization.
pub struct Authorization;

#[async_trait]
impl<B> FromRequest<B> for Authorization
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let authorization = &req
            .headers()
            .and_then(|headers| headers.get(http::header::AUTHORIZATION))
            .and_then(|value| value.to_str().ok());

        unsafe {
            if MASTER_KEY.is_empty() {
                let yaml = fs::read_to_string("config.yaml").unwrap_or_else(|_| {
                    println!("`config.yaml` was not supplied");
                    exit(1);
                });

                let config = serde_yaml::from_str::<Config>(&yaml).unwrap_or_else(|_| {
                    println!("`key` in `config.yaml` was not supplied");
                    exit(1);
                });

                let key = Box::leak(config.key.into_boxed_str());

                MASTER_KEY = &*key;
            }

            if let Some(value) = &authorization {
                if *value == MASTER_KEY {
                    return Ok(Self);
                }
            }
        }

        Err(StatusCode::UNAUTHORIZED)
    }
}
