use std::fmt::Display;
use std::ops;

use num_bigint::{BigUint, ToBigUint, ToBigInt};

use crate::eea;
use crate::util::{buint, bint};

// Type to wrap BigUint and do math in a finite field
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub value: BigUint,
    pub modulus: BigUint,
}

impl Element {
    /// Creates a new Element from a value and a modulus.
    /// If the value passed in is negative, we set the Element to be (modulus - value)
    /// If the value passed in is more than the modulus, we reduce it to be in the range [0, modulus[
    pub fn new<BINT: ToBigInt, BUINT: ToBigUint>(value: BINT, modulus: BUINT) -> Self {
        let m = buint(modulus);
        let v = bint(value);
        
        let val = if v < bint(0) {
            buint(bint(m.clone()) + v)
        } else if v >= bint(m.clone()) {
            buint(v.modpow(&bint(1), &bint(m.clone())))
        } else {
            buint(v)
        };

        Self{
            value: val,
            modulus: m,
        }
    }

    pub fn pow(&self, exponent: usize) -> Self {
        Self {
            value: self.value.modpow(&buint(exponent), &self.modulus),
            modulus: self.modulus.clone(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.value == buint(0)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (mod {})", self.value, self.modulus)
    }
}

impl ops::Neg for Element {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            value: self.modulus.clone() - self.value,
            modulus: self.modulus,
        }
    }
}

impl ops::Add for Element {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.modulus != other.modulus {
            panic!("Cannot add elements with different modulus");
        }
        
        Self {
            value: (self.value + other.value) % self.modulus.clone(),
            modulus: self.modulus,
        }
    }
}

impl ops::Sub for Element {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.modulus != other.modulus {
            panic!("Cannot subtract elements with different modulus");
        }
        let value = if self.value >= other.value {
            self.value - other.value
        } else {
            self.modulus.clone() + self.value - other.value
        };

        Self {
            value: value % self.modulus.clone(),
            modulus: self.modulus,
        }
    }
}

impl ops::Mul for Element {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.modulus != other.modulus {
            panic!("Cannot multiply elements with different modulus");
        }
        
        Self {
            value: (self.value * other.value) % self.modulus.clone(),
            modulus: self.modulus,
        }
    }
}

impl ops::Div for Element {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let res = eea::eea(other.modulus.clone(), other.value.clone());
        if res.is_err() {
            panic!("error while computing EEA: {res:?}");
        }
        
        let eea_out = res.unwrap();
        if eea_out.gcd != buint(1) {
            panic!("cannot inverse because GCD({}, {}) != 1", &other, &other.modulus)
        }

        // EEA gives: other*u + modulus*v = 1.
        // So `u` is an inverse of `other` mod `modulus`.
        // However, `u` could be negative (in this case, return u + modulus)
        let other_inv = if eea_out.u < bint(0) {
            buint(eea_out.u + bint(self.modulus.clone()))
        } else {
            buint(eea_out.u) % self.modulus.clone()
        };
        
        Self {
            value: (self.value * other_inv) % self.modulus.clone(),
            modulus: self.modulus,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{element::Element};
    
    #[test]
    fn test_display_element() {
        let a = Element::new(5, 13);
        assert_eq!(format!("{a}"), "5 (mod 13)");
    }

    #[test]
    fn test_add_element() {
        let a = Element::new(5, 13);
        let b = Element::new(10, 13);
        assert_eq!(a + b, Element::new(2, 13));
    }

    #[test]
    fn test_is_zero() {
        assert_eq!(Element::new(0, 13).is_zero(), true);
        assert_eq!((Element::new(4, 5) + Element::new(1, 5)).is_zero(), true);
    }

    #[test]
    fn test_subtract_element() {
        let a = Element::new(5, 13);
        let b = Element::new(3, 13);
        assert_eq!(a.clone() - b.clone(), Element::new(2, 13));
        assert_eq!(b.clone() - a.clone(), Element::new(11, 13));
        assert_eq!(a.clone() - a.clone(), Element::new(0, 13));
    }

    #[test]
    fn test_negate_element() {
        let a = Element::new(5, 13);
        assert_eq!(-a, Element::new(8, 13));
    }

    #[test]
    fn test_multiply_elements() {
        let a = Element::new(5, 13);
        let b = Element::new(6, 13);
        assert_eq!(a*b, Element::new(4, 13));
    }

    #[test]
    fn test_pow_element() {
        let a = Element::new(5, 13);
        assert_eq!(a.pow(1), Element::new(5, 13));
        assert_eq!(a.pow(2), Element::new(12, 13));
        assert_eq!(a.pow(3), Element::new(8, 13));
    }

    #[test]
    fn test_divide_elements() {
        let a = Element::new(5, 13);
        let b = Element::new(9, 13);
        assert_eq!(a/b, Element::new(2, 13));
    }
}