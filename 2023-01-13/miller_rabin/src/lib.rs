use std::vec;

use crypto_bigint::{CheckedSub, Integer, Random, U8192};
use rand_core::OsRng;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MillerRabinError {
    #[error("This function only supports up to 1,024 bytes of input. Got {0}.")]
    TooManyBytes(usize),
    #[error("The Miller-Rabin test only allows testing numbers > 3. Found n <= 3")]
    LowInteger(),
    #[error("The Miller-Rabin test only allows testing odd number. n is even.")]
    EvenInteger(),
    #[error("Cannot find random number with 2 <= n <= limit")]
    CannotPickRandomBasis(),
    #[error("Multiplication modulo p requires both operands to be < p")]
    MulModOperandTooHigh(),
    #[error("Exponentiation modulo p requires operand to be < p")]
    PowMulOperandTooHigh(),
}

/// Implementation of Miller-Rabbin primality test based on the description
/// of the algorithm in Cryptography Engineering (Ferguson, Schneier, Kohno).
/// This function expects bytes in Big-Endian order.
/// Under the hood, we use crypto-bigint's U8192.
/// Hence this function errors if `bytes` has more than 1,024 u8 elements.
pub fn miller_rabin(bytes: Vec<u8>) -> Result<bool, MillerRabinError> {
    const REQUIRED_BYTE_LENGTH_FOR_U8192: usize = 1024;

    if bytes.len() > REQUIRED_BYTE_LENGTH_FOR_U8192 {
        return Err(MillerRabinError::TooManyBytes(bytes.len()));
    }

    // Compute padding (`bytes` may or may not be shorter than 1,024 bytes)
    let mut padding = vec![];
    if bytes.len() < REQUIRED_BYTE_LENGTH_FOR_U8192 {
        let pad_len = REQUIRED_BYTE_LENGTH_FOR_U8192 - bytes.len();
        padding = vec![0u8; pad_len];
    }

    // Container for our final bytes (exactly 1,024 bytes)
    let mut sized_bytes = vec![];
    sized_bytes.extend(padding);
    sized_bytes.extend(bytes);

    let n = U8192::from_be_slice(&sized_bytes);
    if n <= U8192::from(3u8) {
        return Err(MillerRabinError::LowInteger());
    }
    if bool::from(n.is_even()) {
        return Err(MillerRabinError::EvenInteger());
    }

    // Compute (s, t) such that s is odd and s.2^t = n-1
    let n_minus_one = n.checked_sub(&U8192::ONE).unwrap();
    let mut s = n_minus_one;
    let mut t = 0;
    while bool::from(s.is_even()) {
        s = s.checked_div(&U8192::from(2u8)).unwrap();
        t += 1;
    }

    let mut k = 0;
    let limit = n_minus_one;
    let bit_size = bit_len(&n);
    while k < 128 {
        let a = get_random(bit_size, limit)?;
        k += 1;

        // Compute a^s mod n
        let mut v = pow_mod(&a, &s, &n)?;
        if v == U8192::ONE {
            // If v is 1, we're good for this basis! Carry on...
            continue;
        } else {
            // The sequence v, v^2, v^4, v^(2^t) must end in (n-1), 1.
            let mut i = 0;
            while v != n_minus_one {
                if i == t - 1 {
                    return Ok(false);
                } else {
                    v = mul_mod(&v, &v, &n)?;
                    i += 1
                }
            }
        }
    }
    Ok(true)
}

// Computes n^k (mod p)
// Errors when n >= p
fn pow_mod(n: &U8192, k: &U8192, p: &U8192) -> Result<U8192, MillerRabinError> {
    if n >= p {
        return Err(MillerRabinError::PowMulOperandTooHigh());
    }
    let mut res = U8192::ONE;
    // Series: n, n^2, n^4, n^8...n^(2^k)
    let mut exp_n = *n;
    // Series: m, n/2, m/4, ...m/(2^k)
    let mut divided_k = *k;

    while divided_k != U8192::ZERO {
        // If last bit is one (odd number), we multiply by exp_n
        if bool::from(divided_k.is_odd()) {
            res = mul_mod(&res, &exp_n, p)?;
        }
        divided_k >>= 1;
        exp_n = mul_mod(&exp_n, &exp_n, p)?;
    }
    Ok(res)
}

// Computes n * m (mod p)
// Errors when n >= p
fn mul_mod(n: &U8192, m: &U8192, p: &U8192) -> Result<U8192, MillerRabinError> {
    if n >= p {
        return Err(MillerRabinError::MulModOperandTooHigh());
    }

    let mut res = U8192::ZERO;
    // Series: n, 2n, 4n, 8n...(2^k)n
    let mut doubled_n = *n;
    // Series: m, n/2, m/4, ...m/(2^k)
    let mut divided_m = *m;

    while divided_m != U8192::ZERO {
        // If last bit is one (odd number), we add "multiplied n"
        if bool::from(divided_m.is_odd()) {
            res = res.add_mod(&doubled_n, p);
        }
        divided_m >>= 1;
        doubled_n = doubled_n.add_mod(&doubled_n, p);
    }
    Ok(res)
}

/// Get a random number 2 <= n <= `limit`, of bit size `bit_size`.
/// We start by generating a fully random U8192, then shift its bits right to get the proper bit size.
/// We then check for boundary conditions.
/// If the boundary conditions aren't met, we loop and retry up to 100 times.
///
/// /!\ This function is easy to misuse /!\
/// if `bit_size` is set way higher than log2(limit), randomly picking an integer
/// of `bit_size` length is unlikely to be within (2, limit) -- it'll usually be way higher!
fn get_random(bit_size: usize, limit: U8192) -> Result<U8192, MillerRabinError> {
    const MAX_RANDOM_RETRIES: usize = 100;
    let mut retry_counter = 0;

    while retry_counter < MAX_RANDOM_RETRIES {
        let mut num = U8192::random(OsRng);
        num >>= 8192 - bit_size;
        if num > U8192::ONE && num <= limit {
            return Ok(num);
        }
        retry_counter += 1;
    }
    Err(MillerRabinError::CannotPickRandomBasis())
}

// Returns the number of non-zero bits of a U8192.
// To do this we iterate with the right-shift operator until we reach 0.
// There's probably a more performant way to do this, by e.g. iterating on limbs?
fn bit_len(n: &U8192) -> usize {
    let mut n = *n;
    let mut bit_len = 0;
    while bit_len < 8192 {
        if n == U8192::ZERO {
            return bit_len;
        } else {
            n >>= 1;
            bit_len += 1
        }
    }
    8192
}

#[cfg(test)]
mod test {
    use crypto_bigint::{Checked, U8192};

    use crate::{bit_len, get_random, miller_rabin, mul_mod, pow_mod};

    #[test]
    fn test_miller_rabin() {
        // Low numbers
        assert!(!miller_rabin(45u32.to_be_bytes().to_vec()).unwrap());
        assert!(miller_rabin(547u32.to_be_bytes().to_vec()).unwrap());
        assert!(miller_rabin(2357u32.to_be_bytes().to_vec()).unwrap());
        assert!(miller_rabin(7919u32.to_be_bytes().to_vec()).unwrap());
    }

    #[test]
    fn test_expensive_miller_rabin() {
        // Primes taken from https://safecurves.cr.yp.to/field.html
        let secp256k1_prime =
            hex::decode("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f")
                .unwrap();
        let ed25519_prime =
            hex::decode("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed")
                .unwrap();
        assert!(miller_rabin(secp256k1_prime).unwrap());
        assert!(miller_rabin(ed25519_prime).unwrap());
    }

    #[test]
    fn test_miller_rabin_simple_errors() {
        assert_eq!(
            miller_rabin(1u8.to_be_bytes().to_vec())
                .unwrap_err()
                .to_string(),
            "The Miller-Rabin test only allows testing numbers > 3. Found n <= 3".to_string(),
        );
        assert_eq!(
            miller_rabin(2u8.to_be_bytes().to_vec())
                .unwrap_err()
                .to_string(),
            "The Miller-Rabin test only allows testing numbers > 3. Found n <= 3".to_string(),
        );
        assert_eq!(
            miller_rabin(3u8.to_be_bytes().to_vec())
                .unwrap_err()
                .to_string(),
            "The Miller-Rabin test only allows testing numbers > 3. Found n <= 3".to_string(),
        );
        assert_eq!(
            miller_rabin(4u8.to_be_bytes().to_vec())
                .unwrap_err()
                .to_string(),
            "The Miller-Rabin test only allows testing odd number. n is even.".to_string(),
        );
        assert_eq!(
            miller_rabin(10u8.to_be_bytes().to_vec())
                .unwrap_err()
                .to_string(),
            "The Miller-Rabin test only allows testing odd number. n is even.".to_string(),
        );
        let big_ass_num = vec![1u8; 2000];
        assert_eq!(
            miller_rabin(big_ass_num).unwrap_err().to_string(),
            "This function only supports up to 1,024 bytes of input. Got 2000.".to_string(),
        );
    }

    #[test]
    fn test_checked_bigint_overflow() {
        let a = Checked::new(U8192::MAX);
        let b = Checked::new(U8192::ONE);
        let c = a + b;

        // c.0 is a CtOption, which exposes `is_none` & `is_some`.
        // These methods return a Choice that is a u8 under the covers
        // This `Choice` can then be converted to a bool.
        // Complex you say? That's the cost of abstractions to prevent side-channel attacks by default
        assert!(bool::from(c.0.is_none()))
    }

    #[test]
    fn test_get_random() {
        // We're asking for a random 2048 bit number and asking that it's < 100 after 100 tries
        // Yes, technically there is an extremely low probability that this test fails but...
        assert_eq!(
            get_random(2048, U8192::from(100u32))
                .unwrap_err()
                .to_string(),
            "Cannot find random number with 2 <= n <= limit".to_string()
        );

        // We're asking for a 10-bit number under 512. So we have 50% chance on each random try.
        // The probability of this test failing is 2^(-100)!
        assert!(get_random(10, U8192::from(512u32)).unwrap() <= U8192::from(512u32));
    }

    #[test]
    fn test_bit_len() {
        assert_eq!(bit_len(&U8192::ZERO), 0);
        assert_eq!(bit_len(&U8192::ONE), 1);
        assert_eq!(bit_len(&U8192::from(3u8)), 2);
        assert_eq!(bit_len(&U8192::from(10u8)), 4);
        assert_eq!(bit_len(&U8192::from(1023u32)), 10);
        assert_eq!(bit_len(&U8192::from(1025u32)), 11);
        assert_eq!(bit_len(&U8192::MAX), 8192);
    }

    #[test]
    fn test_mul_mod() {
        assert_eq!(
            mul_mod(&U8192::from(15u8), &U8192::from(5u8), &U8192::from(10u8))
                .unwrap_err()
                .to_string(),
            "Multiplication modulo p requires both operands to be < p".to_string()
        );
        // 5 * 15 = 5 (mod 10)
        assert_eq!(
            mul_mod(&U8192::from(5u8), &U8192::from(15u8), &U8192::from(10u8)).unwrap(),
            U8192::from(5u8)
        );
        // 6 * 7 = 2 (mod 10)
        assert_eq!(
            mul_mod(&U8192::from(6u8), &U8192::from(7u8), &U8192::from(10u8)).unwrap(),
            U8192::from(2u8)
        );
        // 5 * 2 = 0 (mod 10)
        assert_eq!(
            mul_mod(&U8192::from(5u8), &U8192::from(2u8), &U8192::from(10u8)).unwrap(),
            U8192::ZERO
        );
        // (n-1)*2 (mod n) = 2*n - 2 (mod n) = n - 2 (mod n)
        assert_eq!(
            mul_mod(
                &U8192::MAX.wrapping_sub(&U8192::ONE),
                &U8192::from(2u8),
                &U8192::MAX
            )
            .unwrap(),
            U8192::MAX.wrapping_sub(&U8192::from(2u8))
        );
    }

    #[test]
    fn test_pow_mod() {
        assert_eq!(
            pow_mod(&U8192::from(11u8), &U8192::from(10u8), &U8192::from(10u8))
                .unwrap_err()
                .to_string(),
            "Exponentiation modulo p requires operand to be < p".to_string()
        );

        // 2^10 = 1024 = 4 (mod 10)
        assert_eq!(
            pow_mod(&U8192::from(2u8), &U8192::from(10u8), &U8192::from(10u8)).unwrap(),
            U8192::from(4u8)
        );
    }
}
