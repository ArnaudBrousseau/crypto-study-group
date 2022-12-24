use bitvec::prelude::*;
use std::str::from_utf8;

use sha2::{Digest, Sha512};

pub fn sha512n(message: &[u8], n: usize) -> BitVec<u8> {
    assert!(n > 0, "sha512n cannot run with n <= 0");
    assert!(n <= 512, "sha512n cannot run with n > 512");

    let mut hasher = Sha512::new();
    hasher.update(message);
    let result = hasher.finalize().to_vec();

    let bits = result.view_bits::<LocalBits>();
    return bits[0..n].to_bitvec();
}

/// Finds a collision for a target bitvec
/// The collision algorithm takes the base and appends numbers until sha-512-n matches the target
pub fn find_collision(target: BitVec<u8>, base: &[u8]) -> Vec<u8> {
    let mut nonce: u32 = 0;
    loop {
        let mut candidate = base.clone().to_vec();
        let mut nonce_bytes = nonce.to_string().as_bytes().to_vec();
        candidate.append(&mut nonce_bytes);
        let res = sha512n(&candidate, target.len());
        if res == target {
            return candidate;
        }
        // If the candidate isn't a match, keep going by incrementing the nonce
        nonce += 1;
    }
}

fn main() {
    let target = b"hello";
    let target_hash = sha512n(target, 16);
    let collision_base = b"world";
    let collision = find_collision(target_hash.clone(), collision_base);
    println!(
        "Collision found! \"{}\" has the same first {} bytes under SHA-512 than \"{}\"",
        from_utf8(&collision).unwrap(),
        target_hash.len(),
        from_utf8(target).unwrap(),
    );

    // Now we can also target hashes directly. Exercise 5.4 asks for something which hashes to 3D 4B
    let collision = find_collision(
        hex::decode("3D4B")
            .unwrap()
            .view_bits::<LocalBits>()
            .to_bitvec(),
        collision_base,
    );
    println!(
        "Collision found! \"{}\" hashes to 3D4B... with SHA-512",
        from_utf8(&collision).unwrap(),
    );

    let collision = find_collision(
        hex::decode("3D4B")
            .unwrap()
            .view_bits::<LocalBits>()
            .to_bitvec(),
        b"foo",
    );
    println!(
        "Collision found! \"{}\" hashes to 3D4B... with SHA-512",
        from_utf8(&collision).unwrap(),
    );

    let collision = find_collision(
        hex::decode("3D4B")
            .unwrap()
            .view_bits::<LocalBits>()
            .to_bitvec(),
        b"bar",
    );
    println!(
        "Collision found! \"{}\" hashes to 3D4B... with SHA-512",
        from_utf8(&collision).unwrap(),
    );

    let collision = find_collision(
        hex::decode("3D4B")
            .unwrap()
            .view_bits::<LocalBits>()
            .to_bitvec(),
        b"baz",
    );
    println!(
        "Collision found! \"{}\" hashes to 3D4B... with SHA-512",
        from_utf8(&collision).unwrap(),
    );
}
#[cfg(test)]
mod tests {
    use super::find_collision;
    use super::sha512n;
    use bitvec::prelude::*;
    use std::str::from_utf8;

    #[test]
    fn test_sha512n() {
        assert_eq!(sha512n(b"hello", 1), bitvec![1]);
        assert_eq!(sha512n(b"hello", 8), bitvec![1, 1, 0, 1, 1, 0, 0, 1]);

        // Collision under sha-512-8, see next test!
        assert_eq!(sha512n(b"world66", 8), bitvec![1, 1, 0, 1, 1, 0, 0, 1]);
    }

    #[test]
    fn test_collision() {
        // See previous test, "hello" and "world66" have the same first 8 bits with SHA-512
        assert_eq!(
            from_utf8(&find_collision(sha512n(b"hello", 8), b"world",)).unwrap(),
            "world66"
        );
    }
}
