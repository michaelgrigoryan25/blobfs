use serde::Deserialize;

/// An enum for controlling the permissions of User
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum Permission {
    /// Permission for reading files
    #[serde(alias = "Read", alias = "READ", alias = "R", alias = "r")]
    Read,
    /// Permission for creating, updating and deleting files
    #[serde(alias = "Write", alias = "WRITE", alias = "W", alias = "w")]
    Write,
}
