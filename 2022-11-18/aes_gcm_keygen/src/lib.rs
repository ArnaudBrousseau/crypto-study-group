use aes_gcm::{
    aead::{KeyInit, OsRng, rand_core::RngCore},
    Aes256Gcm
};

use ring::{aead};

pub fn rust_crypto_aes_gcm_keygen() -> () {
    let _ = Aes256Gcm::generate_key(&mut OsRng);
}

pub fn ring_aes_gcm_keygen() -> () {
    let mut key: [u8; 32] = [0; 32];
    OsRng.fill_bytes(&mut key);
    let _ = aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap();
}