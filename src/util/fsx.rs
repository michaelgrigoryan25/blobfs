use axum::body::Bytes;
use glob::glob;
use infer::Type;
use rand::{distributions::Alphanumeric, Rng};
use std::{
    fs::{self, File},
    io::{self, Error},
};

#[allow(dead_code)]
// An interface for getting a file with the hash from `data` directory
pub fn get_from_hash(hash: &str) -> Result<File, io::Error> {
    let path = format!("./data/files/{}", &hash);
    File::open(&path)
}

// An interface for removing files with their hash from `data` directory
pub fn remove_file(hash: &str) -> Result<Result<(), Error>, &str> {
    let path = format!("./data/files/{}", &hash);
    let pattern = format!("{}*", &path);
    // Getting the first entry from match list
    let entry = glob(&pattern)
        .expect("Failed to read glob pattern")
        .flatten()
        .into_iter()
        .next();

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
    let mut path = format!("./data/files/{}", &hash);

    if mime.is_some() {
        // Adding suffix to the file
        path += &format!(".{}", &mime.unwrap().extension())
    }

    // Creating the file
    fs::write(&path, &bytes)?;

    Ok(hash)
}
