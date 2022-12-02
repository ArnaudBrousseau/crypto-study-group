use aes::Aes256;
use aes::cipher::{KeyInit, BlockEncrypt};
use aes::cipher::generic_array::{GenericArray};
use aes::cipher::BlockDecrypt;

/// Takes a hex-encoded string and decrypts it with a key, also hex-encoded
/// Uses AES-256
pub fn decrypt(s: String, k: String) -> String {
    let block_bytes: [u8; 16] = hex::decode(s.replace(" ", "")).unwrap().try_into().unwrap();
    let key_bytes: [u8; 32] = hex::decode(k.replace(" ", "")).unwrap().try_into().unwrap();

    let mut block = GenericArray::from(block_bytes);
    let key = GenericArray::from(key_bytes);
    let cipher = Aes256::new(&key);

    cipher.decrypt_block(&mut block);
    return hex::encode(block.to_vec());
}

pub fn encrypt(s: String, k: String) -> String {
    let block_bytes: [u8; 16] = hex::decode(s.replace(" ", "")).unwrap().try_into().unwrap();
    let key_bytes: [u8; 32] = hex::decode(k.replace(" ", "")).unwrap().try_into().unwrap();

    let mut block = GenericArray::from(block_bytes);
    let key = GenericArray::from(key_bytes);
    let cipher = Aes256::new(&key);

    cipher.encrypt_block(&mut block);
    return hex::encode(block.to_vec());
}

fn main() {
    let decrypted = decrypt(
        "53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07".to_string(),
        "80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01".to_string()
    );
    println!("decrypted: {}", decrypted);

    let encrypted = encrypt(
        "29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D".to_string(), 
        "80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01".to_string(),
    );
    println!("encrypted: {}", encrypted);
}
