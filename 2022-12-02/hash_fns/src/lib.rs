use rand::prelude::*;
use sha3::{Digest, Sha3_256};
use sha2::{Sha256};


fn random_bytes() -> Vec<u8> {
    let mut data = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut data);
    return data.clone().to_vec();
}

pub fn hash_random_blake3_256() -> () {
    blake3::hash(&random_bytes());
}

pub fn hash_random_sha3_256() -> () {
    let mut hasher = Sha3_256::new();
    hasher.update(&random_bytes());
    hasher.finalize();
}

pub fn hash_random_sha2_256() -> () {
    let mut hasher = Sha256::new();
    hasher.update(&random_bytes());
    hasher.finalize();
}