use std::env;

pub mod fsx;
pub mod config;

// For reading files from the `data` folder
pub fn get_string_path(path_segments: &[&str]) -> String {
    let mut current_dir = env::current_dir().unwrap();

    for &segment in path_segments {
        current_dir.push(segment);
    }

    current_dir.to_string_lossy().to_string()
}

// This line infers file's mimetype without reading it completely
// Instead it reads the file to the point where the mime type can be inferred
// Here is the equivalent logic
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

    #[test]
    fn test_partial_infer() {
        assert_eq!("image/jpeg", partial_infer([0xFF, 0xD8, 0xFF, 0xAA]));
    }
}
