use crate::util::get_string_path;
use std::fs;

#[allow(dead_code)]
// For reading a key from `keys` directory
fn read_key(key_file: &str) -> String {
    let path = get_string_path(&["data", "keys", &key_file]);
    let file = fs::read_to_string(&path).expect("Key file cannot be read");
    file
}

// Decoding encrypted text
#[allow(dead_code)]
pub fn crypto_to_text(content: &str, key: &str) -> String {
    todo!()
}

// Encrypting raw text
#[allow(dead_code)]
pub fn text_to_crypto(content: &str, key: &str) -> String {
    todo!()
}
