use crate::config::user::{DEFAULT_PERMISSIONS, DEFAULT_USERNAME};
use serde::Deserialize;
use std::{
    mem::MaybeUninit,
    path::PathBuf,
    sync::{Mutex, Once},
};

mod permission;
mod user;

pub use permission::Permission;
pub use user::User;

/// Singleton for loading the configuration once and reading from it
#[derive(Debug)]
pub struct ConfigSingletonReader {
    pub inner: Mutex<Config>,
}

impl ConfigSingletonReader {
    /// Method for initializing and getting [ConfigSingletonReader]
    pub fn singleton() -> &'static ConfigSingletonReader {
        // We'll be writing to this cell only once
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
/// Configuration holder. Content from a config.yaml
/// file will be serialized into corresponding values.
pub struct Config {
    /// Vector of all users specified in the configuration.
    /// Will be only the default user if the configuration
    /// was not found or was invalid.
    users: Vec<User>,

    // These fields are needed only internally, thus we don't need to deserialize them
    /// For detecting if the configuration
    /// has been created with a default user.
    #[serde(skip_deserializing)]
    pub is_default_user: bool,

    /// Configuration path(if exists)
    #[serde(skip_deserializing)]
    pub config_path: Option<PathBuf>,
}

impl Config {
    /// For initializing [ConfigSingletonReader] singleton
    pub fn init() {
        // Initializing the singleton
        let config = ConfigSingletonReader::singleton()
            .inner
            .lock()
            .expect("Thread failed to lock `ConfigSingletonReader`");

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
}

pub trait ConfigTrait {
    fn default() -> Config;
    fn get_users_size(&self) -> usize;
    fn read_config() -> Result<Config, serde_yaml::Error>;
    fn verify_user(&self, username: &str, password: &str) -> Option<&User>;
}

impl ConfigTrait for Config {
    /// Default configuration with default [User]
    fn default() -> Self {
        Config {
            config_path: None,
            is_default_user: true,
            users: vec![User::default()],
        }
    }

    /// For reading the configuration from `config.yaml`
    fn read_config() -> Result<Config, serde_yaml::Error> {
        // If `config.yaml` was found
        if let Ok(config) = std::fs::read_to_string("config.yaml") {
            // Convert YAML to struct
            serde_yaml::from_str::<Config>(&config)
        } else {
            // Default configuration if the supplied config is either invalid or does not exist
            Ok(Config::default())
        }
    }

    /// For getting the size of the list of user
    fn get_users_size(&self) -> usize {
        self.users.len()
    }

    /// For finding a valid user with the supplied username and password
    fn verify_user(&self, username: &str, password: &str) -> Option<&User> {
        self.users
            .iter()
            .find(|it| it.username == username && it.password == password)
    }
}
