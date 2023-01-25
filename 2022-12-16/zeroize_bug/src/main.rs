use zeroize::Zeroize;

#[derive(Debug)]
struct KeyPair {
    public_key: [u8; 4],
    private_key: [u8; 4],
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        self.private_key.zeroize();
        self.public_key.zeroize();
    }
}

fn main() {
    let key_pair = KeyPair {
        public_key: *b"PUBL",
        private_key: *b"PRIV",
    };
    let secret_mem_locus = key_pair.private_key.as_ptr();
    println!("Secret locus: {secret_mem_locus:p}");
    drop(key_pair);
    println!("Keypair erased...or is it?");

    println!("Reading secret locus: {:?}", unsafe {
        String::from_utf8_unchecked(core::slice::from_raw_parts(secret_mem_locus, 4).to_vec())
    });
}
