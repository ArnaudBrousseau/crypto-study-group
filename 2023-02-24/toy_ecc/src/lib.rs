use std::fmt;
use std::ops;

use num_bigint::ToBigInt;
use num_bigint::ToBigUint;
use thiserror::Error;
use num_bigint::{BigUint};
use crate::element::Element;
use crate::util::buint;
mod element;
mod eea;
pub mod util;

#[derive(Error, Debug)]
pub enum ECCError {
    #[error("Cannot operate on two different curves")]
    CurveMismatch,
    #[error("Cannot compute EEA because one or more operand is zero")]
    EEAOperandIsZero,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurveForm {
    Weierstrass,
    Montgomery,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Curve<'a> {
    name: &'a str,
    p: BigUint,
    a2: Element,
    a4: Element,
    a6: Element,
}

impl fmt::Display for Curve<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Curve {}", self.name)?;
        match self.form() {
            CurveForm::Montgomery => {
                write!(f, " (Montgomery):")?
            }
            CurveForm::Weierstrass => {
                write!(f, " (Weierstrass):")?
            }
            CurveForm::Unknown => {
                write!(f, ":")?
            }
        }
        write!(f, " y^2 = x^3")?;
        if self.a2.value != buint(0) {
            write!(f, " + {}x^2", self.a2.value)?;
        }
        if self.a4.value != buint(0) {
            write!(f, " + {}x", self.a4.value)?;
        }
        if self.a6.value != buint(0) {
            write!(f, " + {}", self.a6.value)?;
        }
        write!(f, " (mod {})>", self.p)
    }
}

impl Curve<'_> {
    pub fn new<BUINT: ToBigUint, BINT: ToBigInt>(name: &str, a2: BINT, a4: BINT, a6: BINT, p: BUINT) -> Curve {
        let prime = p.to_biguint().unwrap();
        
        Curve {
            name,
            p: prime.clone(),
            a2: Element::new(a2, prime.clone()),
            a4: Element::new(a4, prime.clone()),
            a6: Element::new(a6, prime),
        }
    }

    fn form(&self) -> CurveForm {
        if self.a4 == self.zero() && self.a2 != self.zero() {
            CurveForm::Montgomery
        } else if self.a2 == self.zero() && self.a4 != self.zero() {
            CurveForm::Weierstrass
        } else {
            // Curve isn't a Montgomery or Weierstrass curve
            // ...but it's still a valid elliptic curve!
            CurveForm::Unknown
        }
    }

    fn discriminant(&self) -> BigUint {
        // See https://mathworld.wolfram.com/EllipticDiscriminant.html for this formula
        let b2 = self.element(4) * self.a2.clone();
        let b4 = self.element(2) * self.a4.clone();
        let b6 = self.element(4) * self.a6.clone();
        let b8 = b2.clone() * self.a6.clone() - self.a4.pow(2);
        
        let delta = - b8*b2.pow(2) - self.element(8)*b4.pow(3) - self.element(27)*b6.pow(2) + self.element(9)*b2*b4*b6;
        
        delta.value
    }

    fn is_smooth(&self) -> bool {
        self.discriminant() != buint(0)
    }

    fn contains(&self, point: Point) -> bool {
        let x = self.element(point.x);
        let y = self.element(point.y);

        y.pow(2) == (x.pow(3) + self.a2.clone()*x.pow(2) + self.a4.clone()*x + self.a6.clone())
    }

    fn element<T: ToBigUint>(&self, value: T) -> Element {
        Element {
            value: buint(value),
            modulus: self.p.clone(),
        }
    }

    fn zero(&self) -> Element {
        Element { value: buint(0), modulus: self.p.clone() }
    }

    fn point_at_infinity(&self) -> Point<'_> {
        Point {
            x: buint(0),
            y: buint(0),
            r#type: PointType::Infinite,
            curve: self.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointType {
    Regular,
    Infinite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point<'a> {
    x: BigUint,
    y: BigUint,
    r#type: PointType,
    curve: Curve<'a>,
}

impl fmt::Display for Point<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Point Object on curve {}: ", self.curve.name)?;
        match self.r#type {
            PointType::Regular => {
                write!(f, "Regular (x: {}, y: {})>", self.x, self.y)
            }
            PointType::Infinite => {
                write!(f, "Point at Infinity>")
            }
        }
    }
}

impl ops::Neg for Point<'_> {
    type Output = Self;
    fn neg(self) -> Self::Output {

        Self {
            x: self.x,
            y: (self.curve.p.clone() - self.y) % self.curve.p.clone(),
            r#type: self.r#type,
            curve: self.curve,
        }
    }
}

impl ops::Add for Point<'_> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if other.curve != self.curve {
            panic!("{}", ECCError::CurveMismatch)
        }

        // O + X = X
        if self.r#type == PointType::Infinite && other.r#type == PointType::Regular {
            return other
        }
        // X + O = X
        if self.r#type == PointType::Regular && other.r#type == PointType::Infinite {
            return self
        }
        // O + O = O
        if self.r#type == PointType::Infinite && other.r#type == PointType::Infinite {
            return Self {
                x: buint(0),
                y: buint(0),
                r#type: PointType::Infinite,
                curve: self.curve,
            }
        }

        // P + -P = O
        if self.x == other.x && (self.y.clone() + other.y.clone()) % self.curve.clone().p == buint(0) {
            return Self {
                x: buint(0),
                y: buint(0),
                r#type: PointType::Infinite,
                curve: self.curve,
            }
        }

        let lambda = if self == other {
            // Compute Lambda for doubling
            (self.curve.element(3)*self.curve.element(self.x.clone()).pow(2) + self.curve.a4.clone())
            / (self.curve.element(2) * self.curve.element(self.y.clone()))
        } else {
            // Standard formula, when the two points are distinct
            (self.curve.element(other.y.clone()) - self.curve.element(self.y.clone()))
            / (self.curve.element(other.x.clone()) - self.curve.element(self.x.clone()))
        };

        let result_x = lambda.pow(2) - self.curve.element(self.x.clone()) - self.curve.element(other.x);
        let result_y = lambda * (self.curve.element(self.x) - result_x.clone()) - self.curve.element(self.y);

           
        // X + Y (both regular points)
        Self {
            x: result_x.value,
            y: result_y.value,
            r#type: PointType::Regular,
            curve: self.curve,
        }
    }
}

impl Point<'_> {
    // Point multiplication, implemented with the double-and-add technique
    // (deviates from the OG toy_ecc implementation, which did this with non-adjacent representation)
    fn mul(&self, scalar: BigUint) -> Point<'_> {
        if scalar == buint(0) {
            return self.curve.point_at_infinity()
        }

        let mut result = self.curve.point_at_infinity();
        // Will be shifted right until 0
        let mut multiplier = scalar;
        // Will be set to p, 2p, 4p, 8p, etc...
        let mut doubled_point = self.clone();
        while multiplier.clone() != buint(0) {
            if multiplier.clone() % buint(2) == buint(1) {
                result = result + doubled_point.clone();
            }
            doubled_point = doubled_point.clone() + doubled_point.clone();
            multiplier >>= 1;
        }

        result
    }
}


#[cfg(test)]
mod test {

    use num_bigint::{BigUint};
    use crate::{Curve, Point, PointType};
    use crate::util::{buint, bint};
    
    fn curve_secp256k1() -> Curve<'static> {
        let p_secp256 = bint(2).pow(256) - bint(2).pow(32) - bint(977);
        Curve::new("secp256k1", 0, 0, 7, p_secp256)
    }

    // Curve from Introduction to Mathematical Cryptography
    // Chapter 5, section 2 and 3
    // Y^2 =X^3+3X+8 on F13
    fn curve_trivial() -> Curve<'static> {
        Curve::new("trivial curve", 0, 3, 8, 13)
    }

    fn curve_secp256k1_generator() -> Point<'static> {
        let x  = BigUint::parse_bytes(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
        let y  = BigUint::parse_bytes(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap();
        let curve = curve_secp256k1();
        Point {
            x, y,
            r#type: crate::PointType::Regular,
            curve: curve,
        }
    }

    #[test]
    fn test_curve_display() {
        let montgomery = Curve::new("MyCurve", 44, 0, 46, 89);
        let weierstrass = Curve::new("MyCurve", 0, 5, 42, 89);
        let unknown = Curve::new("MyCurve", 5, 7, 42, 89);
        let both =  Curve::new("MyCurve", 0, 0, 42, 89);

        assert_eq!(format!("{montgomery}"), "<Curve MyCurve (Montgomery): y^2 = x^3 + 44x^2 + 46 (mod 89)>");
        assert_eq!(format!("{weierstrass}"), "<Curve MyCurve (Weierstrass): y^2 = x^3 + 5x + 42 (mod 89)>");
        assert_eq!(format!("{unknown}"), "<Curve MyCurve: y^2 = x^3 + 5x^2 + 7x + 42 (mod 89)>");
        assert_eq!(format!("{both}"), "<Curve MyCurve: y^2 = x^3 + 42 (mod 89)>");
    }

    #[test]
    fn test_point_display() {
        let p = Point {
            x: buint(1),
            y: buint(2),
            r#type: crate::PointType::Regular,
            curve:  curve_secp256k1(),
        };

        let formatted = format!("{p}");
        assert_eq!(formatted, "<Point Object on curve secp256k1: Regular (x: 1, y: 2)>");
    }

    #[test]
    fn test_curve_discriminant() {
        let c = curve_trivial();
        assert_eq!(true, c.is_smooth());
        assert_eq!(buint(4), c.discriminant());

        // Example from https://mathworld.wolfram.com/EllipticDiscriminant.html
        let cusp = Curve::new("Cusp", 0, 0, 0, 13);
        assert_eq!(buint(0), cusp.discriminant());
        assert_eq!(false, cusp.is_smooth());

        let node = Curve::new("Node", 0, -3, 2, 13);
        assert_eq!(buint(0), node.discriminant());
        assert_eq!(false, node.is_smooth());

        let secp = curve_secp256k1();
        // See https://neuromancer.sk/std/secg/secp256k1
        let secp256k1_discriminant = BigUint::parse_bytes(b"115792089237316195423570985008687907853269984665640564039457584007908834650495", 10).unwrap();
        assert_eq!(true, secp.is_smooth());
        assert_eq!(secp256k1_discriminant, secp.discriminant());
    }

    #[test]
    fn test_negate_point() {
        let p = Point {
            x: buint(1),
            y: buint(2),
            r#type: crate::PointType::Regular,
            curve:  Curve::new("My curve", 7, 0, 1, 17),
        };

        let minus_p = -p;

        assert_eq!(minus_p.curve.name, "My curve");
        assert_eq!(minus_p.x, buint(1));
        assert_eq!(minus_p.y, buint(15));
    }

    #[test]
    fn test_contains() {
        let p = Point {
            x: buint(2),
            y: buint(3),
            r#type: crate::PointType::Regular,
            curve:  curve_trivial(),
        };

        let c = curve_trivial();
        assert_eq!(true, c.contains(p));

        let q = Point {
            x: buint(2),
            y: buint(8),
            r#type: crate::PointType::Regular,
            curve:  curve_trivial(),
        };
        let c = curve_trivial();
        assert_eq!(false, c.contains(q));
    }

    #[test]
    // Test cases taken from page 289 of ItMC book (addition table)
    fn test_point_addition_on_trivial_curve() {
        let p = Point {
            x: buint(2),
            y: buint(3),
            r#type: crate::PointType::Regular,
            curve:  curve_trivial(),
        };

        let q = Point {
            x: buint(9),
            y: buint(6),
            r#type: crate::PointType::Regular,
            curve:  curve_trivial(),
        };

        let sum = p.clone() + q;
        assert_eq!(sum.x, buint(12));
        assert_eq!(sum.y, buint(2));

        let double = p.clone() + p;
        assert_eq!(double.x, buint(12));
        assert_eq!(double.y, buint(11));
    }

    #[test]
    // Test case taken from page 289 of ItMC book (addition table)
    fn test_addition_yields_infinity() {
        let p = Point {
            x: buint(1),
            y: buint(8),
            r#type: crate::PointType::Regular,
            curve:  curve_trivial(),
        };

        let q = Point {
            x: buint(1),
            y: buint(5),
            r#type: crate::PointType::Regular,
            curve:  curve_trivial(),
        };

        let sum = p + q;
        assert_eq!(sum.r#type, PointType::Infinite);
    }
    
    #[test]
    fn test_point_addition_on_secp256k1() {
        let g = curve_secp256k1_generator();

        let sum = g.clone() + g;

        // These test values were obtained by referencing another secp256k1 / Bitcoin library: my own, written in JS a while back, specifically for Bitcoin.
        // See https://github.com/ArnaudBrousseau/arnaudbrousseau.com/blob/master/static/labs/keys.deconstructed/keys.deconstructed.js
        let expected_x = BigUint::parse_bytes(b"C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5", 16).unwrap();
        let expected_y = BigUint::parse_bytes(b"1AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A", 16).unwrap();

        // ----/!\ DIGRESSION /!\------------------------------------------------------------------------------------------------------------------------------
        // Another cool proof that these expected values are correct: let's derive the Bitcoin address for it!
        // These expected coordinates are for "2*G", which means our "secret" seed is simply "2".
        // To derive a new (uncompressed) Bitcoin address from (x, y) coordinates, we can compute:
        //   hash: ripemd160(sha256(04||X_COORD_IN_HEX||Y_COORD_IN_HEX))
        //   checksum: sha256(sha256(00||hash)) -- take the first 4 bytes
        //   address: base58encode(00||hash||checksum)
        //   (in the above, "||" denotes concatenation)
        //
        // ----
        //
        // On the CLI we can compute the first steps directly with `openssl`:
        //
        //   # Computes "hash"
        //   $ echo -n '04C6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE51AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A' \
        //     | xxd -r -p \
        //     | openssl dgst -sha256 -binary \
        //     | openssl dgst -ripemd160
        //   d6c8e828c1eca1bba065e1b83e1dc2a36e387a42
        //
        //   # Now, checksum
        //   $ echo -n '00d6c8e828c1eca1bba065e1b83e1dc2a36e387a42' | xxd -r -p | openssl dgst -sha256 -binary | openssl dgst -sha256
        //   74d4b4aafae3b18583e99da13ff0dae6276a888f5dbd3a05eea50806ebda06c1
        //
        // Finally, we go to http://lenschulwitz.com/base58 to base58 encode `00d6c8e828c1eca1bba065e1b83e1dc2a36e387a4274d4b4aa`
        // ...and we find that the result is `1LagHJk2FyCV2VzrNHVqg3gYG4TSYwDV4m`
        // ...low and behold, that's also the uncompressed address indicated on https://keys.lol/bitcoin/1 on the second row (seed 2!)
        // 
        // Hence proving the correctness of these expected values: we just showed that they yield the right BTC address!
        // --------------------------------------------------------------------------------------------------------------------------------------------------

        assert_eq!(sum.x, expected_x);
        assert_eq!(sum.y, expected_y);
        assert_eq!(sum.curve.name, "secp256k1");
    }

    #[test]
    fn test_addition_with_infinity_point() {
        let p = Point {
            x: buint(1),
            y: buint(2),
            r#type: crate::PointType::Regular,
            curve:  curve_secp256k1(),
        };

        let infinity = Point {
            x: buint(0),
            y: buint(0),
            r#type: crate::PointType::Infinite,
            curve:  curve_secp256k1(),
        };

        assert_eq!(p.clone() + infinity.clone(), p.clone());
        assert_eq!(infinity.clone() + p.clone(), p.clone());
        assert_eq!(infinity.clone() + infinity.clone(), infinity.clone());
        assert_eq!((p.clone() + -p.clone()).r#type, PointType::Infinite);
    }

    #[test]
    fn test_point_multiplication() {
        let p = Point {
            x: buint(1),
            y: buint(8),
            r#type: crate::PointType::Regular,
            curve:  curve_trivial(),
        };

        let noop_mul = p.mul(buint(1));
        assert_eq!(noop_mul.x, p.x);
        assert_eq!(noop_mul.y, p.y);

        let double = p.mul(buint(2));
        assert_eq!(double.x, buint(2));
        assert_eq!(double.y, buint(3));

        let triple = p.mul(buint(3));
        assert_eq!(triple.x, buint(9));
        assert_eq!(triple.y, buint(6));

        let quadruple = p.mul(buint(4));
        assert_eq!(quadruple.x, buint(12));
        assert_eq!(quadruple.y, buint(11));

        let quintuple = p.mul(buint(5));
        assert_eq!(quintuple.x, buint(12));
        assert_eq!(quintuple.y, buint(2));

        let times_nine = p.mul(buint(9));
        assert_eq!(times_nine.r#type, PointType::Infinite);
        assert_eq!(times_nine.x, buint(0));
        assert_eq!(times_nine.y, buint(0));

        // Any multiple of 9 should yield the point at infinity
        let times_thousand = p.mul(buint(9000));
        assert_eq!(times_thousand.r#type, PointType::Infinite);
        assert_eq!(times_thousand.x, buint(0));
        assert_eq!(times_thousand.y, buint(0));
    }
}
