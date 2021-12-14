use crate::middleware::auth::AuthError;
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use serde::Deserialize;
use std::{
    mem::MaybeUninit,
    sync::{Arc, Mutex, Once},
};

pub const DEFAULT_USERNAME: &str = "stormi";
const DEFAULT_PASSWORD: &str = "stormi-admin";
pub const DEFAULT_PERMISSIONS: &[Permission] = &[Permission::Read, Permission::Write];

// An enum for controlling the permissions of users
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum Permission {
    #[serde(rename = "READ")]
    Read,
    #[serde(rename = "WRITE")]
    Write,
}

// A singleton for loading the configuration once and reading from it
#[derive(Debug)]
pub struct ConfigSingletonReader {
    pub inner: Mutex<Config>,
}

impl ConfigSingletonReader {
    pub fn singleton() -> &'static ConfigSingletonReader {
        static ONCE: Once = Once::new();
        // Create an uninitialized static
        static mut SINGLETON: MaybeUninit<ConfigSingletonReader> = MaybeUninit::uninit();

        unsafe {
            ONCE.call_once(|| {
                let singleton: ConfigSingletonReader;

                // Reading the configuration from `config.yaml`
                // If the deserialization did not fail
                if let Ok(config) = Config::read_config() {
                    singleton = ConfigSingletonReader {
                        inner: Mutex::new(config),
                    };
                } else {
                    singleton = ConfigSingletonReader {
                        inner: Mutex::new(Config::default()),
                    }
                }

                // Initializing the singleton
                SINGLETON.write(singleton);
            });

            // Returning the shared reference to the data,
            // which is safe to use concurrently
            SINGLETON.assume_init_ref()
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    users: Vec<User>,

    // This is needed only internally, thus we don't need to deserialize it
    #[serde(skip_deserializing)]
    pub is_default_user: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub permissions: Vec<Permission>,
}

impl User {
    // Default user
    pub fn default() -> Self {
        User {
            username: DEFAULT_USERNAME.to_string(),
            password: DEFAULT_PASSWORD.to_string(),
            permissions: DEFAULT_PERMISSIONS.to_vec(),
        }
    }
}

// TODO: Move the implementation logic with the user elsewhere
#[async_trait]
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
                .expect("Thread failed to unwrap `ConfigSingletonReader`");

            // Splitting the base64 decoded string and getting the username and password from it
            let mut decoded_split = decoded.split(':');
            let username = &decoded_split.next();
            let password = &decoded_split.next();

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

impl Config {
    // Default configuration with default user
    pub fn default() -> Self {
        Config {
            users: vec![User::default()],

            // Used internally
            is_default_user: true,
        }
    }

    // For initializing the `ConfigSingletonReader` singleton
    pub fn init() {
        // Initializing the singleton
        let config = ConfigSingletonReader::singleton()
            .inner
            .lock()
            .expect("Thread failed to unwrap `ConfigSingletonReader`");

        // If configuration file was not found
        if config.is_default_user {
            println!("> Configuration file `config.yaml` is invalid or does not exist");
            println!("..This is highly insecure. Please consider adding valid configuration");
            println!(
                "..Defaulting to user `{}` with default permissions: {:?}",
                DEFAULT_USERNAME, DEFAULT_PERMISSIONS
            );
        } else {
            println!(
                "> Configuration loaded. Detected {} user(s)",
                config.get_users_size()
            )
        }
    }

    // For reading the configuration from `config.yaml`
    pub fn read_config() -> Result<Config, serde_yaml::Error> {
        // If `config.yaml` was found
        if let Ok(config) = std::fs::read_to_string("config.yaml") {
            // Convert YAML to struct
            serde_yaml::from_str::<Config>(&config)
        } else {
            // Default configuration if the supplied config is either invalid or does not exist
            Ok(Config::default())
        }
    }
}

pub trait ConfigTrait {
    fn get_users_size(&self) -> usize;
    fn verify_user(&self, username: &str, password: &str) -> Option<&User>;
}

impl ConfigTrait for Config {
    // For getting one user from the configuration
    fn verify_user(&self, username: &str, password: &str) -> Option<&User> {
        self.users
            .iter()
            .find(|it| it.username == username && it.password == password)
    }

    // For getting the length of user list
    fn get_users_size(&self) -> usize {
        Arc::from(&self.users).len()
    }
}
