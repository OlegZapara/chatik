use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use rand::rngs::OsRng;

pub fn encrypt(text: &String) -> String {
    let key_string =
        dotenvy::var("ENCRYPTION_KEY").expect("Encryption key is missing");

    let key = Key::<Aes256Gcm>::from_slice(key_string.as_bytes());
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, text.as_bytes()).unwrap();
    format!("{}:{}", hex::encode(nonce), hex::encode(ciphertext))
}

pub fn decrypt(value: &String) -> String {
    let key_string =
        dotenvy::var("ENCRYPTION_KEY").expect("Encryption key is missing");
    let key = Key::<Aes256Gcm>::from_slice(key_string.as_bytes());
    let cipher = Aes256Gcm::new(key);
    let parts: Vec<&str> = value.split(':').collect();
    if parts.len() != 2 {
        panic!("Invalid input format for decryption");
    }
    let decoded_nonce = hex::decode(parts[0]).unwrap();
    let nonce = Nonce::from_slice(&decoded_nonce);
    let encrypted_hex = parts[1];
    let decrypted_bytes = cipher
        .decrypt(nonce, hex::decode(encrypted_hex).unwrap().as_slice())
        .expect("Failed to decrypt");
    String::from_utf8(decrypted_bytes)
        .expect("Failed to convert decrypted bytes to String")
}
