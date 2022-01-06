use crate::config::user::{DEFAULT_PERMISSIONS, DEFAULT_USERNAME};
use serde::Deserialize;
use std::{
    env,
    mem::MaybeUninit,
    path::PathBuf,
    sync::{Mutex, Once},
};

mod permission;
mod user;

pub use permission::Permission;
pub use user::User;

const DEFAULT_ULOC: &str = "/data/";

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
                let config = Config::read_config();
                let singleton = ConfigSingletonReader {
                    inner: Mutex::new(config),
                };

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
#[non_exhaustive]
/// Configuration holder. Content from a config.yaml
/// file will be serialized into corresponding values.
pub struct Config {
    /// Vector of all users specified in the configuration.
    /// Will be only the default user if the configuration
    /// was not found or was invalid.
    users: Vec<User>,
    /// Initially, we are going to check if the port from
    /// the configuration exists and is valid. If it is [None]
    /// Stormi will try to get and parse the `STORMI_PORT`
    /// environment variable. If both environment variable
    /// and configuration ports are [None], then it will
    /// default to 6345.
    pub port: Option<u16>,
    /// Stormi will try to verify and bind to the address
    /// supplied from the `config.yaml` file. If the address
    /// is invalid or is [None], it will bind to `127.0.0.1`.
    pub addr: Option<String>,
    /// Default upload location for all files. If is [None],
    /// will default to `$PWD/data/`.
    pub uloc: String,

    // These fields are needed only internally, thus we don't need to deserialize them
    #[serde(skip_deserializing)]
    pub is_default_config: bool,

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
        if config.is_default_config {
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
    fn read_config() -> Config;
    fn get_users_size(&self) -> usize;
    fn verify_user(&self, username: &str, password: &str) -> Option<&User>;
}

impl ConfigTrait for Config {
    /// Default configuration. Includes only one
    /// [User] with default [crate::config::Permission::Read]
    /// and [crate::config::Permission::Write] permissions
    fn default() -> Self {
        Config {
            port: None,
            addr: None,
            config_path: None,
            is_default_config: true,
            users: vec![User::default()],
            uloc: (env::current_dir()
                .expect("Error while getting current directory")
                .to_string_lossy()
                + DEFAULT_ULOC)
                .to_string(),
        }
    }

    /// For reading the configuration from `config.yaml`
    fn read_config() -> Config {
        // If `config.yaml` was found
        if let Ok(config) = std::fs::read_to_string("config.yaml") {
            // Convert YAML to struct
            serde_yaml::from_str::<Config>(&config).expect("Invalid configuration error")
        } else {
            // Default configuration if the supplied config is either invalid or does not exist
            Config::default()
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
