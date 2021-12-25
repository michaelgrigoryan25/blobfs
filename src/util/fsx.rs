use crate::util::{get_string_path, partial_infer};
use axum::body::Bytes;
use glob::{glob, GlobError};
use rand::{distributions::Alphanumeric, Rng};
use std::{fs, path::PathBuf};

const TXT_EXTENSION: &str = ".txt";

/// For removing files with their hash from `data` directory
pub fn remove_file(hash: &str) {
    let path = get_string_path(&["data", "files", hash]);

    // We are skipping the part of checking whether the file exists or not.
    // Instead the result is always void. If the file exists, remove it, if not there is no need for additional handling.
    // This will partially improve the performance. Since we don't need to check whether the file exists, we save some time and are able to send the response faster.
    let _ = fs::remove_file(path);
}

/// For creating files in `data` directory
pub fn write_file(bytes: &Bytes, mime: &str) -> Result<String, std::io::Error> {
    // Generating a random alphanumerical hash
    let hash: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        // Default length for all hashes
        .take(24)
        .map(char::from)
        .collect();

    // The output path of the files
    let mut path = get_string_path(&["data", "files", &hash]);

    // Getting the extension from the mime type if
    // it exists and appending it to the file path
    if let Some(value) = mime.split('/').nth(1) {
        path += &format!(".{}", value);
    } else {
        // Defaulting to a text file if the mime time cannot be determined
        path += TXT_EXTENSION;
    }

    fs::write(&path, bytes)?;

    Ok(hash)
}

/// For finding and reading a file with its hash
pub fn get_from_hash(hash: &str) -> Result<(Vec<u8>, String), Box<dyn std::error::Error>> {
    let hash_path = get_string_path(&["data", "files", hash]);
    let path: Result<PathBuf, GlobError> = glob(&format!("{}.*", hash_path))?.collect();
    let bytes = fs::read(path.unwrap())?;
    let mime = partial_infer(&bytes);
    Ok((bytes, mime))
}

#[cfg(test)]
mod tests {
    use crate::util::fsx::get_from_hash;

    const NONEXISTENT_FILENAME: &str = "nonexistent";

    #[test]
    fn test_get_from_hash() {
        assert!(get_from_hash(NONEXISTENT_FILENAME).is_err())
    }
}
