use crate::util::get_string_path;
use rand::rngs::OsRng;
use rsa::{
    pkcs1::FromRsaPrivateKey, pkcs8::FromPublicKey, PaddingScheme, PublicKey, RsaPrivateKey,
    RsaPublicKey,
};
use std::fs;

#[allow(dead_code)]
// For reading a key from `keys` directory
pub fn read_key(key_file: &str) -> String {
    let path = get_string_path(&["data", "keys", key_file]);
    fs::read_to_string(&path).expect("Key file cannot be read")
}

#[allow(dead_code)]
// Encrypting raw text
pub fn encrypt(content: Vec<u8>, public_key: &str) -> Vec<u8> {
    let rsa = RsaPublicKey::from_public_key_pem(public_key).unwrap();
    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    rsa.encrypt(&mut OsRng, padding, &content).unwrap()
}

#[allow(dead_code)]
// Decoding encrypted text
pub fn decrypt(encrypted: Vec<u8>, private_key: &str) -> Vec<u8> {
    let rsa = RsaPrivateKey::from_pkcs1_pem(private_key).unwrap();
    let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    rsa.decrypt(padding, &encrypted).unwrap()
}

#[test]
fn test_encrypt_decrypt() {
    let encrypted = encrypt("hello".as_bytes().to_vec(), &read_key("test_public.pem"));
    assert_ne!("hello", String::from_utf8_lossy(&encrypted));

    let decrypted = decrypt(encrypted, &read_key("test_private.pem"));
    assert_eq!("hello", String::from_utf8_lossy(&decrypted));
}
