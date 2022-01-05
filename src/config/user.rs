use super::Permission;
use serde::Deserialize;

/// Default username for default user
pub const DEFAULT_USERNAME: &str = "stormi";
/// Default password for default user
pub const DEFAULT_PASSWORD: &str = "stormi-admin";
/// Default permissions for default user
pub const DEFAULT_PERMISSIONS: &[Permission] = &[Permission::Read, Permission::Write];

/// [User] struct that will be read from the configuration.
/// This struct is used internally too, for user authentication.
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,

    /// [User] permissions
    pub permissions: Vec<Permission>,
}

impl User {
    /// Creates a new default user with default credentials.
    /// Username as [DEFAULT_USERNAME], password as [DEFAULT_PASSWORD]
    /// with default permissions [DEFAULT_PERMISSIONS]
    pub fn default() -> Self {
        User {
            username: DEFAULT_USERNAME.to_string(),
            password: DEFAULT_PASSWORD.to_string(),
            permissions: DEFAULT_PERMISSIONS.to_vec(),
        }
    }
}
