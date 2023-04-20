use num_bigint::{BigInt, BigUint};

use crate::util::{bint, buint};
use crate::ECCError;

#[derive(Debug)]
pub struct EEAOutput {
    pub gcd: BigUint,
    pub u: BigInt,
    pub v: BigInt,
}

/// Extended Euclidean Algorithm
/// Returns GCD(a,b) and the pair (u, v) such that:
///     au + bv = GCD(a, b)
///     (Bezout identity)
pub fn eea(a: BigUint, b: BigUint) -> Result<EEAOutput, ECCError> {
    if a == buint(0) || b == buint(0) {
        return Err(ECCError::EEAOperandIsZero);
    }

    if a > b {
        Ok(compute_eea(b, a, bint(1), bint(0), bint(0), bint(1)))
    } else {
        Ok(compute_eea(a, b, bint(1), bint(0), bint(0), bint(1)))
    }
}

/// Recursive inner function
/// For each round, a < b, and we iterate on a, b, ua/va, and ub/vb
fn compute_eea(
    a: BigUint,
    b: BigUint,
    ua: BigInt,
    va: BigInt,
    ub: BigInt,
    vb: BigInt,
) -> EEAOutput {
    if a == buint(0) {
        return EEAOutput {
            gcd: b,
            u: ub,
            v: vb,
        };
    }
    let quotient = bint(b.clone() / a.clone());
    let remainder = bint(b) - quotient.clone() * bint(a.clone());

    compute_eea(
        buint(remainder),
        a,
        ub - quotient.clone() * ua.clone(),
        vb - quotient * va.clone(),
        ua,
        va,
    )
}
