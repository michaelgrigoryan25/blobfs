use crate::util::get_string_path;
use axum::body::Bytes;
use glob::glob;
use infer::Type;
use rand::{distributions::Alphanumeric, Rng};
use std::{fs, io::Error};

// An interface for removing files with their hash from `data` directory
pub fn remove_file(
    hash: &str,
) -> Result<
    Result<(), Error>,
    // String error
    &str,
> {
    let path = get_string_path(&["data", "files", hash]);
    // Glob pattern for finding the file with the corresponding hash
    let pattern = format!("{}*", &path);

    // Getting the first entry from match list
    let entry = glob(&pattern)
        .expect("Failed to read glob pattern")
        .flatten()
        .into_iter()
        .next();

    // Entry exists
    if let Some(path) = entry {
        return Ok(fs::remove_file(&path.as_os_str()));
    }

    Err("File could not be found")
}

// An interface for creating files in `data` directory
pub fn write_file(bytes: &Bytes, mime: &Option<Type>) -> Result<String, Error> {
    // Generating a random alphanumerical hash
    let hash: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();

    // The output path of the files
    let mut path = get_string_path(&["data", "files", &hash]);

    if mime.is_some() {
        // Adding suffix to the file
        path += &format!(".{}", &mime.unwrap().extension())
    }

    // Creating the file
    fs::write(&path, &bytes)?;
    Ok(hash)
}
