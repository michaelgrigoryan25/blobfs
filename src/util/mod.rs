use self::config::{Permission, User};
use std::env;

pub mod config;
pub mod fsx;

// For reading files from the `data` folder
pub fn get_string_path(path_segments: &[&str]) -> String {
    let mut current_dir = env::current_dir().unwrap();
    path_segments.iter().for_each(|it| current_dir.push(it));
    current_dir.to_string_lossy().to_string()
}

// For checking whether a user has the specified permission
pub fn has_permission(user: &User, permission: Permission) -> bool {
    user.permissions.iter().any(|&it| it == permission)
}

// This line infers file's mimetype without reading it completely
// Instead it reads the file to the point where the mime type can be inferred
// Here is equivalent logic:
//
// {
//     let mut tmp: Vec<u8> = vec![];
//     for byte in &bytes {
//         tmp.push(*byte);
//         match infer::get(&tmp) {
//             Some(t) => {
//                 mime = t.mime_type().to_string();
//                 break;
//             }
//             None => {
//                 println!("Moved to {}", &idx);
//             }
//         };
//     }
// }
pub fn partial_infer<T>(bytes: T) -> String
where
    T: AsRef<[u8]>,
{
    #[inline]
    fn inner(bytes: &[u8]) -> String {
        (0..bytes.len())
            .find_map(|end| Some(infer::get(&bytes[..end])?.mime_type().to_string()))
            .unwrap_or_default()
    }

    inner(bytes.as_ref())
}

#[cfg(test)]
mod tests {
    use crate::util::partial_infer;

    const BYTES: &[u8] = &[0xFF, 0xD8, 0xFF, 0xAA];

    #[test]
    fn test_partial_infer() {
        assert_eq!("image/jpeg", partial_infer(BYTES));
    }
}
