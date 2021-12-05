use serde::Deserialize;
use std::{
    mem::MaybeUninit,
    sync::{Mutex, Once},
};

pub const DEFAULT_USERNAME: &str = "stormi";
const DEFAULT_PASSWORD: &str = "stormi-admin";
pub const DEFAULT_PERMISSIONS: &[Permission] = &[Permission::Read, Permission::Write];

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Permission {
    Read,
    Write,
}

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
    pub fn default() -> Self {
        User {
            username: DEFAULT_USERNAME.to_string(),
            password: DEFAULT_PASSWORD.to_string(),
            permissions: DEFAULT_PERMISSIONS.to_vec(),
        }
    }
}

pub trait ConfigTrait {
    fn get_users(&self) -> &Vec<User>;
    fn get_user(&self, username: &str) -> Option<&User>;
    fn read_config() -> Result<Config, serde_yaml::Error>;
}

impl Config {
    pub fn default() -> Self {
        Config {
            users: vec![User::default()],
            is_default_user: true,
        }
    }
}

impl ConfigTrait for Config {
    fn get_user(&self, username: &str) -> Option<&User> {
        let mut filtered = self.users.iter().filter(|it| it.username == username);
        filtered.next()
    }

    fn read_config() -> Result<Config, serde_yaml::Error> {
        if let Ok(config) = std::fs::read_to_string("config.yaml") {
            serde_yaml::from_str::<Config>(&config)
        } else {
            Ok(Config::default())
        }
    }

    fn get_users(&self) -> &Vec<User> {
        &self.users
    }
}
