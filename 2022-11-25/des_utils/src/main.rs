use des::cipher::generic_array::GenericArray;
use des::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use des::Des;

/// Takes an array of bytes and encrypt with DES
pub fn encrypt(s: [u8; 8], k: [u8; 8]) -> [u8; 8] {
    let mut block = GenericArray::from(s);
    let key = GenericArray::from(k);
    let cipher = Des::new(&key);

    cipher.encrypt_block(&mut block);
    return block.into();
}

/// Takes an array of bytes and decrypts it with DES
pub fn decrypt(s: [u8; 8], k: [u8; 8]) -> [u8; 8] {
    let mut block = GenericArray::from(s);
    let key = GenericArray::from(k);
    let cipher = Des::new(&key);

    cipher.decrypt_block(&mut block);
    return block.into();
}

fn main() {
    let pattern = 0b01010101u8;
    let complement = 0b10101010u8;
    let c = encrypt([pattern; 8], [pattern; 8]);

    let c_prime = encrypt([complement; 8], [complement; 8]);

    for i in 0..8 {
        println!("orig. byte#{}: {:#010b}", i, c[i]);
        println!("comp. byte#{}: {:#010b}", i, c_prime[i]);
        println!("")
    }
}
