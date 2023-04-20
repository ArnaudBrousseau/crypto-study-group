use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};

// I'm tired of writing x.to_biguint().unwrap()...
// .............. ...and x.to_bigint().unwrap()...

pub fn buint<T: ToBigUint>(val: T) -> BigUint {
    val.to_biguint().unwrap()
}

pub fn bint<T: ToBigInt>(val: T) -> BigInt {
    val.to_bigint().unwrap()
}
