use sodiumoxide::crypto::secretbox;
use serde_json::{Value, to_vec, from_slice};
use sodiumoxide::base64::{encode, decode, Variant};
use lazy_static::lazy_static;

lazy_static! {
    static ref ENCRYPTION_KEY: secretbox::Key = {
        let key = secretbox::gen_key();
        println!("SODIUM_KEY: {}", encode(key.as_ref(), Variant::Original)); // Print the key
        key
    };
}

pub fn encrypt_json(data: &Value, key: &secretbox::Key) -> Result<String, String> {
    let nonce = secretbox::gen_nonce();
    let plaintext = to_vec(data).map_err(|_| "Failed to serialize JSON".to_string())?;
    let ciphertext = secretbox::seal(&plaintext, &nonce, key);

    let mut combined = nonce.as_ref().to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(encode(&combined, Variant::Original))
}

pub fn decrypt_json(encrypted_data: &str, key: &secretbox::Key) -> Result<Value, String> {
    let combined = decode(encrypted_data, Variant::Original).map_err(|e| format!("Failed to decode base64: {:?}", e))?;
    println!("Decoded Bytes: {:?}", combined);
    let nonce_len = secretbox::NONCEBYTES;
    let nonce_bytes = &combined[..nonce_len];
    let ciphertext = &combined[nonce_len..];

    let nonce = secretbox::Nonce::from_slice(nonce_bytes).ok_or("Failed to create nonce from slice".to_string())?;

    let plaintext = secretbox::open(ciphertext, &nonce, key).map_err(|_| "Failed to decrypt data".to_string())?;

    from_slice(&plaintext).map_err(|_| "Failed to deserialize JSON".to_string())
}

pub fn get_key() -> secretbox::Key {
    ENCRYPTION_KEY.clone()
}

pub fn decrypt_string(encrypted_data: &str) -> Result<Value, String> {
    let key = get_key();
    decrypt_json(encrypted_data, &key)
}

pub fn get_key_base64() -> String {
    encode(ENCRYPTION_KEY.as_ref(), Variant::Original)
}

// Add a function to force key generation.
pub fn initialize_key() {
    let _ = get_key(); // Force key generation by accessing it.
}